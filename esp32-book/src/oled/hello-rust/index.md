{{#title Выводим текст на OLED-дисплей с ESP32 на embedded Rust}}

# Выводим «Hello, Rust!» на OLED с ESP32

Это упражнение — простое знакомство с OLED-дисплеем, поэтому оставим всё просто: выведем «Hello, Rust!» на экран OLED.  


## Создание проекта с помощью esp-generate
Мы включим поддержку async (Embassy) для этого проекта. Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 hello-oled
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

Сначала добавим необходимые импорты:

```rust
// I2C
use esp_hal::i2c::master::Config as I2cConfig; // для удобства импортируем под псевдонимом
use esp_hal::i2c::master::I2c;
use esp_hal::time::Rate;

// OLED
use ssd1306::{I2CDisplayInterface, Ssd1306Async, prelude::*};

// Embedded Graphics
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::Point,
    prelude::*,
    text::{Baseline, Text},
};
```

## Инициализация I2C

Инициализируем интерфейс I2C для связи между ESP32 и OLED-дисплеем. Шина I2C настроена на частоту 400 кГц и тайм-аут в 100 тактов шины. Мы назначаем GPIO18 на SCL (Serial Clock Line, линия тактирования), а GPIO23 — на SDA (Serial Data Line, линия данных) для связи I2C, и включаем асинхронную работу интерфейса.

```rust
let i2c_bus = I2c::new(
        peripherals.I2C0,
        // I2cConfig — псевдоним esp_hal::i2c::master::I2c::Config
        I2cConfig::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO18)
    .with_sda(peripherals.GPIO23)
    .into_async();
```

## Инициализация драйвера ssd1306

Затем мы используем вспомогательную структуру `I2CDisplayInterface`, чтобы создать предварительно настроенный интерфейс I2C для дисплея. Далее используем структуру `Ssd1306Async` (для синхронного варианта — `Ssd1306`), передаём созданный интерфейс, размер дисплея `DisplaySize128x64` и поворот дисплея. Так как поворот нам не нужен, задаём `DisplayRotation::Rotate0`.

Крейт ssd1306 поддерживает три режима работы дисплея:
- BasicMode, который даёт базовое управление с помощью низкоуровневых методов;
- BufferedGraphicsMode, который использует буфер кадра для расширенного рисования и интегрируется с embedded-graphics;
- и TerminalMode — режим без буфера, предназначенный для вывода текста и установки позиции курсора, как в терминале.

В этом упражнении мы будем использовать BufferedGraphicsMode.

Затем вызываем функцию `init()`, которая инициализирует дисплей в графическом режиме и очищает его.

```rust
let interface = I2CDisplayInterface::new(i2c_bus);
// инициализируем дисплей
let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
    .into_buffered_graphics_mode();
display.init().await.unwrap();
```

### Стиль текста и позиция

Для вывода текста мы будем использовать моноширинные шрифты. Построитель `MonoTextStyleBuilder` поможет создать стиль текста, а мы используем шрифт размером 6x10 пикселей. Другие моноширинные шрифты можно найти [здесь](https://docs.rs/embedded-graphics/latest/embedded_graphics/mono_font/ascii/index.html).

Если у вас многоцветный OLED-дисплей, можно задать разные цвета шрифта. Однако, так как у нас монохромный дисплей, мы используем `BinaryColor::On`, чтобы задать белый цвет текста. Это просто включает пиксели, нужные для отображения текста.

```rust
let text_style = MonoTextStyleBuilder::new()
    .font(&FONT_6X10)
    .text_color(BinaryColor::On)
    .build();

Text::with_baseline("Hello, Rust!", Point::new(0, 16), text_style, Baseline::Top)
    .draw(&mut display)
    .unwrap();
```

Базовая линия (baseline) — воображаемая линия, которая определяет выравнивание текста. Мы задаём базовую линию с координатами x = 0 и y = 16. Также указываем, как текст выравнивается в этом пространстве. Перечисление `Baseline` управляет положением текста относительно базовой линии. Например, `Baseline::Top` выравнивает верх текста по начальной точке, а `Baseline::Bottom` — низ текста по начальной точке. Есть и другие варианты, например `Middle`, `Alphabetic`.

Рекомендую поменять значения точки и параметр `Baseline` и посмотреть, как они влияют на результат. Визуальные изменения помогут лучше всё понять.

Затем можно вывести текст на любой объект, который реализует трейт `DrawTarget`. Буферный режим `BufferedGraphicsMode` крейта ssd1306 реализует этот трейт, поэтому мы передаём дисплей в функцию `draw` как изменяемую ссылку.

## Flush

Наконец, вызываем функцию `flush`, которая записывает данные на дисплей. Только после этого обновлённое содержимое появится на OLED-экране.

```rust
display.flush().await.unwrap();
```

## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `hello-oled`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/hello-oled
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
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::Point,
    prelude::*,
    text::{Baseline, Text},
};

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

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello, Rust!", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().await.unwrap();

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
```
