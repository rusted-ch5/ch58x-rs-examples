#![no_std]
#![no_main]

use embassy_ch58x::hal::gpio::{Drive, Level, Output, Pins};
use embassy_time::Timer;
use embedded_hal::digital::OutputPin;

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
    let mut led = Output::new(pins.pa8, Level::Low, Drive::MilliAmps5);

    loop {
        led.set_high().unwrap();
        Timer::after_millis(250).await;
        led.set_low().unwrap();
        Timer::after_millis(250).await;
    }
}
