{{#title Код на Rust для вывода многобайтовых изображений на OLED и ESP32}}

# Код

Основные изменения в коде — это данные изображения и его ширина. Так будет выведен символ резистора в стиле IEC-60617.


## Основа проекта

Мы скопируем проект с предыдущим изображением и продолжим работу на его основе.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cp -r esp32-projects/old-image ~/YOUR_PROJECT_FOLDER/oled-rawimg
```


## Данные изображения

На этот раз мы используем следующие данные, чтобы вывести символ резистора на OLED.

```rust
// 31x7 пикселей
#[rustfmt::skip]
const IMG_DATA: &[u8] = &[
    // 1-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
    // 2-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
    //3-я строка
    0b00000001,0b10000000,0b00000011,0b00000000,
    //4-я строка
    0b11111111,0b10000000,0b00000011,0b11111110,
    //5-я строка
    0b00000001,0b10000000,0b00000011,0b00000000,
    //6-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
    //7-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
];
```

Нужно задать ширину 31. Изображение выведем в точке (x=35, y=35), хотя причин выбрать именно такие координаты особых нет: мне просто хотелось показать что-то, отличное от нулевой точки. Не стесняйтесь экспериментировать с разными значениями точки и изучать другие варианты.

```rust
 let raw_image = ImageRaw::<BinaryColor>::new(IMG_DATA, 31);

let image = Image::new(&raw_image, Point::new(35, 35));
```

## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `oled-rawimg`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/oled-rawimg
```

## Полный код

```rust
#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;

// I2C
use esp_hal::i2c::master::Config as I2cConfig; // для удобства импортируем под псевдонимом
use esp_hal::i2c::master::I2c;
use esp_hal::time::Rate;

// OLED
use ssd1306::{I2CDisplayInterface, Ssd1306Async, prelude::*};

// Embedded Graphics
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::Point,
    prelude::*,
};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Создаёт дескриптор приложения по умолчанию, который требуется загрузчику esp-idf.
// Подробнее см.: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

// 31x7 пикселей
#[rustfmt::skip]
const IMG_DATA: &[u8] = &[
    // 1-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
    // 2-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
    //3-я строка
    0b00000001,0b10000000,0b00000011,0b00000000,
    //4-я строка
    0b11111111,0b10000000,0b00000011,0b11111110,
    //5-я строка
    0b00000001,0b10000000,0b00000011,0b00000000,
    //6-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
    //7-я строка
    0b00000001,0b11111111,0b11111111,0b00000000,
];

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.0.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    info!("Embassy initialized!");

    let _ = spawner;

    let i2c_bus = I2c::new(
        peripherals.I2C0,
        // I2cConfig — псевдоним esp_hal::i2c::master::I2c::Config
        I2cConfig::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO18)
    .with_sda(peripherals.GPIO23)
    .into_async();

    let interface = I2CDisplayInterface::new(i2c_bus);

    // инициализируем дисплей
    let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().await.unwrap();

    let raw_image = ImageRaw::<BinaryColor>::new(IMG_DATA, 31);
    let image = Image::new(&raw_image, Point::new(35, 35));

    image.draw(&mut display).unwrap();
    display.flush().await.unwrap();

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
```
