use core::cell::Cell;

use bt_hci::controller::ExternalController;
use embassy_futures::select::{Either, select};
use embassy_time::{Duration, Ticker, Timer};
use esp_hal::{efuse::Efuse, peripherals::BT};
use esp_println::println;
use esp_wifi::{EspWifiController, ble::controller::BleConnector};
use heapless::Vec;
use trouble_host::{
    att::{AttClient, AttReq},
    prelude::*,
};

use crate::{
    board::{CHANNEL_COUNT, PWM_MAX},
    state::{Controls, Mode, SHARED, Status, WorkMode},
};

const NAME: &str = "Peacock-Stick";
const SERVICE: u128 = 0xb3e10710_6107_420d_a4d4_d7e6bca2ae00;
const CONTROL_UUID: u128 = 0xb3e10710_6107_420d_a4d4_d7e6bca2ae01;
const TELEMETRY_UUID: u128 = 0xb3e10710_6107_420d_a4d4_d7e6bca2ae02;
const TELEMETRY_LEN: usize = 54;
const CONTROL_LEN: usize = 1 + CHANNEL_COUNT * 12 / 8;
const PAGE_LEN: usize = 18;

#[gatt_server(connections_max = 1)]
struct PeacockServer {
    peacock: PeacockService,
}

#[gatt_service(uuid = SERVICE)]
struct PeacockService {
    #[characteristic(uuid = CONTROL_UUID, write, write_without_response)]
    control: Vec<u8, CONTROL_LEN>,
    #[characteristic(uuid = TELEMETRY_UUID, read, notify)]
    telemetry: Vec<u8, TELEMETRY_LEN>,
}

#[embassy_executor::task]
pub async fn run(controller: EspWifiController<'static>, mut bt: BT<'static>) {
    let mut mac = Efuse::read_base_mac_address();
    mac.reverse();
    mac[5] |= 0xc0; // Static random address derived from this module's eFuse MAC.
    let server = PeacockServer::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: NAME,
        appearance: &appearance::UNKNOWN,
    }))
    .expect("fixed Peacock GATT table");
    loop {
        let connector =
            ExternalController::<_, 4>::new(BleConnector::new(&controller, bt.reborrow()));
        let mut resources: HostResources<DefaultPacketPool, 1, 2> = HostResources::new();
        let stack =
            trouble_host::new(connector, &mut resources).set_random_address(Address::random(mac));
        let Host {
            mut peripheral,
            mut runner,
            ..
        } = stack.build();
        match select(runner.run(), advertise(&mut peripheral, &server)).await {
            Either::First(result) => println!("BLE runner ended: {:?}", result),
            Either::Second(result) => println!("BLE peripheral restarting: {:?}", result),
        }
        Timer::after_millis(500).await;
    }
}

async fn advertise<C: Controller>(
    peripheral: &mut Peripheral<'_, C, DefaultPacketPool>,
    server: &PeacockServer<'_>,
) -> Result<(), BleHostError<C::Error>> {
    let mut scan = [0; 31];
    let scan_len = AdStructure::encode_slice(
        &[AdStructure::CompleteLocalName(NAME.as_bytes())],
        &mut scan,
    )
    .expect("device name fits");
    let seq = Cell::new(0);
    loop {
        SHARED.wake_advertiser.reset();
        let status = SHARED.status();
        let mut adv = [0; 31];
        let mut service_data = [0; 22];
        let uuids = [SERVICE.to_le_bytes()];
        let payload = if status.charge_wake {
            service_data[..16].copy_from_slice(&uuids[0]);
            service_data[16] = 1;
            service_data[17] = flags(status);
            service_data[18] = validity(status);
            service_data[19] = status.charge.map_or(0xff, |c| c.status);
            service_data[20..22]
                .copy_from_slice(&status.charge.map_or(0, |c| c.vbat_mv).to_le_bytes());
            AdStructure::Unknown {
                ty: 0x21,
                data: &service_data,
            }
        } else {
            AdStructure::ServiceUuids128(&uuids)
        };
        let adv_len = AdStructure::encode_slice(
            &[
                AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
                payload,
            ],
            &mut adv,
        )
        .expect("legacy advertisement fits");
        let charge_only = status.work == WorkMode::ChargeOnly;
        let interval = Duration::from_millis(if charge_only { 800 } else { 160 });
        let params = AdvertisementParameters {
            interval_min: interval,
            interval_max: interval,
            ..Default::default()
        };
        let advertiser = peripheral
            .advertise(
                &params,
                Advertisement::ConnectableScannableUndirected {
                    adv_data: &adv[..adv_len],
                    scan_data: &scan[..scan_len],
                },
            )
            .await?;
        // Refresh voltage while advertising and update the interval on mode changes.
        let refresh = async {
            if status.charge_wake {
                select(SHARED.wake_advertiser.wait(), Timer::after_secs(4)).await;
            } else {
                SHARED.wake_advertiser.wait().await;
            }
        };
        if let Either::First(result) = select(advertiser.accept(), refresh).await {
            let conn = result?.with_attribute_server(server)?;
            select(
                gatt_events(&conn, server, &seq),
                notifications(&conn, server, &seq),
            )
            .await;
        }
    }
}

async fn gatt_events(
    conn: &GattConnection<'_, '_, DefaultPacketPool>,
    server: &PeacockServer<'_>,
    seq: &Cell<u8>,
) {
    let mut last_read = [0; TELEMETRY_LEN];
    loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { .. } => return,
            GattConnectionEvent::Gatt {
                event: GattEvent::Write(event),
            } if event.handle() == server.peacock.control.handle => {
                let result = match control(event.data()) {
                    Ok(command) => {
                        let reply = event.accept();
                        if reply.is_ok() {
                            SHARED.edit(|controls| command.apply(controls));
                        }
                        reply
                    }
                    Err(code) => {
                        if matches!(event.payload().incoming(), AttClient::Command(_)) {
                            drop(event.into_payload());
                            continue;
                        }
                        event.reject(code)
                    }
                };
                if let Ok(reply) = result {
                    reply.send().await;
                }
            }
            GattConnectionEvent::Gatt {
                event: GattEvent::Read(event),
            } if event.handle() == server.peacock.telemetry.handle => {
                // Keep READ_BLOB fragments from the same snapshot at the default MTU.
                if !matches!(
                    event.payload().incoming(),
                    AttClient::Request(AttReq::ReadBlob { offset: 1.., .. })
                ) || last_read[0] == 0
                {
                    last_read = telemetry(seq.get());
                }
                let _ = server.set(&server.peacock.telemetry, &as_value(&last_read));
                if let Ok(reply) = event.accept() {
                    reply.send().await;
                }
            }
            _ => {}
        }
    }
}

async fn notifications(
    conn: &GattConnection<'_, '_, DefaultPacketPool>,
    server: &PeacockServer<'_>,
    seq: &Cell<u8>,
) {
    let mut ticker = Ticker::every(Duration::from_millis(500));
    loop {
        ticker.next().await;
        seq.set(seq.get().wrapping_add(1));
        let snapshot = telemetry(seq.get());
        let characteristic = &server.peacock.telemetry;
        if usize::from(conn.raw().att_mtu()) >= TELEMETRY_LEN + 3 {
            let _ = characteristic.notify(conn, &as_value(&snapshot)).await;
        } else {
            for (index, chunk) in snapshot.as_chunks::<PAGE_LEN>().0.iter().enumerate() {
                let mut part = [0; PAGE_LEN + 2];
                part[0] = 0x80 | index as u8;
                part[1] = seq.get();
                part[2..].copy_from_slice(chunk);
                if characteristic.notify(conn, &as_value(&part)).await.is_err() {
                    break;
                }
            }
        }
    }
}

fn as_value(bytes: &[u8]) -> Vec<u8, TELEMETRY_LEN> {
    Vec::from_slice(bytes).expect("bounded telemetry payload")
}

enum Command {
    Mode(Mode),
    Master(u8),
    Sensitivity(u8),
    Channels([u16; CHANNEL_COUNT]),
}

impl Command {
    fn apply(self, controls: &mut Controls) {
        match self {
            Self::Mode(value) => controls.mode = value,
            Self::Master(value) => controls.master = value,
            Self::Sensitivity(value) => controls.sensitivity = value,
            Self::Channels(value) => controls.custom = value,
        }
    }
}

fn control(bytes: &[u8]) -> Result<Command, AttErrorCode> {
    match bytes {
        [0x01, value] => Mode::from_byte(*value)
            .map(Command::Mode)
            .ok_or(AttErrorCode::VALUE_NOT_ALLOWED),
        [0x02, value @ 0..=100] => Ok(Command::Master(*value)),
        [0x03, value @ 0..=2] => Ok(Command::Sensitivity(*value)),
        [0x02 | 0x03, _] => Err(AttErrorCode::VALUE_NOT_ALLOWED),
        [0x04, packed @ ..] if bytes.len() == CONTROL_LEN => {
            let mut channels = [0; CHANNEL_COUNT];
            let pairs = channels.as_chunks_mut::<2>().0;
            let triples = packed.as_chunks::<3>().0;
            for (pair, triple) in pairs.iter_mut().zip(triples) {
                pair[0] = u16::from(triple[0]) | (u16::from(triple[1] & 0x0f) << 8);
                pair[1] = u16::from(triple[1] >> 4) | (u16::from(triple[2]) << 4);
            }
            Ok(Command::Channels(channels))
        }
        [0x01..=0x04, ..] | [] => Err(AttErrorCode::INVALID_ATTRIBUTE_VALUE_LENGTH),
        _ => Err(AttErrorCode::VALUE_NOT_ALLOWED),
    }
}

fn flags(status: Status) -> u8 {
    u8::from(status.charge_wake)
        | (u8::from(status.work == WorkMode::Show) << 1)
        | (u8::from(status.mode_button == Some(true)) << 2)
        | (u8::from(status.flash_pressed) << 3)
        | (u8::from(status.lights_enabled) << 4)
        | (u8::from(status.charge.is_some_and(|c| c.fast)) << 5)
        | ((status.work as u8) << 6)
}

fn validity(status: Status) -> u8 {
    u8::from(status.charge.is_some())
        | (u8::from(status.power.total.is_some_and(|t| !t.overflow)) << 1)
        | (u8::from(status.power.banks[0].is_some()) << 2)
        | (u8::from(status.power.banks[1].is_some()) << 3)
        | (u8::from(status.mode_button.is_some()) << 4)
        | (u8::from(status.power.total.is_some_and(|t| t.estimated)) << 5)
        | (u8::from(status.power.bus_mv.is_some()) << 6)
        | (u8::from(status.power.total.is_some_and(|t| t.overflow)) << 7)
}

fn telemetry(seq: u8) -> [u8; TELEMETRY_LEN] {
    let status = SHARED.status();
    let controls = SHARED.controls();
    let mut frame = [0; TELEMETRY_LEN];
    frame[..8].copy_from_slice(&[
        1,
        seq,
        controls.mode as u8,
        controls.master,
        controls.sensitivity,
        flags(status),
        validity(status),
        status.charge.map_or(0xff, |c| c.status),
    ]);
    let pack_mv = status.power.bus_mv.unwrap_or(0);
    let c = status.charge.unwrap_or_default();
    let mut word = |offset: usize, value: u16| {
        frame[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    };
    word(8, status.charge.map_or(pack_mv, |c| c.vbat_mv));
    word(10, c.vbus_mv);
    word(12, c.ibus_ma as u16);
    word(14, c.ibat_ma as u16);
    word(16, c.temp_half_c as u16);
    word(18, pack_mv);
    word(20, c.input_mw());
    word(22, status.power.total.map_or(0, |t| t.ma) as u16);
    for (i, ma) in status.power.channels().into_iter().enumerate() {
        word(28 + 2 * i, ma);
    }
    let mw = status.power.total.map_or(0, |t| t.mw);
    frame[24..28].copy_from_slice(&mw.to_le_bytes());
    frame[52] = (u32::from(status.cap) * 100 / u32::from(PWM_MAX)) as u8;
    frame
}
