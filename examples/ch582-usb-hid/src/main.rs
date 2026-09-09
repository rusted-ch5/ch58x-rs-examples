#![no_std]
#![no_main]

use embassy_futures::join::join;
use embassy_time::Timer;
use embassy_usb::class::hid::{
    Config as HidConfig, HidBootProtocol, HidSubclass, HidWriter, State,
};

embassy_ch58x::bind_interrupts!(struct Irqs {
    USB => embassy_ch58x::usb::InterruptHandler;
});

const KEYBOARD_REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x01, 0x09, 0x06, 0xa1, 0x01, 0x05, 0x07, 0x19, 0xe0, 0x29, 0xe7, 0x15, 0x00, 0x25, 0x01,
    0x75, 0x01, 0x95, 0x08, 0x81, 0x02, 0x95, 0x01, 0x75, 0x08, 0x81, 0x01, 0x95, 0x06, 0x75, 0x08,
    0x15, 0x00, 0x25, 0x65, 0x05, 0x07, 0x19, 0x00, 0x29, 0x65, 0x81, 0x00, 0xc0,
];

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
    let driver = embassy_ch58x::usb::Driver::new(peripherals.USB, Irqs);

    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("rusted-ch5");
    config.product = Some("CH582 USB HID example");
    config.serial_number = Some("EXAMPLE");
    config.device_class = 0;
    config.device_sub_class = 0;
    config.device_protocol = 0;
    config.composite_with_iads = false;

    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 32];
    let mut msos_descriptor = [0; 0];
    let mut control_buffer = [0; 64];
    let mut hid_state = State::new();
    let mut builder = embassy_usb::Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buffer,
    );
    let mut keyboard = HidWriter::<_, 8>::new(
        &mut builder,
        &mut hid_state,
        HidConfig {
            report_descriptor: KEYBOARD_REPORT_DESCRIPTOR,
            request_handler: None,
            poll_ms: 1,
            max_packet_size: 8,
            hid_subclass: HidSubclass::Boot,
            hid_boot_protocol: HidBootProtocol::Keyboard,
        },
    );
    let mut device = builder.build();

    let reports = async {
        keyboard.ready().await;
        loop {
            // Keep the endpoint path exercised without generating key presses.
            let _ = keyboard.write(&[0; 8]).await;
            Timer::after_secs(1).await;
        }
    };
    join(device.run(), reports).await.0
}
