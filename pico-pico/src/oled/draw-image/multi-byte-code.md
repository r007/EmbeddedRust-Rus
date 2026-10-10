## Рисуем многобайтовое изображение в embedded Rust с embedded-graphics

Теперь напишем код, который выводит более широкое изображение на OLED-экран. Основные отличия от предыдущего примера — данные изображения и значение ширины. На этот раз мы выведем резистор в стиле IEC-60617.

## Основа проекта

Мы скопируем проект byte-oled и будем работать поверх него.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cp -r pico2-embassy-projects/oled/byte-oled ~/YOUR_PROJECT_FOLDER/oled-rawimg
```

Или можно просто создать новый проект из шаблона и повторить те же шаги, что и раньше.

## Данные изображения

Вот массив байтов для символа резистора. Обратите внимание, что каждой строке нужно несколько байтов, потому что изображение шириной 31 пиксель.

```rust
// 31x7 пикселей
#[rustfmt::skip]
const IMG_DATA: &[u8] = &[
    // 1-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
    // 2-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
    // 3-я строка
    0b00000001,0b10000000,0b00000011,0b00000000,
    // 4-я строка
    0b11111111,0b10000000,0b00000011,0b11111110,
    // 5-я строка
    0b00000001,0b10000000,0b00000011,0b00000000,
    // 6-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
    // 7-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
];
```

## Создание и размещение изображения

Нужно задать ширину 31 пиксель. Изображение мы выведем в позиции (x=35, y=35). Для этих координат нет особой причины: я просто хотел показать, что изображение можно разместить в любом месте экрана, а не только в точке ноль. Не стесняйтесь пробовать другие значения положения и смотреть, что получится.

```rust
let raw_image = ImageRaw::<BinaryColor>::new(IMG_DATA, 31);

let image = Image::new(&raw_image, Point::new(35, 35));
```

## Клонируйте существующий проект

Также можно клонировать проект, который я создал, и перейти в папку `oled-rawimg`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/oled/oled-rawimg
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

// OLED
use ssd1306::{I2CDisplayInterface, Ssd1306Async, prelude::*};

// Embedded Graphics
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::Point,
    prelude::*,
};

/// Сообщаем Boot ROM о нашем приложении
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});

// 31x7 пикселей
#[rustfmt::skip]
const IMG_DATA: &[u8] = &[
    // 1-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
    // 2-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
    // 3-я строка
    0b00000001,0b10000000,0b00000011,0b00000000,
    // 4-я строка
    0b11111111,0b10000000,0b00000011,0b11111110,
    // 5-я строка
    0b00000001,0b10000000,0b00000011,0b00000000,
    // 6-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
    // 7-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let sda = p.PIN_16;
    let scl = p.PIN_17;

    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = 400_000; // 400kHz

    let i2c_bus = I2c::new_async(p.I2C0, scl, sda, Irqs, i2c_config);

    let i2c_interface = I2CDisplayInterface::new(i2c_bus);

    let mut display = Ssd1306Async::new(i2c_interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display
        .init()
        .await
        .expect("failed to initialize the display");

    let raw_image = ImageRaw::<BinaryColor>::new(IMG_DATA, 31);

    let image = Image::new(&raw_image, Point::new(35, 35));

    image
        .draw(&mut display)
        .expect("failed to draw text to display");

    display
        .flush()
        .await
        .expect("failed to flush data to display");

    loop {
        Timer::after_millis(100).await;
    }
}

// Метаданные программы для `picotool info`.
// Это не обязательно, но рекомендуется добавить эти минимальные записи.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"oled-rawimg"),
    embassy_rp::binary_info::rp_program_description!(c"Multi Byte Image on OLED"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// Конец файла
```
