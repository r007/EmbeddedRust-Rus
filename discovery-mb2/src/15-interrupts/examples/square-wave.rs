#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use microbit::{
    hal::{gpio, timer},
    Board,
};

/// «Период» — это время одного цикла. Он равен
/// 1/f, где f — частота в Гц. В этом случае
/// время измеряем в миллисекундах.
const PERIOD: u32 = 1000 / 220;

/// Количество циклов для пяти секунд звучания.
const CYCLES: u32 = 5000 / PERIOD;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut speaker_pin = board.speaker_pin.into_push_pull_output(gpio::Level::Low);
    let mut timer = timer::Timer::new(board.TIMER0);

    for _ in 0..CYCLES {
        speaker_pin.set_high().unwrap();
        timer.delay_ms(PERIOD / 2);
        speaker_pin.set_low().unwrap();
        timer.delay_ms(PERIOD / 2);
    }

    loop {
        asm::wfi();
    }
}
