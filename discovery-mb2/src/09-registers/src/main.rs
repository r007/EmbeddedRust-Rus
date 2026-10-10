#![no_main]
#![no_std]

#[allow(unused_imports)]
use registers::entry;

#[entry]
fn main() -> ! {
    registers::init();

    unsafe {
        // Волшебный адрес!
        const PORT_P0_OUT: u32 = 0x50000504;

        // Включаем верхний ряд
        *(PORT_P0_OUT as *mut u32) |= 1 << 21;

        // Включаем нижний ряд
        *(PORT_P0_OUT as *mut u32) |= 1 << 19;

        // Выключаем верхний ряд
        *(PORT_P0_OUT as *mut u32) &= !(1 << 21);

        // Выключаем нижний ряд
        *(PORT_P0_OUT as *mut u32) &= !(1 << 19);
    }

    loop {
        core::hint::spin_loop();
    }
}
