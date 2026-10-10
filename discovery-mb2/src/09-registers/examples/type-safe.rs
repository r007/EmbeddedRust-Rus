#![no_main]
#![no_std]

#[allow(unused_imports)]
use registers::entry;

#[entry]
fn main() -> ! {
    let (p0, _p1) = registers::init();

    // Включаем верхний ряд
    p0.out.modify(|_, w| w.pin21().set_bit());

    // Включаем нижний ряд
    p0.out.modify(|_, w| w.pin19().set_bit());

    // Выключаем верхний ряд
    p0.out.modify(|_, w| w.pin21().clear_bit());

    // Выключаем нижний ряд
    p0.out.modify(|_, w| w.pin19().clear_bit());

    loop {
        core::hint::spin_loop();
    }
}
