#![no_std]
#![no_main]

use embassy_ch58x::hal::gpio::Pins;
use embassy_ch58x::uart::{Config, Uart};

embassy_ch58x::bind_interrupts!(struct Irqs {
    UART1 => embassy_ch58x::uart::InterruptHandler<embassy_ch58x::pac::UART1>;
});

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
    let mut uart = Uart::new(
        peripherals.UART1,
        pins.pa9,
        pins.pa8,
        Irqs,
        Config::default(),
    )
    .unwrap();
    let mut buffer = [0u8; 32];

    loop {
        let received = uart.read(&mut buffer).await.unwrap();
        let mut written = 0;
        while written < received {
            written += uart.write(&buffer[written..received]).await.unwrap();
        }
        uart.flush().await.unwrap();
    }
}
