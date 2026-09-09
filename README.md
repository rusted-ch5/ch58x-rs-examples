# ch58x-rs-examples

Small, independently buildable examples for CH58x microcontrollers using the
`rusted-ch5` Rust crates.

Each example is introduced as a tested milestone in [the development
journey](docs/journey.md). Build success is a source/toolchain result; board
behavior is recorded separately when it has been observed on hardware.

## Board scope

The current pin assignments and board-level examples target the CH582M EVT
board only. Board-specific cases use an `*-evt-*` directory name and carry an
`EVT board only` heading. Hosted Actions compile and link the firmware; they do
not claim behavior on physical hardware.

## Toolchain

- Rust 1.92 or newer;
- target `riscv32imc-unknown-none-elf`.

## Examples — CH582M EVT board only

- `examples/ch582-blinky`: minimal runtime, clock, owned GPIO, and polled
  delay on PA8.
- `examples/ch582-embassy-blinky`: event-sleep executor and asynchronous
  SysTick delays driving PA8.
- `examples/ch582-uart-echo`: interrupt-driven UART1 echo on PA9 TX and PA8 RX.
- `examples/ch582-usb-hid`: minimal event-driven USBFS boot-keyboard device.
- `examples/rmk-ch58x`: RMK 0.9 USB keyboard with a one-key interrupt-driven
  matrix on PA9/PA8. The RMK dependency disables its default features and
  enables only `async_matrix`; USB is RMK's core transport path.

Build an example by package name, for example:

```sh
cargo build --release -p ch582-embassy-blinky
```

The USB examples use conspicuous development-only VID/PID placeholders.
Replace them with identifiers assigned to your product before distributing
firmware.

The minimal HAL example uses the PAC-provided vector table, while Embassy owns
its vector table. Build them in separate Cargo invocations so those mutually
exclusive linker features are not unified by `--workspace`.

## License

Licensed under either Apache-2.0 or MIT at your option.
