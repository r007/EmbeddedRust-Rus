#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use critical_section_lock_mut::LockMut;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    hal::{
        gpio,
        pac::{self, interrupt},
        timer,
    },
    Board,
};

/// Базовая частота сирены в Гц.
const BASE_FREQ: u32 = 440;
/// Максимальный подъём частоты сирены в Гц.
const FREQ_RISE: u32 = 220;
/// Время одного полного цикла в мкс.
const RISE_TIME: u32 = 500_000;

/// Эти удобные типы упрощают жизнь.
type SpeakerPin = gpio::Pin<gpio::Output<gpio::PushPull>>;
type SirenTimer = timer::Timer<pac::TIMER0>;

/// Текущее состояние сирены. Обновляется обработчиком прерывания
/// во время работы.
struct Siren {
    /// Таймер, который использует сирена.
    timer: SirenTimer,
    /// Пин динамика MB2. Должен принадлежать
    /// этой структуре, чтобы его мог использовать обработчик прерывания.
    speaker_pin: SpeakerPin,
    /// Сейчас пин динамика в высоком или низком уровне?
    pin_high: bool,
    /// Время в мкс с начала текущего цикла сирены.
    cur_time: u32,
}

impl Siren {
    /// Создать новую сирену с заданной периферией.
    fn new(speaker_pin: SpeakerPin, timer: SirenTimer) -> Self {
        Self {
            timer,
            speaker_pin,
            pin_high: false,
            cur_time: 0,
        }
    }

    /// Запустить сирену.
    fn start(&mut self) {
        self.speaker_pin.set_low().unwrap();
        self.pin_high = false;
        self.cur_time = 0;
        self.timer.enable_interrupt();
        // Интервал таймера задаётся в тиках.
        // Таймер [nrf52833_hal] жёстко настроен на 1 млн тиков в секунду.
        self.timer.start(1_000_000 / BASE_FREQ);
    }

    /// Остановить сирену.
    fn stop(&mut self) {
        self.timer.disable_interrupt();
    }

    /// Перевести сирену к следующему изменению состояния динамика.
    /// Обычно вызывается из обработчика прерывания таймера.
    fn step(&mut self) {
        // Переключаем пин динамика.
        if self.pin_high {
            self.speaker_pin.set_low().unwrap();
            self.pin_high = false;
        } else {
            self.speaker_pin.set_high().unwrap();
            self.pin_high = true;
        }

        // Вычисляем следующий период. Математика здесь немного
        // особенная.

        // Сначала, если нужно, переходим к следующему циклу сирены.
        while self.cur_time >= 2 * RISE_TIME {
            self.cur_time -= 2 * RISE_TIME;
        }
        // Затем определяем, где мы находимся в текущем цикле сирены.
        let cycle_time = if self.cur_time < RISE_TIME {
            self.cur_time
        } else {
            2 * RISE_TIME - self.cur_time
        };
        // Наконец, вычисляем частоту и период.
        let frequency = BASE_FREQ + FREQ_RISE * cycle_time / RISE_TIME;
        let period = 1_000_000 / frequency;

        // Предсказываем время следующего прерывания.
        self.cur_time += period / 2;

        // Обязательно сбросьте текущее прерывание перед
        // запуском следующего, иначе может произойти
        // новое прерывание сразу же.
        self.timer.reset_event();
        self.timer.start(period / 2);
    }
}

/// Сирена. Доступна и из обработчика прерывания,
/// и из основной программы.
static SIREN: LockMut<Siren> = LockMut::new();

/// Прерывание таймера для сирены. Просто переводит сирену к следующему шагу.
#[interrupt]
fn TIMER0() {
    SIREN.with_lock(|siren| siren.step());
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    // Удобно использовать пин, преобразованный через `degrade()`,
    // чтобы не разбираться с типом
    // пина динамика, а не выяснять его:
    // пин хранится глобально в `SIREN`, поэтому его
    // размер должен быть известен.
    //
    // Типобезопасность при этом теряется, но вряд ли
    // это будет важно после этого места.
    let speaker_pin = board
        .speaker_pin
        .into_push_pull_output(gpio::Level::Low)
        .degrade();
    let timer0 = timer::Timer::new(board.TIMER0);
    let mut timer1 = timer::Timer::new(board.TIMER1);

    // Настраиваем NVIC для обработки прерываний.
    unsafe { pac::NVIC::unmask(pac::Interrupt::TIMER0) };
    pac::NVIC::unpend(pac::Interrupt::TIMER0);

    // Размещаем структуру сирены там, где её найдёт обработчик прерывания.
    let siren = Siren::new(speaker_pin, timer0);
    SIREN.init(siren);

    // Запускаем сирену и отсчитываем время.
    SIREN.with_lock(|siren| siren.start());
    for t in (1..=10).rev() {
        rprintln!("{}", t);
        timer1.delay_ms(1_000);
    }
    rprintln!("launch!");
    SIREN.with_lock(|siren| siren.stop());

    loop {
        asm::wfi();
    }
}
