#![no_main]
#![no_std]

use core::sync::atomic::{
    AtomicUsize,
    Ordering::{AcqRel, Acquire},
};

use cortex_m::asm;
use cortex_m_rt::entry;
use critical_section_lock_mut::LockMut;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    hal::{
        self, gpiote,
        pac::{self, interrupt},
    },
    Board,
};

static COUNTER: AtomicUsize = AtomicUsize::new(0);
static GPIOTE_PERIPHERAL: LockMut<gpiote::Gpiote> = LockMut::new();
static DEBOUNCE_TIMER: LockMut<hal::Timer<pac::TIMER0>> = LockMut::new();

// 100 мс при частоте счёта 1 МГц.
const DEBOUNCE_TIME: u32 = 100 * 1_000_000 / 1000;

#[interrupt]
fn GPIOTE() {
    DEBOUNCE_TIMER.with_lock(|debounce_timer| {
        if debounce_timer.read() == 0 {
            let _ = COUNTER.fetch_add(1, AcqRel);
            debounce_timer.start(DEBOUNCE_TIME);
        }
    });
    GPIOTE_PERIPHERAL.with_lock(|gpiote| {
        gpiote.channel0().reset_events();
    });
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
    GPIOTE_PERIPHERAL.init(gpiote);

    // Настраиваем таймер подавления дребезга.
    let mut debounce_timer = hal::Timer::new(board.TIMER0);
    debounce_timer.disable_interrupt();
    debounce_timer.reset_event();
    DEBOUNCE_TIMER.init(debounce_timer);

    // Настраиваем NVIC для обработки прерываний.
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::Interrupt::GPIOTE);

    // Поскольку мы не отключаем прерывания GPIOTE даже во время отсчёта таймера подавления дребезга,
    // дополнительные прерывания от кнопки могут приходить и внутри интервала подавления.
    // Разумно было бы отключать прерывания кнопки, когда запущен таймер подавления дребезга,
    // и включать их снова по его истечении, но тогда понадобился бы обработчик прерывания таймера.
    // Чтобы простое «исправление» не ухудшало читаемость,
    //  вводим переменную-«охранник» cur_count.
    let mut cur_count = 0;
    loop {
        // «wait for interrupt»: процессор засыпает до прихода прерывания.
        asm::wfi();
        let count = COUNTER.load(Acquire);
        if count > cur_count {
            rprintln!("ouch {}", count);
            cur_count = count;
        }
    }
}
