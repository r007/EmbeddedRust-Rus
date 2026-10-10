{{#title Пользовательские символы на LCD-дисплее с Embassy на Raspberry Pi Pico 2}}

# Ferris на ЖК-дисплее

В этом разделе мы нарисуем Ferris на символьном ЖК-дисплее. Это моя попытка сделать так, чтобы он был похож на краба. Если придумаете более удачный рисунок, присылайте pull request.

Хотя один пользовательский символ ограничен одной ячейкой 5x8, мы не обязаны ограничиваться одной ячейкой. Объединив 4 или даже 6 соседних сеток, можно вывести символ побольше. Насколько далеко вы зайдёте, зависит только от вашей фантазии.

Для создания Ferris мы будем использовать генератор пользовательских глифов со предыдущей страницы. Генератор выдаёт массив байтов, который можно сразу использовать в коде.

<img style="display: block; margin: auto;" alt="Ferris из пользовательских символов на LCD1602" src="./images/custom-character-ferris.png"/>

## Крейт liquid_crystal

Крейт hd44780-driver, который мы использовали раньше, не поддерживает определение пользовательских глифов. Для работы с глифами, которые хранятся в CGRAM, мы будем использовать крейт `liquid_crystal`.

Этот крейт поддерживает пользовательские глифы и также предоставляет асинхронный API, который мы используем в этой главе.

### Обновление Cargo.toml

При добавлении зависимости включите функцию async:

```toml
liquid_crystal = { version = "0.2.0", features = ["async"] }
```

## Дополнительные импорты

Добавьте эти импорты в начало main.rs:

```rust
// Привязка прерываний
use embassy_rp::peripherals::I2C0;
use embassy_rp::{bind_interrupts, i2c};

// I2C
use embassy_rp::i2c::{Config as I2cConfig, I2c};

// Драйвер ЖК-дисплея
use liquid_crystal::I2C;
use liquid_crystal::LiquidCrystal;
use liquid_crystal::prelude::*;

use embassy_time::Delay;
```

## Привязка прерывания I2C

Привяжите прерывание `I2C0_IRQ` к обработчику прерываний I2C из Embassy для `I2C0`:

```rust
bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});
```

## Инициализация I2C

Сначала настроим шину I2C для связи с дисплеем:

```rust
let sda = p.PIN_16;
let scl = p.PIN_17;

let mut i2c_config = I2cConfig::default();
i2c_config.frequency = 100_000; // 100кГц

let i2c_bus = I2c::new_async(p.I2C0, scl, sda, Irqs, i2c_config);
```

## Инициализация интерфейса ЖК-дисплея

Когда интерфейс I2C настроен, инициализируем ЖК-дисплей.

```rust
// Инициализация ЖК-дисплея
let mut i2c_interface = I2C::new(i2c_bus, 0x27);
let mut lcd = LiquidCrystal::new(&mut i2c_interface, Bus4Bits, LCD16X2);
lcd.begin(&mut Delay);
```

## Сгенерированный массив байтов для пользовательского глифа

```rust
const FERRIS: [u8; 8] = [
    0b01010, 0b10001, 0b10001, 0b01110, 0b01110, 0b01110, 0b11111, 0b10001,
];
// Определяем символ
lcd.custom_char(&mut Delay, &FERRIS, 0);
```

## Вывод

Вывести символ очень просто. Нужно использовать перечисление CustomChar и передать индекс пользовательского символа. Мы определили только один пользовательский символ, он находится в позиции 0.

```rust
lcd.write(&mut Delay, CustomChar(0));
lcd.write(&mut Delay, Text(" implRust!"));
```

## Клонируйте существующий проект

Можно клонировать проект, который я создал, и перейти в папку `custom-glyph`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/lcd/custom-glyph/
```

## Версия на rp-hal

Можно клонировать проект, который я создал, и перейти в папку `custom-glyph` (версия на rp-hal).

```sh
git clone https://github.com/ImplFerris/pico2-rp-projects
cd pico2-rp-projects/lcd/custom-glyph/
```

## Полный код

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_time::Timer;

//Обработчик паники
use panic_probe as _;
// Логирование через defmt
use defmt_rtt as _;

// Привязка прерываний
use embassy_rp::peripherals::I2C0;
use embassy_rp::{bind_interrupts, i2c};

// I2C
use embassy_rp::i2c::{Config as I2cConfig, I2c};

// Драйвер ЖК-дисплея
use liquid_crystal::I2C;
use liquid_crystal::LiquidCrystal;
use liquid_crystal::prelude::*;

use embassy_time::Delay;

/// Сообщаем Boot ROM о нашем приложении
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});

// const LCD_I2C_ADDRESS: u8 = 0x27;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let sda = p.PIN_16;
    let scl = p.PIN_17;

    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = 100_000; //100кГц

    let i2c_bus = I2c::new_async(p.I2C0, scl, sda, Irqs, i2c_config);

    // Инициализация ЖК-дисплея
    let mut i2c_interface = I2C::new(i2c_bus, 0x27);
    let mut lcd = LiquidCrystal::new(&mut i2c_interface, Bus4Bits, LCD16X2);
    lcd.begin(&mut Delay);

    const FERRIS: [u8; 8] = [
        0b01010, 0b10001, 0b10001, 0b01110, 0b01110, 0b01110, 0b11111, 0b10001,
    ];
    // Определяем символ
    lcd.custom_char(&mut Delay, &FERRIS, 0);

    lcd.write(&mut Delay, CustomChar(0));
    // Обычный текст
    lcd.write(&mut Delay, Text(" implRust!"));

    loop {
        Timer::after_millis(100).await;
    }
}

// Метаданные программы для `picotool info`.
// Это не обязательно, но рекомендуется добавить эти минимальные записи.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"custom-chars"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// Конец файла
```
