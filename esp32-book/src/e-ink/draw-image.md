{{#title Учебное руководство по ESP32 e-Ink с embedded-graphics на Rust}}

# Вывод изображения или фигуры на e-Paper (e-ink) дисплей с ESP32

Давайте поднимем уровень ещё выше: выведем на e-Paper дисплей изображение и фигуру. Для этого можно использовать крейт embedded-graphics и, по желанию, крейт tinybmp.

Крейт tinybmp мы уже рассматривали в разделе про OLED. Он позволяет загрузить BMP-файл и вывести его на экран. Кроме того, тот же результат можно получить, используя сырой байтовый массив изображения.

Крейт требует, чтобы изображение было в формате BMP. Если ваше изображение в другом формате, его нужно конвертировать в BMP. Например, в Linux можно использовать такую команду, чтобы превратить PNG в монохромный BMP:

```sh
convert ferris.png -monochrome ferris.bmp
```

Я подготовил файл Ferris в формате BMP, который можно использовать для этого упражнения. Скачать его можно [здесь](./images/ferris.bmp).

<img style="display: block; margin: auto;" alt="файл ferris.bmp" src="./images/ferris.bmp"/>


## Создание проекта с помощью esp-generate
Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 e-ink-image
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
embedded-hal-bus = { version = "0.1" }
embedded-graphics = "0.8.1"
tinybmp = "0.6.0"
```

Подробности остальных крейтов, а также базовую настройку SPI и кода дисплея мы уже разобрали, поэтому повторяться не будем. Сразу перейдём к выводу изображения и фигуры после очистки экрана.

## Чёрный фон
Вместо белого фона мы зальём фон чёрным цветом. Я выбрал чёрный, потому что у изображения, с которым мы работаем, тёмный фон, а главный объект состоит из белого цвета. Так картинка будет выглядеть лучше.

```rust
// Очищаем имеющееся изображение
epd.clear_frame(&mut spi_dev, &mut Delay).unwrap();
display.clear(Color::Black).unwrap();
epd.update_and_display_frame(&mut spi_dev, display.buffer(), &mut Delay)
    .unwrap();
Timer::after(Duration::from_secs(5)).await;
```

## Вывод изображения
Поместите файл ferris.bmp в папку src. Код довольно простой: загрузите изображение как байты и передайте их в функцию `from_slice` у `Bmp`. Затем можно использовать результат с `Image`.

```rust
let bmp_data = include_bytes!("../ferris.bmp");
let bmp = Bmp::from_slice(bmp_data).unwrap();
let image = Image::new(&bmp, Point::new(25, 60));
image.draw(&mut display).unwrap();

epd.update_and_display_frame(&mut spi_dev, display.buffer(), &mut Delay)
    .unwrap();
Timer::after(Duration::from_secs(5)).await;
```

## Вывод фигуры
Нарисуем окружность на дисплее.
```rust
// Выводим окружность
Circle::new(Point::new(80, 10), 40)
    .into_styled(PrimitiveStyle::with_stroke(Color::White, 2))
    .draw(&mut display)
    .ok();

epd.update_and_display_frame(&mut spi_dev, display.buffer(), &mut Delay)
    .unwrap();
Timer::after(Duration::from_secs(5)).await;
```

## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `e-ink-image`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/e-ink-image/
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

// embedded-graphics
use embedded_graphics::image::Image;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, PrimitiveStyle};

// Bmp
use tinybmp::Bmp;

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
    display.clear(Color::Black).unwrap();
    epd.update_and_display_frame(&mut spi_dev, display.buffer(), &mut Delay)
        .unwrap();
    Timer::after(Duration::from_secs(5)).await;

    // Вывод изображения
    let bmp_data = include_bytes!("../ferris.bmp");
    let bmp = Bmp::from_slice(bmp_data).unwrap();
    let image = Image::new(&bmp, Point::new(25, 60));
    image.draw(&mut display).unwrap();
    epd.update_and_display_frame(&mut spi_dev, display.buffer(), &mut Delay)
        .unwrap();
    Timer::after(Duration::from_secs(5)).await;

    // Выводим окружность
    Circle::new(Point::new(80, 10), 40)
        .into_styled(PrimitiveStyle::with_stroke(Color::White, 2))
        .draw(&mut display)
        .ok();
    epd.update_and_display_frame(&mut spi_dev, display.buffer(), &mut Delay)
        .unwrap();
    Timer::after(Duration::from_secs(5)).await;

    epd.sleep(&mut spi_dev, &mut Delay).unwrap();

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(60)).await;
    }
}
```
