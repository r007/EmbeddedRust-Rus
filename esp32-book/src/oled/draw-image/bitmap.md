{{#title Как вывести BMP-изображения на OLED-дисплей с ESP32 на Rust}}

# Использование файла растрового изображения

Вместо сырых данных изображения можно использовать файлы BMP (.bmp) напрямую, применив крейт [tinybmp](https://docs.rs/tinybmp/latest/tinybmp/). tinybmp — это лёгкий парсер BMP, созданный для встраиваемых окружений. Он в первую очередь предназначен для вывода BMP-изображений в DrawTarget embedded_graphics, но его также можно использовать для разбора BMP-файлов в других задачах. Это идеально подходит для наших целей.

## Файл BMP
Крейт требует, чтобы изображение было в формате BMP. Если ваше изображение в другом формате, его нужно конвертировать в BMP. Например, в Linux можно использовать такую команду, чтобы превратить PNG в монохромный BMP:

```sh
convert ferris.png -monochrome ferris.bmp
```

Я подготовил файл Ferris в формате BMP, который можно использовать для этого упражнения. Скачать его можно [здесь](../images/ferris.bmp).

<img style="display: block; margin: auto;" alt="файл ferris.bmp" src="../images/ferris.bmp"/>

## Основа проекта

Мы скопируем проект с предыдущим изображением и продолжим работу на его основе.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cp -r esp32-projects/old-image ~/YOUR_PROJECT_FOLDER/oled-bmp
```

## Обновляем Cargo.toml

Нам понадобится ещё один крейт, `tinybmp`, чтобы загрузить BMP-изображение.

```toml
tinybmp = "0.6.0"

```

## Использование BMP-файла

Поместите файл `ferris.bmp` в папку `src`. Код довольно простой: загрузите изображение как байты и передайте их в функцию `from_slice` у `Bmp`. Затем используйте результат с `Image`.

```rust
// здесь обычный шаблонный код...

// Подключаем данные BMP-файла.
let bmp_data = include_bytes!("../ferris.bmp");

// Разбираем BMP-файл.
let bmp = Bmp::from_slice(bmp_data).unwrap();

// обычный код:
let image = Image::new(&bmp, Point::new(32, 0));
image.draw(&mut display).unwrap();
display.flush().await.unwrap();
```

## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `oled-bmp`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/oled-bmp
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
use embedded_graphics::{image::Image, prelude::Point, prelude::*};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Создаёт дескриптор приложения по умолчанию, который требуется загрузчику esp-idf.
// Подробнее см.: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

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

    // Подключаем данные BMP-файла.
    let bmp_data = include_bytes!("../ferris.bmp");

    // Разбираем BMP-файл.
    let bmp = tinybmp::Bmp::from_slice(bmp_data).unwrap();

    // обычный код:
    let image = Image::new(&bmp, Point::new(32, 0));
    image.draw(&mut display).unwrap();
    display.flush().await.unwrap();

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
```
