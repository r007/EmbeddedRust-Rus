# Вывод изображения из одного байта в embedded Rust с embedded-graphics

К этому моменту, надеюсь, вы поняли, как изображение представлено массивом байтов. Теперь перейдём к части с кодом.

## Создание проекта

Как обычно, создайте проект из шаблона с помощью cargo-generate:

```sh
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

Когда появится запрос, задайте проекту имя, например «byte-oled», и выберите «embassy» в качестве HAL. Включите логирование defmt, если у вас есть отладочный зонд, чтобы также видеть логи.

## Обновление зависимостей

Добавьте следующие строки в раздел dependencies вашего Cargo.toml:

```toml
embedded-graphics = "0.8.1"
ssd1306 = { version = "0.10.0", features = ["async"] }
```

## Дополнительные импорты

Добавьте эти импорты в начало main.rs:

```rust
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
```

## Шаблонный код

Эту часть мы уже разобрали в предыдущей главе.

### Привязка прерывания I2C

```rust
bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});
```

### Инициализация I2C и экземпляра дисплея

```rust
let sda = p.PIN_16;
let scl = p.PIN_17;

let mut i2c_config = I2cConfig::default();
i2c_config.frequency = 400_000; // 400 кГц

let i2c_bus = I2c::new_async(p.I2C0, scl, sda, Irqs, i2c_config);

let i2c_interface = I2CDisplayInterface::new(i2c_bus);

let mut display = Ssd1306Async::new(i2c_interface, DisplaySize128x64, DisplayRotation::Rotate0)
    .into_buffered_graphics_mode();

display
    .init()
    .await
    .expect("failed to initialize the display");
```

## Создаём изображение

Изображение мы храним как массив байтов. Каждый байт представляет одну строку пикселей.

```rust
// 8x5 пикселей
#[rustfmt::skip]
const IMG_DATA: &[u8] = &[
    0b00111000,
    0b01000100,
    0b01000100,
    0b00101000,
    0b11101110,
];
```

Это символ Ом (Ω) шириной 8 пикселей и высотой 5 пикселей. Каждая запись `0b` означает, что число записано в двоичном виде с помощью единиц и нулей. Единица означает, что пиксель ВКЛЮЧЁН (белый), а ноль — что пиксель ВЫКЛЮЧЕН (чёрный). Каждая строка представляет одну строку изображения сверху вниз.

## Выводим изображение

Теперь выведем изображение на дисплей:

```rust
let raw_image = ImageRaw::<BinaryColor>::new(IMG_DATA, 8);
let image = Image::new(&raw_image, Point::zero());
```

Первая строка создаёт сырое изображение из наших байтов. Мы говорим, что ширина изображения 8 пикселей, а высоту оно определяет само. Вторая строка размещает изображение в позиции (0, 0), то есть в левом верхнем углу экрана.

## Отображаем изображение

Как и в предыдущей главе, нужно нарисовать изображение в буфер дисплея и затем отправить его на экран:

```rust
image.draw(&mut display).expect("failed to draw text to display");
display.flush().await.expect("failed to flush data to display");
```

## Клонируйте существующий проект

Также можно клонировать проект, который я создал, и перейти в папку `byte-oled`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/oled/byte-oled
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

// 8x5 пикселей
#[rustfmt::skip]
const IMG_DATA: &[u8] = &[
    0b00111000,
    0b01000100,
    0b01000100,
    0b00101000,
    0b11101110,
];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let sda = p.PIN_16;
    let scl = p.PIN_17;

    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = 400_000; // 400 кГц

    let i2c_bus = I2c::new_async(p.I2C0, scl, sda, Irqs, i2c_config);

    let i2c_interface = I2CDisplayInterface::new(i2c_bus);

    let mut display = Ssd1306Async::new(i2c_interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display
        .init()
        .await
        .expect("failed to initialize the display");

    let raw_image = ImageRaw::<BinaryColor>::new(IMG_DATA, 8);

    let image = Image::new(&raw_image, Point::zero());

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
    embassy_rp::binary_info::rp_program_name!(c"byte-oled"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// Конец файла
```
