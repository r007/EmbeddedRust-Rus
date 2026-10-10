#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use registers::entry;

#[entry]
fn main() -> ! {
    registers::init();

    unsafe {
        // «Магический» адрес!
        const PORT_P0_OUT: u32 = 0x50000504;

        // Включаем верхний ряд
        let out = ptr::read_volatile(PORT_P0_OUT as *mut u32);
        ptr::write_volatile(PORT_P0_OUT as *mut u32, out | 1 << 21);

        // Включаем нижний ряд
        let out = ptr::read_volatile(PORT_P0_OUT as *mut u32);
        ptr::write_volatile(PORT_P0_OUT as *mut u32, out | 1 << 19);

        // Выключаем верхний ряд
        let out = ptr::read_volatile(PORT_P0_OUT as *mut u32);
        ptr::write_volatile(PORT_P0_OUT as *mut u32, out & !(1 << 21));

        // Выключаем нижний ряд
        let out = ptr::read_volatile(PORT_P0_OUT as *mut u32);
        ptr::write_volatile(PORT_P0_OUT as *mut u32, out & !(1 << 19));
    }

    loop {
        core::hint::spin_loop();
    }
}
