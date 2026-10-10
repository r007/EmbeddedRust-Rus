#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::asm;
use cortex_m_rt::entry;
use critical_section::Mutex;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    hal::{
        gpiote,
        pac::{self, interrupt},
    },
    Board,
};

static COUNTER: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));

/// Эта «функция» будет вызвана при получении прерывания. Пока что просто
/// выводим сообщение и паникуем.
#[interrupt]
fn GPIOTE() {
    critical_section::with(|cs| {
        let mut count = COUNTER.borrow(cs).borrow_mut();
        *count += 1;
        rprintln!("count: {}", count);
    });
    panic!();
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let button_a = board.buttons.button_a.into_floating_input();

    // Настраиваем GPIOTE так, чтобы он генерировал прерывание при нажатии Button A (GPIO
    // линия переходит в низкий уровень).
    let gpiote = gpiote::Gpiote::new(board.GPIOTE);
    let channel = gpiote.channel0();
    channel
        .input_pin(&button_a.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel.reset_events();

    // Настраиваем NVIC для обработки прерываний GPIO.
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::Interrupt::GPIOTE);

    loop {
        // «wait for interrupt»: процессор засыпает до прихода прерывания.
        asm::wfi();
    }
}
