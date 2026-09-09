# Development journey

This log records completed, reproducible examples. Every milestone states the
exact validation performed so a successful build is not confused with a
physical-device result.

## 1. Minimal CH582 runtime and GPIO

`examples/ch582-blinky` establishes the smallest complete image: QingKe reset
runtime, CH582 clock setup, owned GPIO tokens, and a PA8 output loop.

Validation: release ELF build and linker completion for
`riscv32imc-unknown-none-elf`. No board observation is claimed here.
