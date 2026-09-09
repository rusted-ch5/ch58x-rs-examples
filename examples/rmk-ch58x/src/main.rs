#![no_std]
#![no_main]

#[cfg(all(feature = "ch582", feature = "ch585"))]
compile_error!("rmk-ch58x chip features ch582 and ch585 are mutually exclusive");
#[cfg(not(any(feature = "ch582", feature = "ch585")))]
compile_error!("rmk-ch58x requires exactly one chip feature: ch582 or ch585");

use embassy_ch58x::gpio::{Drive, Input, Level, Output, Pins, Pull};
#[cfg(feature = "vial")]
use rmk::config::RMK_BUILD_INFO;
#[cfg(feature = "storage")]
use rmk::config::StorageConfig;
use rmk::config::{BehaviorConfig, DeviceConfig, PositionalConfig};
#[cfg(feature = "vial")]
use rmk::config::{RmkConfig, VialConfig};
use rmk::debounce::default_debouncer::DefaultDebouncer;
#[cfg(feature = "vial")]
use rmk::host::HostService;
#[cfg(not(feature = "storage"))]
use rmk::initialize_keymap;
#[cfg(feature = "storage")]
use rmk::initialize_keymap_and_storage;
use rmk::keyboard::Keyboard;
use rmk::matrix::Matrix;
use rmk::types::action::{Action, KeyAction};
use rmk::types::keycode::{HidKeyCode, KeyCode};
use rmk::usb::UsbTransport;
use rmk::{KeymapData, run_all};

#[cfg(feature = "vial")]
mod vial {
    include!(concat!(env!("OUT_DIR"), "/vial_generated.rs"));
}

embassy_ch58x::bind_interrupts!(struct Irqs {
    USB => embassy_ch58x::usb::InterruptHandler;
});

const ROWS: usize = 1;
const COLS: usize = 1;
const LAYERS: usize = 1;
#[cfg(feature = "ch582")]
const PRODUCT_NAME: &str = "RMK CH582M EVT USB";
#[cfg(feature = "ch585")]
const PRODUCT_NAME: &str = "RMK CH585M EVT USB";
#[cfg(not(feature = "vial"))]
const SERIAL_NUMBER: &str = "EXAMPLE";
#[cfg(feature = "vial")]
const SERIAL_NUMBER: &str = RMK_BUILD_INFO;
const DEFAULT_KEYMAP: [[[KeyAction; COLS]; ROWS]; LAYERS] =
    [[[KeyAction::Single(Action::Key(KeyCode::Hid(HidKeyCode::A)))]]];

fn device_config() -> DeviceConfig<'static> {
    DeviceConfig {
        vid: 0xc0de,
        pid: 0xcafe,
        manufacturer: "rusted-ch5",
        product_name: PRODUCT_NAME,
        serial_number: SERIAL_NUMBER,
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[embassy_executor::main(
    entry = "qingke_rt::entry",
    executor = "embassy_ch58x::executor::Executor"
)]
async fn main(_spawner: embassy_executor::Spawner) -> ! {
    let peripherals = embassy_ch58x::init(embassy_ch58x::hal::sysctl::Config::default());
    let pins = Pins::new(peripherals.GPIOA, peripherals.GPIOB);

    // CH582M/CH585M EVT boards only: connect one switch between PA9 and PA8.
    let rows = [Input::new(pins.pa8, Pull::Down)];
    let columns = [Output::new(pins.pa9, Level::Low, Drive::MilliAmps5)];
    let mut matrix =
        Matrix::<_, _, _, ROWS, COLS, true>::new(rows, columns, DefaultDebouncer::new());

    let mut keymap_data = KeymapData::new(DEFAULT_KEYMAP);
    let mut behavior_config = BehaviorConfig::default();
    let positional_config = PositionalConfig::default();

    #[cfg(feature = "storage")]
    {
        let storage_config = StorageConfig {
            num_sectors: 32,
            ..Default::default()
        };
        let (keymap, mut storage) = initialize_keymap_and_storage(
            &mut keymap_data,
            embassy_ch58x::hal::dataflash::DataFlash::new(peripherals.DATAFLASH),
            &storage_config,
            &mut behavior_config,
            &positional_config,
        )
        .await;
        let mut keyboard = Keyboard::new(&keymap);
        let driver = embassy_ch58x::usb::Driver::new(peripherals.USB, Irqs);

        #[cfg(feature = "vial")]
        {
            let rmk_config = RmkConfig {
                device_config: device_config(),
                vial_config: VialConfig::new(&vial::VIAL_KEYBOARD_ID, vial::VIAL_KEYBOARD_DEF, &[]),
                storage_config,
            };
            let host_service = HostService::new(&keymap, &rmk_config);
            let mut usb_transport = UsbTransport::new(driver, rmk_config.device_config)
                .with_host_service(&host_service);
            run_all!(matrix, keyboard, storage, usb_transport).await.0
        }

        #[cfg(not(feature = "vial"))]
        {
            let mut usb_transport = UsbTransport::new(driver, device_config());
            run_all!(matrix, keyboard, storage, usb_transport).await.0
        }
    }

    #[cfg(not(feature = "storage"))]
    {
        let keymap =
            initialize_keymap(&mut keymap_data, &mut behavior_config, &positional_config).await;
        let mut keyboard = Keyboard::new(&keymap);

        let driver = embassy_ch58x::usb::Driver::new(peripherals.USB, Irqs);
        let mut usb_transport = UsbTransport::new(driver, device_config());

        run_all!(matrix, keyboard, usb_transport).await.0
    }
}
