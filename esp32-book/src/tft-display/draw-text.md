{{#title Вывод текста на TFT-дисплей с ESP32 на Rust}}

# Вывод текста на TFT-дисплей с ESP32

Создадим простую программу, которая выводит текст на дисплей с помощью крейта [ili9341](https://docs.rs/ili9341/0.6.0/ili9341/). 

## Создание проекта с помощью esp-generate
Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 tft-display-hello
```

Откроется экран с вопросом о выборе опций. 

- Сначала выберите опцию «Enable unstable HAL features.»

Сохраните настройки, нажав на клавиатуре «s».

## Обновляем cargo.toml

```toml
embedded-hal-bus = { version = "0.3" }
display-interface-spi = "0.5"
ili9341 = "0.6.0"
embedded-graphics = "0.8.1"
profont = "0.7.0"
```

К этому моменту вы должны быть знакомы с крейтами embedded-hal-bus и embedded-graphics. Крейт embedded-hal предоставляет стандартизированные интерфейсы (например, SPI, I2C) для периферии микроконтроллеров, позволяя разработчикам писать переиспользуемые драйверы, которые работают на любом совместимом железе. По сути, мы будем использовать его, чтобы превратить SpiBus, предоставляемый esp-hal, в SpiDevice.

Однако в отличие от предыдущих глав, где мы использовали SpiDevice напрямую, крейт ili9341 требует ещё один уровень: он ожидает интерфейс, который реализует трейты из крейта display-interface-spi. Этот крейт определяет трейты и обёртки, которые связывают драйверы SPI-шины с драйверами дисплеев, сами обрабатывая детали, например линию данных/команд (DC).

Крейт profont мы используем, чтобы получить более крупный моноширинный шрифт для дисплея, потому что встроенные шрифты embedded-graphics слишком мелкие.

Крейт embedded-graphics мы будем использовать для отрисовки текста, фигур и изображений на дисплее.

## Необходимые импорты
```rust

// Обычные импорты
use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println as _;

// Связанное с embedded-graphics
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Baseline, Text};

// Крупный шрифт
use profont::{PROFONT_18_POINT, PROFONT_24_POINT};

// Мост между SPI ESP32 и драйвером дисплея
use display_interface_spi::SPIInterface;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::delay::Delay;
use esp_hal::spi::master::Config as SpiConfig;
use esp_hal::spi::master::Spi;
use esp_hal::spi::Mode as SpiMode;
use esp_hal::time::Rate; // Для задания частоты SPI
use ili9341::{DisplaySize240x320, Ili9341, Orientation};

// Для управления состоянием GPIO
use esp_hal::gpio::{Level, Output, OutputConfig};
```

## Настройка SPI
Инициализируем SPI-устройство для связи между ESP32 и дисплеем. Это стандартная настройка: сначала инициализируем шину SPI, а затем преобразуем её в SPI-устройство с помощью embedded-hal-bus.

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
let cs = Output::new(peripherals.GPIO15, Level::Low, OutputConfig::default());
let dc = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
let reset = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());

let spi_dev = ExclusiveDevice::new_no_delay(spi, cs).unwrap();
let interface = SPIInterface::new(spi_dev, dc);
```

На этот раз мы добавили ещё один шаг: создаём `SPIInterface` с помощью крейта display-interface-spi. Этот интерфейс объединяет SPI-устройство и вывод данных/команд (DC) в одну абстракцию. Он упрощает обмен, сам управляя тем, как команды и данные отправляются по SPI. Этот интерфейс мы передадим драйверу TFT-дисплея.

## Инициализация дисплея

Чтобы инициализировать дисплей, передаём драйверу `Ili9341` SPI-интерфейс, вывод сброса, задержку, ориентацию и размер экрана. Так задаётся всё, что нужно драйверу для работы с дисплеем.

```rust
let mut display = Ili9341::new(
        interface,
        reset,
        &mut Delay::new(),
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();
```

Ориентацию задаём как портретную, то есть дисплей считается шириной 240 пикселей и высотой 320 пикселей. Размер экрана 240 на 320 пикселей соответствует разрешению дисплея. Вместе эти настройки помогают драйверу правильно выводить содержимое с учётом формы и размера дисплея.

## Очистка дисплея

Очистим дисплей, залив фон белым цветом. Поскольку TFT — цветной дисплей, мы используем формат цвета `Rgb565`, который представляет 16-битные значения цвета (5 бит красного, 6 бит зелёного, 5 бит синего). Это первый раз, когда мы используем `Rgb565`: до сих пор мы работали только с монохромными дисплеями.

```rust
display.clear(Rgb565::WHITE).unwrap();
```

## Вывод текста

Теперь наконец выведем текст «impl Rust для ESP32» на экран. Две части текста мы выведем отдельно, используя разные размеры шрифта и цвета.

```rust

let text_style = MonoTextStyle::new(&PROFONT_24_POINT, Rgb565::RED);
Text::with_baseline("impl Rust", Point::new(50, 150), text_style, Baseline::Top)
    .draw(&mut display)
    .unwrap();

let text_style = MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::CSS_DIM_GRAY);

Text::with_baseline("for ESP32", Point::new(60, 180), text_style, Baseline::Top)
    .draw(&mut display)
    .unwrap();

```

Первую строку «impl Rust» мы рисуем красным шрифтом, на расстоянии 50 пикселей от левого края и 150 пикселей от верха экрана. Вторая строка «for ESP32» расположена чуть ниже: в 60 пикселях от левого края и в 180 пикселях от верха. Если выводите другой текст, смело меняйте координаты, чтобы добиться выравнивания и интервалов, которые выглядят лучше всего.

## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `tft-display-hello`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/tft-display-hello/
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
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println as _;

// Связанное с embedded-graphics
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Baseline, Text};

// Крупный шрифт
use profont::{PROFONT_18_POINT, PROFONT_24_POINT};

// Мост между SPI ESP32 и драйвером дисплея
use display_interface_spi::SPIInterface;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::delay::Delay;
use esp_hal::spi::Mode as SpiMode;
use esp_hal::spi::master::Config as SpiConfig;
use esp_hal::spi::master::Spi;
use esp_hal::time::Rate; // Для задания частоты SPI
use ili9341::{DisplaySize240x320, Ili9341, Orientation};

// Для управления состоянием GPIO
use esp_hal::gpio::{Level, Output, OutputConfig};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Создаёт дескриптор приложения по умолчанию, который требуется загрузчику esp-idf.
// Подробнее см.: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 1.0.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

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
    let cs = Output::new(peripherals.GPIO15, Level::Low, OutputConfig::default());
    let dc = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    let reset = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());

    let spi_dev = ExclusiveDevice::new_no_delay(spi, cs).unwrap();
    let interface = SPIInterface::new(spi_dev, dc);

    let mut display = Ili9341::new(
        interface,
        reset,
        &mut Delay::new(),
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    display.clear(Rgb565::WHITE).unwrap();

    let text_style = MonoTextStyle::new(&PROFONT_24_POINT, Rgb565::RED);
    Text::with_baseline("impl Rust", Point::new(50, 150), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    let text_style = MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::CSS_DIM_GRAY);

    Text::with_baseline("for ESP32", Point::new(60, 180), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    loop {
        info!("Hello world!");
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
```
