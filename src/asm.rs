#[inline(always)]
pub fn nop() {
    unsafe {
        asm!("nop");
    }
}
