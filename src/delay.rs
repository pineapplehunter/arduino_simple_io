/// A small busy loop.
pub fn delay_ms(ms: u16) {
    for _ in 0..ms {
        for _ in 0..1600 {
            unsafe { asm!("" :::: "volatile") }
        }
    }
}
