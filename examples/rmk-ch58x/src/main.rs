#![no_std]
#![no_main]

use embassy_ch58x::gpio::{Drive, Input, Level, Output, Pins, Pull};
use rmk::config::{BehaviorConfig, DeviceConfig, PositionalConfig};
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::keyboard::Keyboard;
use rmk::matrix::Matrix;
use rmk::types::action::{Action, KeyAction};
use rmk::types::keycode::{HidKeyCode, KeyCode};
use rmk::usb::UsbTransport;
use rmk::{KeymapData, initialize_keymap, run_all};

embassy_ch58x::bind_interrupts!(struct Irqs {
    USB => embassy_ch58x::usb::InterruptHandler;
});

const ROWS: usize = 1;
const COLS: usize = 1;
const LAYERS: usize = 1;
const DEFAULT_KEYMAP: [[[KeyAction; COLS]; ROWS]; LAYERS] =
    [[[KeyAction::Single(Action::Key(KeyCode::Hid(HidKeyCode::A)))]]];

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

    // CH582M EVT board only: connect one switch between PA9 and PA8.
    let rows = [Input::new(pins.pa8, Pull::Down)];
    let columns = [Output::new(pins.pa9, Level::Low, Drive::MilliAmps5)];
    let mut matrix =
        Matrix::<_, _, _, ROWS, COLS, true>::new(rows, columns, DefaultDebouncer::new());

    let mut keymap_data = KeymapData::new(DEFAULT_KEYMAP);
    let mut behavior_config = BehaviorConfig::default();
    let positional_config = PositionalConfig::default();
    let keymap =
        initialize_keymap(&mut keymap_data, &mut behavior_config, &positional_config).await;
    let mut keyboard = Keyboard::new(&keymap);

    let driver = embassy_ch58x::usb::Driver::new(peripherals.USB, Irqs);
    let device_config = DeviceConfig {
        vid: 0xc0de,
        pid: 0xcafe,
        manufacturer: "rusted-ch5",
        product_name: "RMK CH582M EVT USB",
        serial_number: "EXAMPLE",
    };
    let mut usb_transport = UsbTransport::new(driver, device_config);

    run_all!(matrix, keyboard, usb_transport).await.0
}
