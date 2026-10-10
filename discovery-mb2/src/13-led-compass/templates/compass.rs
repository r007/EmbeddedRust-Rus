#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

// Эти штуки вам пригодятся ;-).
use core::f32::consts::PI;
use libm::{atan2f, floorf};

use microbit::{
    display::blocking::Display,
    hal::{Timer, twim},
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{Lsm303agr, MagMode, MagOutputDataRate};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut timer0 = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_mag_mode_and_odr(
        &mut timer0,
        MagMode::HighResolution,
        MagOutputDataRate::Hz10,
    ).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    let mut leds = [[0u8; 5]; 5];

    // Индексы 16 светодиодов, которые используются на дисплее, и их
    // направления компаса.
    #[rustfmt::skip]
    let indices = [
        (2, 0) /* W */, (3, 0) /* W-SW */, (3, 1) /* SW */, (4, 1) /* S-SW */,
        (4, 2) /* S */, (4, 3) /* S-SE */, (3, 3) /* SE */, (3, 4) /* E-SE */,
        (2, 4) /* E */, (1, 4) /* E-NE */, (1, 3) /* NE */, (0, 3) /* N-NE */,
        (0, 2) /* N */, (0, 1) /* N-NW */, (1, 1) /* NW */, (1, 0) /* W-NW */,
    ];

    loop {
        // Измеряем магнитное поле.
        let (x, y) = todo!();

        // Получаем угол от -180° до 180° относительно оси x.
        let theta = atan2f(y as f32, x as f32);

        // Определяем, какой индекс светодиода мигнуть.
        let index = todo!();

        // Мигаем выбранным светодиодом.
        let (r, c) = indices[index];
        leds[r][c] = 255u8;
        display.show(&mut timer0, leds, 50);
        leds[r][c] = 0u8;
        display.show(&mut timer0, leds, 50);
    }
}
