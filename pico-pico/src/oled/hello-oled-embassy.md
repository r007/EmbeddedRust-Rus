{{#title Вывод текста на OLED с Embassy и SSD1306 на Raspberry Pi Pico 2}}

# Hello OLED

Будем делать всё просто: выведем «Hello, Rust!» на OLED-дисплей. Сначала сделаем это с Embassy, а затем повторим то же самое с помощью `rp-hal`.

## Создание проекта

Как обычно, создайте проект из шаблона с помощью `cargo-generate`:

```sh
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

Когда появится запрос, задайте проекту имя, например «hello-oled», и выберите «embassy» в качестве HAL. Включите логирование `defmt`, если у вас есть отладочный зонд, чтобы также видеть логи.

## Обновление зависимостей

Добавьте следующие строки в `Cargo.toml` в раздел dependencies:

```toml
embedded-graphics = "0.8.1"
ssd1306 = { version = "0.10.0", features = ["async"] }
```

Мы включаем функцию `async`, чтобы драйвер `ssd1306` работал с асинхронным I2C из Embassy. Можно использовать его и без этой функции, тогда I2C из Embassy будет работать в блокирующем режиме.

## Дополнительные импорты

Добавьте эти импорты в начало `main.rs`:

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
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::Point,
    prelude::*,
    text::{Baseline, Text},
};
```

## Привязка прерывания I2C

Мы подробно обсуждали это в разделе о прерываниях, так что вы уже должны знать, что это делает. Здесь прерывание `I2C0_IRQ` привязывается к обработчику прерываний I2C из Embassy для `I2C0`.

```rust
bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});
```

## Инициализация I2C

Сначала нужно настроить шину I2C для связи с дисплеем.

```rust
let sda = p.PIN_16;
let scl = p.PIN_17;

let mut i2c_config = I2cConfig::default();
i2c_config.frequency = 400_000; //400 кГц

let i2c_bus = I2c::new_async(p.I2C0, scl, sda, Irqs, i2c_config);
```

Мы подключили линию SDA OLED к выводу 16, а линию SCL — к выводу 17. В этой главе мы будем использовать эти же выводы. Если вы подключили дисплей к другой допустимой паре I2C, измените код, чтобы он соответствовал вашей проводке.

Мы используем метод `new_async`, чтобы создать экземпляр I2C в асинхронном режиме. Это позволяет передачам I2C ожидать завершения (await), вместо того чтобы блокировать процессор. Скорость шины 400 кГц — распространённое значение, которое поддерживают дисплеи SSD1306.

## Инициализация дисплея

Теперь создаём интерфейс дисплея и инициализируем его:

```rust
let i2c_interface = I2CDisplayInterface::new(i2c_bus);

let mut display = Ssd1306Async::new(i2c_interface, DisplaySize128x64, DisplayRotation::Rotate0)
    .into_buffered_graphics_mode();
```

`I2CDisplayInterface::new(i2c_bus)` оборачивает асинхронную шину I2C, чтобы её мог использовать драйвер SSD1306. Он использует адрес I2C по умолчанию 0x3C, который стандартен для большинства модулей SSD1306.

Мы создаём экземпляр дисплея, указав дисплей 128x64 и ориентацию по умолчанию. Также включаем режим буферизованной графики, чтобы рисовать в буфер RAM с помощью embedded-graphics.

```rust
display
    .init()
    .await
    .expect("failed to initialize the display");
```

Наконец, `display.init()` отправляет команды инициализации аппаратуре дисплея. Это пробуждает дисплей и правильно его настраивает.

## Вывод текста

Прежде чем рисовать текст, нужно определить, как он будет выглядеть:

```rust
 let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
```

Здесь создаётся стиль текста с использованием FONT_6X10 — встроенного моноширинного шрифта шириной 6 пикселей и высотой 10 пикселей. Мы задаём `BinaryColor::On`, чтобы выводить белые пиксели на чёрном фоне, поскольку OLED монохромный.

Теперь выведем текст в буфер дисплея:

```rust
defmt::info!("sending text to display");
Text::with_baseline("Hello, Rust!", Point::new(0, 16), text_style, Baseline::Top)
    .draw(&mut display)
    .expect("failed to draw text to display");
```

Мы выводим «Hello, Rust!» в позицию (0, 16), то есть на 16 пикселей ниже верхнего края экрана. Используем стиль текста, который определили ранее, и выравниваем текст по верхнему краю с помощью `Baseline::Top`.

Вызов `.draw(&mut display)` отрисовывает текст во внутренний буфер дисплея. На этом этапе текст существует в RAM, но ещё не виден на физическом экране.

## Отображение текста

Наконец, отправляем содержимое буфера на настоящую OLED-аппаратуру:

```rust
display
    .flush()
    .await
    .expect("failed to flush data to display");
```

Вот когда происходит обмен по I2C. Драйвер отправляет байты из RAM контроллеру дисплея, и вы увидите, как на OLED-экране появляется «Hello, Rust!»!

## Полный код

Вот всё вместе:

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
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::Point,
    prelude::*,
    text::{Baseline, Text},
};

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
    i2c_config.frequency = 400_000; //400 кГц

    let i2c_bus = I2c::new_async(p.I2C0, scl, sda, Irqs, i2c_config);

    let i2c_interface = I2CDisplayInterface::new(i2c_bus);

    let mut display = Ssd1306Async::new(i2c_interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display
        .init()
        .await
        .expect("failed to initialize the display");

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    defmt::info!("sending text to display");
    Text::with_baseline("Hello, Rust!", Point::new(0, 16), text_style, Baseline::Top)
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
    embassy_rp::binary_info::rp_program_name!(c"hello-oled"),
    embassy_rp::binary_info::rp_program_description!(c"Hello OLED"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// Конец файла
```

## Клонируйте существующий проект

Можно клонировать проект, который я создал, и перейти в папку `hello-oled`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/oled/hello-oled
```
