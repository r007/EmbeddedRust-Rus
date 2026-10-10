# Пишем код на embedded Rust для вывода температуры на OLED-дисплей

В этом разделе мы переходим к части с кодом. Мы напишем программу, которая считывает значение термистора через АЦП, переводит его в температуру по уравнению B и выводит результат на OLED-дисплей по I2C.

## Проект из шаблона

Как обычно, начнём с создания нового проекта из шаблона.

```sh
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

Когда появится запрос, задайте проекту имя, например «temperature-oled», и выберите `embassy` в качестве HAL.

## Требуемые дополнительные крейты

Нам понадобится несколько дополнительных крейтов для поддержки OLED-дисплея, форматирования текста и математических операций. Добавьте следующие записи в Cargo.toml вместе с существующими зависимостями.

```rust
ssd1306 = { version = "0.10.0", features = ["async"] }
heapless = "0.9.2"
libm = "0.2.15"
embedded-graphics = "0.8"
```

- **[`ssd1306`](https://docs.rs/ssd1306/latest/ssd1306/)**: драйвер для управления OLED-дисплеями на основе SSD1306.

- **[`heapless`](https://docs.rs/heapless/latest/heapless/)**: в окружении no_std стандартный тип String недоступен, потому что он требует выделения памяти в куче. Этот крейт предоставляет структуры фиксированного размера, размещаемые на стеке. Мы используем его, чтобы хранить отформатированный текст, например значения АЦП, сопротивление и температуру, перед отправкой на OLED.

- **[`libm`](https://crates.io/crates/libm)**: предоставляет математические функции для окружения no_std. Он нужен, чтобы вычислять натуральный логарифм при использовании уравнения B.

- **[`embedded-graphics`](https://docs.rs/embedded-graphics/latest/embedded_graphics/)**:
  Драйвер SSD1306 поддерживает разные способы вывода содержимого на дисплей.

  При использовании `into_buffered_graphics_mode` дисплей рассматривается как буфер пикселей. Текст и фигуры сначала рисуются во внутренний кадровый буфер в памяти с помощью API embedded-graphics, а затем весь буфер отправляется на OLED. Этот режим требует крейт embedded-graphics.

  При использовании `into_terminal_mode` драйвер предоставляет простой текстовый интерфейс. Вы выводите символы прямо на дисплей, не рисуя пиксели или фигуры самостоятельно. В этом случае embedded-graphics не нужен.

## Дополнительные импорты

```rust
// Форматирование текста без выделения памяти в куче
use core::fmt::Write;
use heapless::String;

// Для OLED-дисплея
use ssd1306::{I2CDisplayInterface, Ssd1306Async, prelude::*};

// Для АЦП
use embassy_rp::adc::{Adc, Channel, Config as AdcConfig};
use embassy_rp::gpio::Pull;

// Привязка прерываний
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::I2C0;
use embassy_rp::{adc, i2c};

// I2C
use embassy_rp::i2c::{Config as I2cConfig, I2c};

// Embedded Graphics
use embedded_graphics::{
    mono_font::{MonoTextStyle, iso_8859_13::FONT_7X13_BOLD},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
```

## Обработчик прерываний

В этом проекте мы используем периферию и АЦП, и I2C. Каждая из них генерирует прерывания, и Embassy требует, чтобы эти прерывания были явно привязаны на этапе компиляции.

```rust
bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => adc::InterruptHandler;
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});
```

ADC_IRQ_FIFO — прерывание, которое генерирует АЦП, когда в его FIFO появляются данные. Оно необходимо для работы АЦП в Embassy. I2C0_IRQ — прерывание периферии I2C0. Оно нужно для асинхронного обмена по I2C с OLED-дисплеем.

## Константы термистора

Определим несколько констант, которые описывают термистор и работу АЦП.

```rust
const ADC_LEVELS: f64 = 4096.0;

const B_VALUE: f64 = 3950.0;
const REF_RES: f64 = 10_000.0; // Опорное сопротивление в омах (10 кОм)
const REF_TEMP: f64 = 25.0; // Опорная температура 25 °C
```

Термистор, который мы использовали, имеет сопротивление 10 кОм при 25 °C и бета-значение 3950. У Pico 12-битный АЦП, поэтому есть 4096 возможных уровней АЦП.

## Вспомогательные функции

Определим несколько вспомогательных функций.

Эта функция, которая переводит значения АЦП в сопротивление, использует уравнение делителя напряжения. Мы уже рассмотрели эту формулу раньше, поэтому здесь просто переиспользуем её.

```rust
// Эту формулу мы уже разбирали в главе об АЦП
fn adc_to_resistance(adc_value: u16, r2_res: f64) -> f64 {
    let adc = adc_value as f64;
    ((ADC_LEVELS / adc) - 1.0) * r2_res
}
```

Эта функция, которая переводит сопротивление в температуру, применяет уравнение B. Поскольку мы работаем в окружении no_std, для вычисления натурального логарифма используем крейт libm.

```rust
// Уравнение B для перевода сопротивления в температуру
fn calculate_temperature(current_res: f64, ref_res: f64, ref_temp: f64, b_val: f64) -> f64 {
    let ln_value = libm::log(current_res / ref_res); // Используем libm для `no_std`
    let inv_t = (1.0 / ref_temp) + ((1.0 / b_val) * ln_value);
    1.0 / inv_t
}
```

Также определим небольшие вспомогательные функции для перевода между кельвинами и градусами Цельсия.

```rust
fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}
```

## Настройка дисплея

Настроим OLED на работу по I2C: SDA — на GPIO 16, SCL — на GPIO 17. Частоту I2C установим 400 кГц.

Инициализируем дисплей SSD1306 в режиме буферизованной графики. В этом режиме все операции рисования сначала выполняются в памяти. Содержимое отправляется на OLED только тогда, когда мы выгружаем буфер. Также задаём стиль текста: размер шрифта можно изменить.

```rust
// Настройка дисплея
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

let text_style = MonoTextStyle::new(&FONT_7X13_BOLD, BinaryColor::On);
```

## Настройка АЦП

Настроим канал АЦП, подключённый к выводу термистора. Затем инициализируем периферию АЦП, используя привязки прерываний, определённые ранее.

```rust
// Настройка АЦП для термистора
let mut adc_pin = Channel::new_pin(p.PIN_28, Pull::None);
let mut adc = Adc::new(p.ADC, Irqs, AdcConfig::default());
```

### Строка heapless

Создадим строку heapless фиксированной ёмкостью 64 символа. Эта строка живёт на стеке и переиспользуется на каждой итерации цикла. Мы используем её, чтобы хранить отформатированные значения температуры, показания АЦП и сопротивления перед тем, как выводить их на OLED.

```rust
let mut buff: String<64> = String::new();
```

### Переводим опорную температуру в кельвины

Опорную температуру термистора задаём 25 °C. Поскольку уравнение B требует температуру в кельвинах, мы переводим это значение один раз при инициализации. Так мы не повторяем преобразование внутри цикла.

```rust
let ref_temp = celsius_to_kelvin(REF_TEMP);
```

## Главный цикл

На каждой итерации цикла мы считываем значение термистора через АЦП, переводим показание в температуру, форматируем результат как текст и обновляем OLED.

Сначала очищаем и строковый буфер, и буфер дисплея.

```rust
buff.clear();
display
    .clear(BinaryColor::Off)
    .expect("failed to clear the display");
```

### Читаем АЦП

Затем считываем значение АЦП с вывода термистора.

```rust
let adc_value = adc
        .read(&mut adc_pin)
        .await
        .expect("failed to read adc value");
```

### Переводим значение АЦП в температуру

Переводим показание АЦП в сопротивление по уравнению делителя напряжения. Затем переводим сопротивление в температуру по уравнению B, которое даёт температуру в кельвинах. Наконец, переводим значение в градусы Цельсия.

```rust
let current_res = adc_to_resistance(adc_value, REF_RES);

let temperature_kelvin = calculate_temperature(current_res, REF_RES, ref_temp, B_VALUE);
let temperature_celsius = kelvin_to_celsius(temperature_kelvin);
```

### Форматируем текст вывода

Форматируем температуру, значение АЦП и сопротивление в строку heapless.

```rust
writeln!(buff, "Temp: {:.2} °C", temperature_celsius)
    .expect("failed to format temperature");

writeln!(buff, "ADC: {}", adc_value).expect("failed to format ADC value");

writeln!(buff, "R: {:.2}", current_res).expect("failed to format Resistance");
```

### Обновляем OLED

Выводим отформатированный текст в буфер дисплея, выгружаем буфер, чтобы обновить OLED, и ждём перед следующей итерацией.

```rust
Text::new(&buff, Point::new(5, 20), text_style)
            .draw(&mut display)
            .expect("Failed to write the text");

display.flush().await.expect("failed to send to display");

Timer::after_secs(2).await;

```

## Запуск

Когда вы прошьёте прошивку, вы должны увидеть температуру вместе со значениями сопротивления и АЦП. Можно переместить установку в другую комнату, понаблюдать, как меняются показания днём и ночью, или вынести её на улицу с внешним источником питания, чтобы увидеть, как температура реагирует в разных условиях.

## Полный код

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::{self as hal};
use embassy_time::Timer;

//Обработчик паники
use panic_probe as _;
// Логирование через defmt
use defmt_rtt as _;

// Форматирование текста без выделения памяти в куче
use core::fmt::Write;
use heapless::String;

// Для OLED-дисплея
use ssd1306::{I2CDisplayInterface, Ssd1306Async, prelude::*};

// Для АЦП
use embassy_rp::adc::{Adc, Channel, Config as AdcConfig};
use embassy_rp::gpio::Pull;

// Привязка прерываний
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::I2C0;
use embassy_rp::{adc, i2c};

// I2C
use embassy_rp::i2c::{Config as I2cConfig, I2c};

// Embedded Graphics
use embedded_graphics::{
    mono_font::{MonoTextStyle, iso_8859_13::FONT_7X13_BOLD},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};

/// Сообщаем Boot ROM о нашем приложении
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => adc::InterruptHandler;
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});

const ADC_LEVELS: f64 = 4096.0;

const B_VALUE: f64 = 3950.0;
const REF_RES: f64 = 10_000.0; // Опорное сопротивление в омах (10 кОм)
const REF_TEMP: f64 = 25.0; // Опорная температура 25 °C

// Эту формулу мы уже разбирали в главе об АЦП
fn adc_to_resistance(adc_value: u16, r2_res: f64) -> f64 {
    let adc = adc_value as f64;
    ((ADC_LEVELS / adc) - 1.0) * r2_res
}

// Уравнение B для перевода сопротивления в температуру
fn calculate_temperature(current_res: f64, ref_res: f64, ref_temp: f64, b_val: f64) -> f64 {
    let ln_value = libm::log(current_res / ref_res); // Используем libm для `no_std`
    let inv_t = (1.0 / ref_temp) + ((1.0 / b_val) * ln_value);
    1.0 / inv_t
}

fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Настройка дисплея
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

    let text_style = MonoTextStyle::new(&FONT_7X13_BOLD, BinaryColor::On);

    // Настройка АЦП для термистора
    let mut adc_pin = Channel::new_pin(p.PIN_28, Pull::None);
    let mut adc = Adc::new(p.ADC, Irqs, AdcConfig::default());

    let mut buff: String<64> = String::new();
    let ref_temp = celsius_to_kelvin(REF_TEMP);

    loop {
        buff.clear();
        display
            .clear(BinaryColor::Off)
            .expect("failed to clear the display");

        let adc_value = adc
            .read(&mut adc_pin)
            .await
            .expect("failed to read adc value");

        let current_res = adc_to_resistance(adc_value, REF_RES);

        let temperature_kelvin = calculate_temperature(current_res, REF_RES, ref_temp, B_VALUE);
        let temperature_celsius = kelvin_to_celsius(temperature_kelvin);

        writeln!(buff, "Temp: {:.2} °C", temperature_celsius)
            .expect("failed to format temperature");

        writeln!(buff, "ADC: {}", adc_value).expect("failed to format ADC value");

        writeln!(buff, "R: {:.2}", current_res).expect("failed to format Resistance");

        Text::new(&buff, Point::new(5, 20), text_style)
            .draw(&mut display)
            .expect("Failed to write the text");

        display.flush().await.expect("failed to send to display");

        Timer::after_secs(2).await;
    }
}

// Метаданные программы для `picotool info`.
// Это не обязательно, но рекомендуется добавить эти минимальные записи.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"temperature-oled"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// Конец файла
```

## Клонируйте существующий проект

Можно клонировать проект, который я создал, и перейти в папку `temperature-oled`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/temperature-oled/
```
