#![no_std]
#![no_main]

use ch58x_hal::gpio::{Drive, Level, Output, Pins};
use embedded_hal::digital::OutputPin;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[qingke_rt::entry]
fn main() -> ! {
    let peripherals = ch58x_hal::init(ch58x_hal::sysctl::Config::default());
    let pins = Pins::new(peripherals.GPIOA, peripherals.GPIOB);
    let mut led = Output::new(pins.pa8, Level::Low, Drive::MilliAmps5);

    loop {
        led.set_high().unwrap();
        qingke::riscv::asm::delay(3_000_000);
        led.set_low().unwrap();
        qingke::riscv::asm::delay(3_000_000);
    }
}
