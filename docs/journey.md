# Development journey

This log records completed, reproducible examples. Every milestone states the
exact validation performed so a successful build is not confused with a
physical-device result.

## 1. Minimal CH582 runtime and GPIO

`examples/ch582-blinky` establishes the smallest complete image: QingKe reset
runtime, CH582 clock setup, owned GPIO tokens, and a PA8 output loop.

Validation: release ELF build and linker completion for
`riscv32imc-unknown-none-elf`. No board observation is claimed here.

## 2. Embassy executor and time driver

`examples/ch582-embassy-blinky` replaces the polled delay with
`embassy_time::Timer`. When no task is ready, the CH58x executor sleeps on the
PFIC event latch; GPIO writes remain short synchronous register operations.

Validation: release ELF build and linker completion for
`riscv32imc-unknown-none-elf`. No board observation is claimed here.

## 3. Interrupt-driven UART

`examples/ch582-uart-echo` binds UART1 to an Embassy handler and echoes input
from PA8 back through PA9. Receive, transmit backpressure, and final flush all
yield to IRQ or timer futures; the async task contains no FIFO polling loop.

Validation: release ELF build and linker completion for
`riscv32imc-unknown-none-elf`. No UART traffic observation is claimed here.

## 4. Event-driven USB HID

`examples/ch582-usb-hid` builds a minimal boot-keyboard interface on USBFS. The
device state machine and endpoint transfers wait on interrupts; the example
sends an all-released report once per second using `embassy_time::Timer`, so it
does not generate key presses.

Validation: release ELF build and linker completion for
`riscv32imc-unknown-none-elf`. Enumeration and HID traffic on a physical board
are not claimed here.
