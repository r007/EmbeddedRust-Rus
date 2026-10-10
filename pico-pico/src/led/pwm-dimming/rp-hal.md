{{#title Плавное изменение яркости светодиода с ШИМ на rp-hal для Raspberry Pi Pico 2 (embedded Rust)}}

# Программа плавного изменения яркости светодиода с RP HAL

rp-hal — это реализация Embedded-HAL для микроконтроллеров серии RP и альтернатива фреймворку Embassy для Pico.

Этот пример взят из репозитория rp235x-hal (там также есть примеры, не ограничивающиеся мигающим светодиодом):

[https://github.com/rp-rs/rp-hal/tree/main/rp235x-hal-examples](https://github.com/rp-rs/rp-hal/tree/main/rp235x-hal-examples)

## Основной код

```rust
#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use hal::block::ImageDef;
use rp235x_hal as hal;

// Трейт для ШИМ
use embedded_hal::pwm::SetDutyCycle;

//Обработчик паники
use panic_probe as _;
// Логирование через defmt
use defmt_rtt as _;

/// Сообщаем Boot ROM о нашем приложении
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();
/// Внешний высокоскоростной кварц на плате Raspberry Pi Pico 2 работает на 12 МГц.
/// Измените значение, если на вашей плате другая частота
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

/// Минимальное значение ШИМ (то есть яркости светодиода), которое нам нужно
const LOW: u16 = 0;

/// Максимальное значение ШИМ (то есть яркости светодиода), которое нам нужно
const HIGH: u16 = 25000;

#[hal::entry]
fn main() -> ! {
    // Получаем наши синглтон-объекты
    let mut pac = hal::pac::Peripherals::take().unwrap();

    // Настраиваем драйвер сторожевого таймера — он нужен для настройки тактирования
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Настраиваем тактирование
    //
    // По умолчанию генерируется системная частота 125 МГц
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Блок SIO с однотактным доступом управляет нашими выводами GPIO
    let sio = hal::Sio::new(pac.SIO);

    // Настраиваем выводы в соответствии с их функциями на этой плате
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Инициализируем ШИМ
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    // Настраиваем PWM4
    let pwm = &mut pwm_slices.pwm4;
    pwm.set_ph_correct();
    pwm.enable();

    // Выводим канал B слайса PWM4 на GPIO 25
    let channel = &mut pwm.channel_b;
    channel.output_to(pins.gpio25);

    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    loop {
        for i in LOW..=HIGH {
            timer.delay_us(8);
            let _ = channel.set_duty_cycle(i);
        }

        for i in (LOW..=HIGH).rev() {
            timer.delay_us(8);
            let _ = channel.set_duty_cycle(i);
        }

        timer.delay_ms(500);
    }
}
// Метаданные программы для `picotool info`.
// Это не обязательно, но рекомендуется добавить эти минимальные записи.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 5] = [
    hal::binary_info::rp_cargo_bin_name!(),
    hal::binary_info::rp_cargo_version!(),
    hal::binary_info::rp_program_description!(c"your program description"),
    hal::binary_info::rp_cargo_homepage_url!(),
    hal::binary_info::rp_program_build_attribute!(),
];

// Конец файла
```

## Клонируйте существующий проект

Можно клонировать проект с мигающим светодиодом, который я создал, и перейти в папку `led-dimming`, чтобы запустить эту версию программы:

```sh
git clone https://github.com/ImplFerris/pico2-rp-projects
cd pico2-rp-projects/led-dimming
```
