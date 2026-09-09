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

## License

Licensed under either Apache-2.0 or MIT at your option.
