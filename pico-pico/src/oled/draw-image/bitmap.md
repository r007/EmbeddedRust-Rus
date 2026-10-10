{{#title Вывод BMP-изображения на OLED с помощью tinybmp на Raspberry Pi Pico 2}}

# Использование файла растрового изображения

Вместо сырых данных изображения можно использовать файлы BMP (.bmp) напрямую, подключив крейт [tinybmp](https://docs.rs/tinybmp/latest/tinybmp/). tinybmp — лёгкий парсер BMP, созданный для встраиваемых окружений. В основном он предназначен для вывода BMP-изображений в DrawTarget из embedded_graphics, но его также можно использовать для разбора файлов BMP в других задачах.

## Файл BMP

Крейт требует, чтобы изображение было в формате BMP. Если ваше изображение в другом формате, его нужно преобразовать в BMP. Например, в Linux можно использовать такую команду, чтобы преобразовать PNG в монохромный BMP:

```sh
convert ferris.png -monochrome ferris.bmp
```

Я подготовил [файл BMP с Ferris](../images/ferris.bmp), который вы можете скачать и использовать в этом упражнении.

<img style="display: block; margin: auto;" alt="Файл BMP с Ferris" src="../images/ferris.bmp"/>

## Основа проекта

Мы скопируем проект oled-rawimg и будем работать поверх него.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cp -r pico2-embassy-projects/oled/oled-rawimg ~/YOUR_PROJECT_FOLDER/oled-bmp
```

Или можно просто создать новый проект из шаблона и повторить те же шаги, что и раньше.

## Обновление Cargo.toml

Нужен ещё один крейт — «tinybmp», чтобы загружать BMP-изображение.

```toml
tinybmp = "0.6.0"
```

## Использование файла BMP

Поместите файл «ferris.bmp» в папку src. Код довольно простой: загружаем изображение как байты и передаём их в функцию `from_slice` структуры `Bmp`. Затем можно использовать результат с `Image`.

```rust
// Здесь стандартный шаблонный код...

// Подключаем данные файла BMP.
let bmp_data = include_bytes!("../ferris.bmp");

// Разбираем файл BMP.
let bmp = Bmp::from_slice(bmp_data).unwrap();

// Обычный код:
let image = Image::new(&bmp, Point::new(32, 0));

image
    .draw(&mut display)
    .expect("failed to draw text to display");

defmt::info!("Displaying image");
display.flush().await.expect("failed to flush data to display");
```

## Клонируйте существующий проект

Также можно клонировать проект, который я создал, и перейти в папку `oled-bmp`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/oled/oled-bmp
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
use embedded_graphics::{image::Image, prelude::Point, prelude::*};
use tinybmp::Bmp;

/// Сообщаем Boot ROM о нашем приложении
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});

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

    // Подключаем данные файла BMP.
    let bmp_data = include_bytes!("../ferris.bmp");

    // Разбираем файл BMP.
    let bmp = Bmp::from_slice(bmp_data).unwrap();

    // Обычный код:
    let image = Image::new(&bmp, Point::new(32, 0));

    image
        .draw(&mut display)
        .expect("failed to draw text to display");

    defmt::info!("Displaying image");
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
