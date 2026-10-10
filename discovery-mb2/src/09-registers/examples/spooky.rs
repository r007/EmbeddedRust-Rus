#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use registers::{entry, rprintln};

// Выводим текущее содержимое P0.OUT
fn print_out() {
    const P0_OUT: u32 = 0x5000_0504;

    let out = unsafe { ptr::read_volatile(P0_OUT as *const u32) };

    rprintln!("P0.OUT = {:#08x}", out);
}

#[entry]
fn main() -> ! {
    registers::init();

    unsafe {
        // Куча «магических» адресов!
        const P0_OUTSET: u32 = 0x5000_0508;
        const P0_OUTCLR: u32 = 0x5000_050C;

        // Выводим начальное содержимое OUT
        print_out();

        // Включаем верхний ряд светодиодов
        ptr::write_volatile(P0_OUTSET as *mut u32, 1 << 21);
        print_out();

        // Включаем нижний ряд светодиодов
        ptr::write_volatile(P0_OUTSET as *mut u32, 1 << 19);
        print_out();

        // Выключаем верхний ряд светодиодов
        ptr::write_volatile(P0_OUTCLR as *mut u32, 1 << 21);
        print_out();

        // Выключаем нижний ряд светодиодов
        ptr::write_volatile(P0_OUTCLR as *mut u32, 1 << 19);
        print_out();
    }

    loop {
        core::hint::spin_loop();
    }
}
