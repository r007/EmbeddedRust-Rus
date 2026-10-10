# Hello Rust на OLED

То же самое приветствие мы сделаем и с `rp-hal`.

## Создание проекта из шаблона

Чтобы создать проект, выполните:

```sh
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

Когда появится запрос, придумайте имя проекта — пусть будет «oh-led». Не забудьте выбрать `rp-hal` в качестве HAL.

Затем перейдите в папку проекта:

```sh
cd PROJECT_NAME
# Например, если вы назвали проект «oh-led»:
# cd oh-led
```

### Добавление дополнительных зависимостей

Так как мы используем OLED-дисплей на SSD1306, нужно подключить драйвер SSD1306. Чтобы добавить эту зависимость, используйте команду cargo:

```sh
cargo add ssd1306@0.10.0
```

Для вывода графики на OLED-дисплей, рисования изображений, фигур и текста мы будем использовать крейт embedded_graphics.

```sh
cargo add embedded-graphics@0.8.1
```

## Дополнительные импорты

Помимо импортов из шаблона, для этой задачи понадобятся следующие зависимости.

```rust
// Embedded Graphics
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Baseline, Text};

// Для установки частоты
use hal::fugit::RateExtU32;
use hal::gpio::{FunctionI2C, Pin};

// Дисплей SSD1306
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};
```

## Настройка выводов

Начнём с настройки выводов GPIO для связи по I2C. В этом случае GPIO16 назначен как вывод SDA, а GPIO17 — как вывод SCL. Затем настраиваем периферию I2C в режиме контроллера.

```rust
// Переназначаем два вывода как I²C, а не GPIO
let sda_pin: Pin<_, FunctionI2C, _> = pins.gpio16.reconfigure();
let scl_pin: Pin<_, FunctionI2C, _> = pins.gpio17.reconfigure();

// Создаём шину I²C, используя два заранее настроенных вывода. Это вызовет
// ошибку на этапе компиляции, если выводы в неправильном режиме или если эта
// периферия I²C недоступна на этих выводах!
let i2c = hal::I2C::i2c0(
    pac.I2C0,
    sda_pin,
    scl_pin,
    400.kHz(),
    &mut pac.RESETS,
    &clocks.system_clock,
);
```

### Подготовка дисплея

Создаём интерфейс для OLED-дисплея через I2C.

```rust
// Вспомогательную структуру предоставляет крейт ssd1306
let interface = I2CDisplayInterface::new(i2c);
// Инициализируем дисплей
let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
    .into_buffered_graphics_mode();
display.init().expect("failed to initialize the display");
```

### Задаём стиль текста и рисуем

Далее определяем стиль текста и используем его, чтобы вывести «Hello Rust» на экран:

```rust
// Embedded graphics
let text_style = MonoTextStyleBuilder::new()
    .font(&FONT_6X10)
    .text_color(BinaryColor::On)
    .build();

Text::with_baseline(
    "Hello, Rusty!",
    Point::new(0, 16),
    text_style,
    Baseline::Top,
)
.draw(&mut display)
.expect("failed to draw text to display");
```

Здесь мы пишем сообщение с координатами (x=0, y=16).

### Выгрузка данных на дисплей

```rust
display.flush().expect("failed to flush data to display");
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

// Embedded Graphics
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Baseline, Text};

// Для установки частоты
use hal::fugit::RateExtU32;
use hal::gpio::{FunctionI2C, Pin};

// Дисплей SSD1306
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};

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

    // Переназначаем два вывода как I²C, а не GPIO
    let sda_pin: Pin<_, FunctionI2C, _> = pins.gpio16.reconfigure();
    let scl_pin: Pin<_, FunctionI2C, _> = pins.gpio17.reconfigure();

    // Создаём шину I²C, используя два заранее настроенных вывода. Это вызовет
    // ошибку на этапе компиляции, если выводы в неправильном режиме или если эта
    // периферия I²C недоступна на этих выводах!
    let i2c = hal::I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    // Вспомогательную структуру предоставляет крейт ssd1306
    let interface = I2CDisplayInterface::new(i2c);
    // Инициализируем дисплей
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().expect("failed to initialize the display");

    // Embedded graphics
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline(
        "Hello, Rusty!",
        Point::new(0, 16),
        text_style,
        Baseline::Top,
    )
    .draw(&mut display)
    .expect("failed to draw text to display");

    display.flush().expect("failed to flush data to display");

    loop {
        timer.delay_ms(100);
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

Можно клонировать проект, который я создал, и перейти в папку `hello-oled`.

```sh
git clone https://github.com/ImplFerris/pico2-rp-projects
cd pico2-rp-projects/oled/hello-oled
```
