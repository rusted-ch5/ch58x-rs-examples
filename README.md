# ch58x-rs-examples

Small, independently buildable examples for CH58x microcontrollers using the
`rusted-ch5` Rust crates.

Each example is introduced as a tested milestone in [the development
journey](docs/journey.md). Build success is a source/toolchain result; board
behavior is recorded separately when it has been observed on hardware.

## Board scope

The current pin assignments target EVT boards only. The first four examples
target CH582M EVT; `rmk-ch58x` can select either CH585M EVT or CH582M EVT with a
chip feature. Hosted Actions compile and link the firmware; they do not claim
behavior on physical hardware.

## Toolchain

- Rust 1.92 or newer;
- target `riscv32imc-unknown-none-elf`.

## Examples — EVT boards only

- `examples/ch582-blinky`: minimal runtime, clock, owned GPIO, and polled
  delay on PA8.
- `examples/ch582-embassy-blinky`: event-sleep executor and asynchronous
  SysTick delays driving PA8.
- `examples/ch582-uart-echo`: interrupt-driven UART1 echo on PA9 TX and PA8 RX.
- `examples/ch582-usb-hid`: minimal event-driven USBFS boot-keyboard device.
- `examples/rmk-ch58x`: RMK 0.9 USB keyboard for CH585M EVT (default) or CH582M
  EVT, with a one-key interrupt-driven matrix on PA9/PA8. Its default profile
  enables persistent keymap storage in the final 8 KiB DataFlash window and a
  one-key Vial definition over USB.

Build an example by package name, for example:

```sh
cargo build --release -p ch582-embassy-blinky
```

Build the RMK USB example for an explicit chip:

```sh
cargo build --release -p rmk-ch58x --no-default-features --features ch585,storage,vial
cargo build --release -p rmk-ch58x --no-default-features --features ch582,storage,vial
```

An ordinary `cargo build --release -p rmk-ch58x` selects CH585M together with
storage and Vial. The explicit CH582M command above enables the same default
capabilities while changing only the chip selection.

The USB examples use conspicuous development-only VID/PID placeholders.
Replace them with identifiers assigned to your product before distributing
firmware.

The minimal HAL example uses the PAC-provided vector table, while Embassy owns
its vector table. Build them in separate Cargo invocations so those mutually
exclusive linker features are not unified by `--workspace`.

## License

Licensed under either Apache-2.0 or MIT at your option.
