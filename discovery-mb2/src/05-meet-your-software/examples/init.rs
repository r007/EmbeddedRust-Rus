#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use microbit as _;
use panic_halt as _;

#[entry]
fn main() -> ! {
    #[allow(clippy::needless_late_init)]
    let _y;
    let x = 42;
    _y = x;

    // бесконечный цикл, чтобы не покидать этот стековый фрейм
    loop {
        asm::nop();
    }
}
