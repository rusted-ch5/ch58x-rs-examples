# ch58x-rs-examples

Small, independently buildable examples for CH58x microcontrollers using the
`rusted-ch5` Rust crates.

Each example is introduced as a tested milestone in [the development
journey](docs/journey.md). Build success is a source/toolchain result; board
behavior is recorded separately when it has been observed on hardware.

## Toolchain

- Rust 1.87 or newer;
- target `riscv32imc-unknown-none-elf`.

## Examples

- `examples/ch582-blinky`: minimal runtime, clock, owned GPIO, and polled
  delay on PA8.
- `examples/ch582-embassy-blinky`: event-sleep executor and asynchronous
  SysTick delays driving PA8.

Build an example by package name, for example:

```sh
cargo build --release -p ch582-embassy-blinky
```

The minimal HAL example uses the PAC-provided vector table, while Embassy owns
its vector table. Build them in separate Cargo invocations so those mutually
exclusive linker features are not unified by `--workspace`.

## License

Licensed under either Apache-2.0 or MIT at your option.
