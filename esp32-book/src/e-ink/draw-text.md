{{#title Как вывести текст на e-Ink дисплей с ESP32 и Rust}}

# Вывод текста на e-Paper (e-ink) дисплей с помощью ESP32

Давайте создадим простую программу, которая выводит текст на дисплей с помощью крейта [epd-waveshare](https://docs.rs/epd-waveshare/latest/epd_waveshare/). Однако на момент написания этот крейт работал не так, как ожидалось, с 1,54-дюймовым дисплеем V2. Чтобы решить проблему, мне пришлось применить несколько патчей, поэтому пока мы будем использовать наш форк. Я ещё не отправил pull request, потому что нужны дополнительные доработки (частичные обновления работают не так, как ожидается). Тем не менее для наших упражнений форк подойдёт. Как только проблемы будут решены, эта глава будет обновлена и станет использовать последнюю исправленную версию.

## Создание проекта с помощью esp-generate
Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 e-ink-hello
```

Откроется экран с вопросом о выборе опций. 

- Сначала выберите опцию «Enable unstable HAL features.»
- Выберите опцию «Adds embassy framework support».

Сохраните настройки, нажав на клавиатуре «s».

## Обновляем cargo.toml

```toml
epd-waveshare = { features = [
  "graphics",
], git = "https://github.com/ImplFerris/epd-waveshare", branch = "1in54_v2_fix" }
embedded-hal-bus = { version = "0.3" }
embedded-graphics = "0.8.1"
```

К этому моменту вы должны быть знакомы с крейтами embedded-hal-bus и embedded-graphics. Крейт embedded-hal предоставляет стандартизированные интерфейсы (например, SPI и I2C) для периферии микроконтроллеров, позволяя разработчикам писать переиспользуемые драйверы, которые работают на любом совместимом железе. По сути, мы будем использовать его, чтобы превратить SpiBus, предоставляемый esp-hal, в SpiDevice, который требуется драйверу epd-waveshare.

Крейт embedded-graphics мы будем использовать для отрисовки текста, фигур и изображений на дисплее.

## Настройка SPI
Инициализируем SPI-устройство для связи между ESP32 и e-paper дисплеем. Это стандартная настройка: сначала инициализируем шину SPI, а затем преобразуем её в SPI-устройство с помощью embedded-hal-bus.

```rust
// Инициализируем SPI
let spi = Spi::new(
    peripherals.SPI2,
    SpiConfig::default()
        .with_frequency(Rate::from_mhz(4))
        .with_mode(SpiMode::_0),
)
.unwrap()
//CLK
.with_sck(peripherals.GPIO18)
//DIN
.with_mosi(peripherals.GPIO23);
let cs = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());
let mut spi_dev = ExclusiveDevice::new(spi, cs, Delay).unwrap();
```

## Инициализация e-paper дисплея

Теперь инициализируем e-paper дисплей, передав SPI-устройство и соответствующие выводы GPIO для управления.

```rust
// Инициализируем дисплей
let busy_in = Input::new(
    peripherals.GPIO22,
    InputConfig::default().with_pull(Pull::None),
);
let dc = Output::new(peripherals.GPIO17, Level::Low, OutputConfig::default());
let reset = Output::new(peripherals.GPIO16, Level::Low, OutputConfig::default());
let mut display = Display1in54::default();
let mut epd = Epd1in54::new(&mut spi_dev, busy_in, dc, reset, &mut Delay, None).unwrap();
```

## Очистка дисплея

Сначала очищаем внутренний буфер e-paper дисплея, затем заполняем экран белым цветом. После этого обновляем экран, чтобы изменения отобразились. Наконец, добавляем короткую задержку, чтобы дисплей успел стабилизироваться.

```rust
epd.clear_frame(&mut spi_dev, &mut Delay).unwrap();
display.clear(Color::White).unwrap();
epd.update_and_display_frame(&mut spi_dev, display.buffer(), &mut Delay)
    .unwrap();
Timer::after(Duration::from_secs(5)).await;
```

## Вывод текста

Теперь наконец выведем текст «impl Rust для ESP32» на экран в позицию (x=3, y=100). После этого обновим и перерисуем дисплей, чтобы показать текст. Также добавляем короткую задержку, чтобы изменения успели примениться.

```rust
draw_text(&mut display, "impl Rust для ESP32", 3, 100);
epd.update_and_display_frame(&mut spi_dev, display.buffer(), &mut Delay)
    .unwrap();
Timer::after(Duration::from_secs(5)).await;
```

Мы используем вспомогательную функцию `draw_text`, которая применяет API `Text` из крейта embedded_graphics, чтобы записать текст заданного размера и чёрным цветом в буфер дисплея.

```rust
fn draw_text(display: &mut Display1in54, text: &str, x: i32, y: i32) {
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(Color::Black)
        .build();

    Text::with_baseline(text, Point::new(x, y), text_style, Baseline::Top)
        .draw(display)
        .unwrap();
}
```

## Время сна

Одна из важных мер предосторожности при использовании модуля e-Ink: после работы нужно либо перевести его в спящий режим, либо полностью отключить питание. Иначе вы рискуете повредить дисплей.

```rust
epd.sleep(&mut spi_dev, &mut Delay).unwrap();
```

## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `e-ink-hello`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/e-ink-hello/
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
use embassy_time::{Delay, Duration, Timer};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;

// SPI
use esp_hal::spi;
use esp_hal::spi::master::Spi;
use esp_hal::time::Rate;

// epd
use epd_waveshare::color::Color;
use epd_waveshare::epd1in54_v2::{Display1in54, Epd1in54};
use epd_waveshare::prelude::WaveshareDisplay;

// embedded graphics
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Baseline, Text};

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

    let spi_bus = Spi::new(
        peripherals.SPI2,
        spi::master::Config::default()
            .with_frequency(Rate::from_mhz(4))
            .with_mode(spi::Mode::_0),
    )
    .unwrap()
    //CLK
    .with_sck(peripherals.GPIO18)
    //DIN
    .with_mosi(peripherals.GPIO23);

    let cs = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());
    let mut spi_dev = ExclusiveDevice::new(spi_bus, cs, Delay).unwrap();

    // Инициализируем дисплей
    let busy_in = Input::new(
        peripherals.GPIO22,
        InputConfig::default().with_pull(Pull::None),
    );
    let dc = Output::new(peripherals.GPIO17, Level::Low, OutputConfig::default());
    let reset = Output::new(peripherals.GPIO16, Level::Low, OutputConfig::default());
    let mut display = Display1in54::default();
    let mut epd = Epd1in54::new(&mut spi_dev, busy_in, dc, reset, &mut Delay, None).unwrap();

    // Очищаем имеющееся изображение
    epd.clear_frame(&mut spi_dev, &mut Delay).unwrap();
    display.clear(Color::White).unwrap();
    epd.update_and_display_frame(&mut spi_dev, display.buffer(), &mut Delay)
        .unwrap();
    Timer::after(Duration::from_secs(5)).await;

    draw_text(&mut display, "impl Rust для ESP32", 3, 100);
    epd.update_and_display_frame(&mut spi_dev, display.buffer(), &mut Delay)
        .unwrap();
    Timer::after(Duration::from_secs(5)).await;

    epd.sleep(&mut spi_dev, &mut Delay).unwrap();

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }
}

fn draw_text(display: &mut Display1in54, text: &str, x: i32, y: i32) {
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(Color::Black)
        .build();

    Text::with_baseline(text, Point::new(x, y), text_style, Baseline::Top)
        .draw(display)
        .unwrap();
}
```
