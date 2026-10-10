{{#title Мигание внешним светодиодом с помощью rp-hal на Raspberry Pi Pico 2 | impl Rust для RP2350}}

# Пример Blinky с использованием rp-hal

В предыдущем разделе мы использовали Embassy. Схему и подключение оставляем те же. В этом примере переходим на `rp-hal`, чтобы показать, как выглядят оба подхода. Embassy подойдёт, если нужна поддержка `async`, а `rp-hal` — если вы предпочитаете блокирующий стиль. В этой книге мы в основном будем использовать Embassy.

Снова создадим проект с помощью `cargo-generate` и того же шаблона.

В терминале введите:

```sh
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

Когда программа предложит выбрать HAL, выберите на этот раз «rp-hal».

## Импорты

Шаблон уже содержит большинство импортов. Для этого примера нужно добавить трейт `OutputPin` из embedded-hal:

```rust
// Трейт Embedded HAL для выходного вывода (Output Pin)
use embedded_hal::digital::OutputPin;
```

Этот трейт предоставляет методы `set_high()` и `set_low()`, которыми мы будем управлять светодиодом.

## Основная логика

Если сравнить с версией на Embassy, разница в переключении светодиода невелика. Основное отличие — в том, как работает задержка. Embassy использует `async` и `await`, что позволяет программе приостанавливаться без блокировки и даёт другим задачам выполняться в фоне. rp-hal использует блокирующий `delay`, который останавливает программу, пока не пройдёт нужное время.

```rust
let mut led_pin = pins.gpio13.into_push_pull_output();

loop {
    led_pin.set_high().unwrap();
    timer.delay_ms(200);

    led_pin.set_low().unwrap();
    timer.delay_ms(200);
}
```

## Полный код

```rust
#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use hal::block::ImageDef;
use rp235x_hal as hal;

//Обработчик паники
use panic_probe as _;
// Логирование через defmt
use defmt_rtt as _;

// Трейт Embedded HAL для выходного вывода (Output Pin)
use embedded_hal::digital::OutputPin;

/// Сообщаем Boot ROM о нашем приложении
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();
/// Внешний высокоскоростной кварц на плате Raspberry Pi Pico 2 работает на 12 МГц.
/// Измените значение, если на вашей плате другая частота
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

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

    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    let mut led_pin = pins.gpio13.into_push_pull_output();

    loop {
        led_pin.set_high().unwrap();
        timer.delay_ms(200);

        led_pin.set_low().unwrap();
        timer.delay_ms(200);
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
```

## Клонируйте существующий проект

Можно клонировать проект, который я создал, и перейти в папку `external-led`:

```sh
git clone https://github.com/ImplFerris/pico2-rp-projects
cd pico2-rp-projects/external-led
```
