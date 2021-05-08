#![no_std]
#![no_main]
#![allow(dead_code)]
#![feature(asm, start)]

mod asm;
mod stm32f103_consts;
use stm32f103_consts::*;

/// defines panic handler (no need to call it explicitly in any way)
mod panic_halt;

// from Embedonomicon
//    https://docs.rust-embedded.org/embedonomicon/memory-layout.html
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn Reset() -> ! {
    main()
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;
// End of embedonomicon snippet

// specific to this program: the number of the pin we want to toggle
// PC13 is the LED on the bluepill board
const GPIO_NUM: u8 = 13;

unsafe fn enable_gpioc_clock() {
    *RCC_APB2ENR |= RCC_APB2ENR_IOPCEN;
}

unsafe fn gpio_setup() {
    // (GPIO_NUM - 8) because it's CRH (high) and it starts from pin 8 (first 8 pins are in CRL)
    //
    // *4 because each combination of MODE and CNF takes 4 bits (2 for mode and 2 for CNF)
    // That's also why we add 2 to the CNF
    *GPIOC_CRH = (GPIO_CNF_OUTPUT_PUSHPULL as u32) << (((GPIO_NUM - 8) * 4) + 2)
        | (GPIO_MODE_OUTPUT_2_MHZ as u32) << ((GPIO_NUM - 8) * 4);
}

fn wait(cycles: u32) {
    for _ in 0..cycles {
        asm::nop();
    }
}

fn main() -> ! {
    unsafe {
        enable_gpioc_clock();
        gpio_setup();

        loop {
            // sets GPIO high which turns LED off
            // because it's connected between VCC and the pin
            *(GPIOC_BSRR as *mut usize) = 1 << GPIO_NUM as u32;
            wait(500_000);

            // sets GPIO low which turns LED on
            *(GPIOC_BSRR as *mut usize) = (1 << GPIO_NUM as u32) << 16;
            wait(100_000);
        }
    }
}
