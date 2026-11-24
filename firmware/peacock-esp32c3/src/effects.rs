use embassy_time::{Duration, Instant, Ticker};

use crate::{
    board::{CHANNEL_COUNT, PWM_MAX},
    state::{Mode, SHARED, WorkMode},
};

pub const FRAME_PERIOD: Duration = Duration::from_millis(10);
const RADIAL: [u16; CHANNEL_COUNT] = [5, 4, 3, 2, 1, 0, 0, 1, 2, 3, 4, 5];
const DECAY: [u16; 31] = [
    4095, 3621, 3202, 2831, 2503, 2213, 1957, 1730, 1530, 1353, 1196, 1058, 935, 827, 731, 646,
    572, 505, 447, 395, 349, 309, 273, 241, 213, 189, 167, 147, 130, 115, 101,
];

#[embassy_executor::task]
pub async fn render() {
    loop {
        if SHARED.status().work != WorkMode::Show {
            SHARED.wake_render.wait().await;
            continue;
        }
        let mut ticker = Ticker::every(FRAME_PERIOD);
        let start = Instant::now();
        let mut last_beat = SHARED.audio.lock(|value| value.get().beat_counter);
        let mut last_burst = SHARED.controls().burst;
        let mut beat_at = None;
        let mut burst_at = None;
        let mut last_mode = SHARED.controls().mode;
        let mut mode_start = start;
        while SHARED.status().work == WorkMode::Show {
            let now = Instant::now();
            let controls = SHARED.controls();
            let audio = SHARED.audio.lock(|value| value.get());
            let fresh = audio
                .updated_at
                .is_some_and(|t| now - t < Duration::from_millis(100));
            if fresh && audio.beat_counter != last_beat {
                beat_at = Some(now);
            }
            last_beat = audio.beat_counter;
            if controls.burst != last_burst {
                burst_at = Some(now);
                last_burst = controls.burst;
            }
            if last_mode != controls.mode {
                last_mode = controls.mode;
                mode_start = now;
            }

            let elapsed = (now - mode_start).as_millis();
            let envelope = if fresh { audio.envelope } else { 0 };
            let burst = burst_at
                .map(|t| (now - t).as_millis())
                .filter(|&ms| ms < 650);
            let beat = beat_at.map(|t| (now - t).as_millis());
            let levels = core::array::from_fn(|index| {
                let distance = RADIAL[index];
                let level = if let Some(ms) = burst {
                    if ms % 200 < 45 { PWM_MAX } else { 0 }
                } else {
                    match controls.mode {
                        Mode::PeacockWave => wave(elapsed, distance),
                        Mode::BeatFanBurst => fan_burst(beat, distance),
                        Mode::AudioVuFan => vu(envelope, distance),
                        Mode::CallStrobe => {
                            if elapsed % 330 < 70 {
                                PWM_MAX
                            } else {
                                0
                            }
                        }
                        Mode::Custom => controls.custom[index],
                    }
                };
                (u32::from(level) * u32::from(controls.master) / 100) as u16
            });
            SHARED.frame.lock(|frame| frame.set(Some(levels)));
            ticker.next().await;
        }
    }
}

fn wave(ms: u64, distance: u16) -> u16 {
    let phase = (ms + 2_400 - u64::from(distance) * 140) % 2_400;
    let rising = phase.min(2_400 - phase) as u32 * 1024 / 1_200;
    260 + (u32::from(PWM_MAX - 260) * rising * rising / (1024 * 1024)) as u16
}

fn fan_burst(beat: Option<u64>, distance: u16) -> u16 {
    let pulse = beat
        .and_then(|ms| ms.checked_sub(u64::from(distance) * 20))
        .and_then(|age| DECAY.get((age / 10) as usize).copied())
        .unwrap_or(0);
    pulse.max(180)
}

fn vu(envelope: u16, distance: u16) -> u16 {
    (i32::from(envelope) * 6 - i32::from(distance) * i32::from(PWM_MAX))
        .clamp(0, i32::from(PWM_MAX)) as u16
}
