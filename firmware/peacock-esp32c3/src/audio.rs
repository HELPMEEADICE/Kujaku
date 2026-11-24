use embassy_time::{Duration, Instant, Ticker, Timer};
use esp_hal::{
    Blocking, dma_circular_buffers_chunk_size,
    i2s::master::{DataFormat, I2s, I2sRx, Standard},
    peripherals::{DMA_CH0, I2S0},
    time::Rate,
};
use esp_println::println;

use crate::{
    board::{MicBclk, MicData, MicWs, PWM_MAX},
    state::{Audio, SHARED, WorkMode},
};

const SAMPLE_HZ: u32 = 16_000;
const SAMPLES: usize = 160;
const STEREO_BYTES: usize = 8;
const FRAME_BYTES: usize = SAMPLES * STEREO_BYTES;
const RING_BYTES: usize = 3 * FRAME_BYTES;
const Q12_MAX: u32 = PWM_MAX as u32;

#[embassy_executor::task]
pub async fn capture(
    i2s: I2S0<'static>,
    dma: DMA_CH0<'static>,
    bclk: MicBclk,
    ws: MicWs,
    data: MicData,
) {
    // The pinned HAL's DMA allocation macro spells out integer ceil division.
    #[allow(clippy::manual_div_ceil)]
    let (mut ring, descriptors, _, _) =
        dma_circular_buffers_chunk_size!(RING_BYTES, 0, FRAME_BYTES);
    let mut pending = [0; RING_BYTES];
    let mut rx = I2s::new(
        i2s,
        Standard::Philips,
        DataFormat::Data32Channel32,
        Rate::from_hz(SAMPLE_HZ),
        dma,
    )
    .i2s_rx
    .with_bclk(bclk)
    .with_ws(ws)
    .with_din(data)
    .build(descriptors);
    let mut beat_counter = 0u16;

    loop {
        if SHARED.status().work != WorkMode::Show {
            SHARED.wake_audio.wait().await;
            continue;
        }
        if !capture_show(&mut rx, &mut ring, &mut pending, &mut beat_counter).await {
            println!("I2S RX stalled; retrying");
            Timer::after_millis(250).await;
        }
        SHARED.audio.lock(|slot| {
            slot.set(Audio {
                beat_counter,
                ..Audio::SILENT
            })
        });
    }
}

/// Returns false if DMA needs to be restarted. Dropping the transfer stops RX.
async fn capture_show(
    rx: &mut I2sRx<'_, Blocking>,
    ring: &mut &mut [u8; RING_BYTES],
    pending: &mut [u8; RING_BYTES],
    beat_counter: &mut u16,
) -> bool {
    let Ok(mut transfer) = rx.read_dma_circular(ring) else {
        return false;
    };
    let mut detector = Detector::default();
    let mut latest = Instant::now();
    let mut poll = Ticker::every(Duration::from_millis(5));
    while SHARED.status().work == WorkMode::Show {
        let Ok(available) = transfer.available() else {
            return false;
        };
        if available >= FRAME_BYTES {
            let Ok(count) = transfer.pop(pending) else {
                return false;
            };
            let sensitivity = SHARED.controls().sensitivity;
            for frame in pending[..count].as_chunks::<FRAME_BYTES>().0 {
                let (envelope, beat_triggered) = detector.analyze(frame, sensitivity);
                if beat_triggered {
                    *beat_counter = beat_counter.wrapping_add(1);
                }
                latest = Instant::now();
                SHARED.audio.lock(|slot| {
                    slot.set(Audio {
                        envelope,
                        beat_counter: *beat_counter,
                        updated_at: Some(latest),
                    })
                });
            }
        }
        if latest.elapsed() > Duration::from_millis(100) {
            return false;
        }
        poll.next().await;
    }
    true
}

#[derive(Default)]
struct Detector {
    dc: i32,
    bass: i32,
    floor_q8: i32,
    previous_bass: u32,
    flux_floor_q8: i32,
    envelope: u32,
    frames: u16,
    refractory: u8,
}

impl Detector {
    fn analyze(&mut self, frame: &[u8; FRAME_BYTES], sensitivity: u8) -> (u16, bool) {
        let mut sum = 0u64;
        let mut bass_sum = 0u64;
        for lr in frame.as_chunks::<STEREO_BYTES>().0 {
            // Philips I2S, left slot only. Keep the upper 16 bits of signed PCM.
            let sample = i32::from(i16::from_le_bytes([lr[2], lr[3]]));
            self.dc += ((sample << 8) - self.dc) >> 9;
            let ac = sample - (self.dc >> 8);
            self.bass += ((ac << 8) - self.bass) >> 4;
            let bass = self.bass >> 8;
            sum += (i64::from(ac) * i64::from(ac)) as u64;
            bass_sum += (i64::from(bass) * i64::from(bass)) as u64;
        }
        let rms = ((sum / SAMPLES as u64) as u32).isqrt();
        let bass_rms = ((bass_sum / SAMPLES as u64) as u32).isqrt();
        let flux = bass_rms.saturating_sub(self.previous_bass);
        self.previous_bass = bass_rms;
        self.frames = self.frames.saturating_add(1);
        self.refractory = self.refractory.saturating_sub(1);

        if self.frames <= 25 {
            track(&mut self.floor_q8, rms, 4);
            track(&mut self.flux_floor_q8, flux, 4);
            return (0, false);
        }

        let (gain, flux_ratio, absolute_gate) =
            [(2, 5, 36), (3, 4, 24), (5, 3, 16)][sensitivity as usize];
        let floor = (self.floor_q8 >> 8).max(8) as u32;
        // Use a proportional noise margin to reject normal frame-to-frame
        // variation once the room is already loud.
        let over_noise = rms.saturating_sub(floor + absolute_gate.max(floor / 12));
        let target = (over_noise * Q12_MAX * gain / (floor * 3 + 512)).min(Q12_MAX);
        self.envelope = if target > self.envelope {
            self.envelope + (target - self.envelope).div_ceil(2)
        } else {
            self.envelope - (self.envelope - target).div_ceil(8)
        };
        let flux_floor = (self.flux_floor_q8 >> 8).max(2) as u32;
        let beat_triggered = self.refractory == 0
            && over_noise > 0
            && flux > absolute_gate.max(flux_floor * flux_ratio).max(floor / 14);
        if beat_triggered {
            self.refractory = 17; // Minimum separation of 170 ms.
        }

        // Both trackers continue adapting during loud passages. A transient
        // only slows adaptation, so a sustained loud room eventually settles.
        let rate = if beat_triggered {
            128
        } else if rms > floor {
            40
        } else {
            80
        };
        track(&mut self.floor_q8, rms, rate);
        track(&mut self.flux_floor_q8, flux, 32);
        (self.envelope as u16, beat_triggered)
    }
}

fn track(value: &mut i32, sample: u32, divisor: i32) {
    *value += (((sample as i32) << 8) - *value) / divisor;
}
