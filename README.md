# Blinky-from-scratch

Blink built-in LED on STM32F103 "bluepill" board in Rust with no dependencies.
No "cortex-m-rt" or other libraries.

My goal was to figure out how to build a program for a microcontroller in Rust
while understanding every part of it (therefore no dependencies).

I've written some comments that try to explain everything that we're doing.

If you're learning how to build programs for microcontrollers then take a look
at these resources I used while building this:

 - [STM32F1 reference manual](https://www.st.com/resource/en/reference_manual/cd00171190-stm32f101xx-stm32f102xx-stm32f103xx-stm32f105xx-and-stm32f107xx-advanced-arm-based-32-bit-mcus-stmicroelectronics.pdf)
   Explains how to program STM32F1. That's where you would look up all the memory addresses and values and stuff.
 - [Rust Embedonomicon](https://docs.rust-embedded.org/embedonomicon/) Very nice walk-through on how to build programs for microcontrollers in Rust from scratch.
 - [libopencm3](https://github.com/libopencm3/libopencm3) A library for various microcontrollers. Relatively easy to read C code.
 - [libopencm3-examples](https://github.com/libopencm3/libopencm3-examples)
 - [cortex-m-rt Rust Crate](https://github.com/rust-embedded/cortex-m-rt) "Startup code and minimal runtime for Cortex-M microcontrollers". Seems too high-level for my taste, but useful nonetheless.
