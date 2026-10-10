{{#title Код на Rust для вывода сырых изображений на OLED-дисплей с ESP32}}

# Код

Надеюсь, теперь вы понимаете, как изображение представлено в байтовом массиве. Перейдём к программированию.

## Создание проекта с помощью esp-generate
Мы включим поддержку async (Embassy) для этого проекта. Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 oled-image
```

Откроется экран с вопросом о выборе опций. 

- Выберите опцию «Enable unstable HAL features»
- Затем выберите опцию «Adds embassy framework support».

Сохраните настройки, нажав на клавиатуре «s».


## Обновляем Cargo.toml

```toml
ssd1306 = { version = "0.10.0", features = ["async"] }
embedded-graphics = "0.8.1"
```

## Импорты

Добавьте необходимые импорты:

```rust

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
```

## Шаблонный код: инициализация I2C и экземпляра дисплея

Эту часть мы уже разобрали в предыдущей [главе](../hello-rust/index.md).

```rust
let i2c_bus = esp_hal::i2c::master::I2c::new(
    peripherals.I2C0,
    esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
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
```

## Вывод изображения

Мы создали константу с байтовым массивом, которая представляет символ Ом.

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

Создадим сырое изображение с помощью функции `ImageRaw::new`. Нужно указать ширину изображения (то есть 8) и формат цвета пикселей с помощью синтаксиса turbofish `::<>`. Высота изображения будет вычислена автоматически по длине данных и формату. Поскольку наш дисплей двухцветный, мы используем перечисление `BinaryColor`.

Затем выведем изображение в начальную позицию дисплея — точку ноль (x = 0, y = 0). Наконец, передадим данные на дисплей командой `flush`.

```rust
let raw_image = ImageRaw::<BinaryColor>::new(IMG_DATA, 8);

let image = Image::new(&raw_image, Point::zero());

image.draw(&mut display).unwrap();
display.flush().await.unwrap();
```

## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `oled-image`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/oled-image
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

// 8x5 пикселей
#[rustfmt::skip]
const IMG_DATA: &[u8] = &[
    0b00111000,
    0b01000100,
    0b01000100,
    0b00101000,
    0b11101110,
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

    let raw_image = ImageRaw::<BinaryColor>::new(IMG_DATA, 8);

    let image = Image::new(&raw_image, Point::zero());

    image.draw(&mut display).unwrap();
    display.flush().await.unwrap();

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
```
