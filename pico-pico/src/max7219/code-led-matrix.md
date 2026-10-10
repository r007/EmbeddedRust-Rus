{{#title Код на embedded Rust для управления светодиодной матрицей MAX7219 на Raspberry Pi Pico 2}}

# Код на embedded Rust для управления светодиодной матрицей MAX7219 на Raspberry Pi Pico 2

Перейдём к части с кодом. В этой программе мы нарисуем квадрат на светодиодной матрице 8x8 с помощью embedded-graphics.

> [!Tip]
> Рисование квадрата — лишь пример, но, освоив основы, можно делать гораздо больше. Можно прокручивать текст, выводить иконки или смайлики и даже анимировать объекты. Примеры можно посмотреть здесь: [https://github.com/ImplFerris/max7219-examples](https://github.com/ImplFerris/max7219-examples). Также можно попробовать собрать простые часы из последовательно соединённых матриц.

Как обычно, создайте новый проект с помощью предоставленного шаблона.

```rust
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

Когда появится запрос, задайте проекту имя, например «hello-max7219», и выберите embassy в качестве HAL.

## Дополнительные крейты

Добавим следующие крейты вместе с уже существующими зависимостями:

```toml
max7219-display = { version = "0.1.5", features = ["led-matrix", "graphics"] }
embedded-graphics = "0.8.0"
embedded-hal-bus = "0.3.0"
```

Крейт max7219-display предоставляет драйвер, который используется в этой программе. Драйвер поддерживает разные виды модулей дисплеев, например 7-сегментные дисплеи и светодиодные матрицы. В нашем случае используется светодиодная матрица 8x8, поэтому мы включаем функцию led-matrix. Поскольку мы хотим рисовать фигуры, а не управлять отдельными светодиодами вручную, мы также включаем функцию graphics, которая реализует для драйвера трейты embedded-graphics.

Крейт embedded-graphics используется для создания фигур и текста в структурированном виде. Он предоставляет примитивы вроде прямоугольников, точек и стилей, что значительно упрощает рисование на светодиодной матрице.

### embedded-hal-bus

Крейт embedded-hal определяет общие трейты для периферии вроде SPI, чтобы драйверы могли работать на разных микроконтроллерах. Чтобы понять, зачем нужен embedded-hal-bus, полезно посмотреть, как организованы эти трейты.

Embedded HAL разделяет SPI на два отдельных понятия: **шину SPI** (представляет общие линии тактирования и данных) и **устройство SPI** (представляет отдельную периферию со своей линией выбора микросхемы, chip select). HAL микроконтроллеров обычно предоставляют реализацию шины SPI, но драйверы устройств пишутся для работы с устройствами SPI.

Крейт embedded-hal-bus закрывает этот разрыв. Он предоставляет адаптеры, которые оборачивают шину SPI и управляют выводом chip select, создавая интерфейс устройства SPI, который ожидают драйверы. В этой главе мы используем `ExclusiveDevice` — самый простой адаптер. Он рассчитан на сценарии, когда шиной SPI пользуется только одно устройство: он автоматически активирует chip select в начале каждой транзакции и деактивирует его по завершении.

## Дополнительные импорты

Теперь добавим импорты, необходимые для SPI, драйвера дисплея MAX7219 и рисования с помощью embedded-graphics.

```rust
// Для MAX7219
use embedded_hal_bus::spi::ExclusiveDevice;
use max7219_display::led_matrix::display::SingleMatrix;

// Для рисования фигур
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};

// Для SPI
use embassy_rp::spi::{Config as SpiConfig, Spi};

// Для вывода CS
use embassy_rp::gpio::{Level, Output};
```

## Настройка SPI и chip select

Начнём с создания вывода chip select. MAX7219 использует активно-низкий сигнал chip select, поэтому инициализируем вывод в высоком состоянии. Так устройство остаётся неактивным, пока не начнётся передача по SPI.

```rust
let cs_pin = Output::new(p.PIN_13, Level::High);
```

Далее назначаем выводы тактирования SPI и MOSI. Поскольку MAX7219 работает только на запись, нужны только эти два сигнала. В этой схеме вывод MISO не используется.

```rust
let clk = p.PIN_14;
let mosi = p.PIN_15;
```

Выбрав выводы, создаём периферию SPI. Мы используем блокирующий экземпляр SPI только на передачу, так как вся связь идёт от Pico к дисплею.

```rust
let spi_bus = Spi::new_blocking_txonly(p.SPI1, clk, mosi, SpiConfig::default());
```

На этом этапе у нас есть шина SPI, но она ещё не управляет chip select. Чтобы это обеспечить, оборачиваем шину в `ExclusiveDevice`. Это превращает шину SPI в устройство SPI, автоматически управляя выводом chip select для каждой передачи.

```rust
let spi_dev =
        ExclusiveDevice::new_no_delay(spi_bus, cs_pin).expect("Failed to get exclusive device");
```

После этого шага устройство SPI готово к использованию драйвером дисплея MAX7219.

## Создание экземпляра дисплея

Когда устройство SPI готово, создаём экземпляр светодиодной матрицы. Здесь мы работаем с одной матрицей 8x8, а не с последовательным соединением. Драйвер дисплея берёт устройство SPI во владение и использует его для отправки команд и данных о пикселях в MAX7219.

```rust
// Создаём экземпляр дисплея для одной светодиодной матрицы 8x8 (без последовательного соединения)
let mut display = SingleMatrix::from_spi(spi_dev).expect("display count 1 should not panic");
```

## Настройка яркости дисплея

После создания экземпляра дисплея задаём уровень яркости. MAX7219 поддерживает несколько уровней интенсивности, и здесь мы настраиваем единственное устройство с индексом 0. Низкого значения интенсивности обычно достаточно, и оно не даёт излишней яркости.

```rust
// Задаём яркость (уровень интенсивности) единственного устройства с индексом 0
    display
        .driver()
        .set_intensity(0, 1)
        .expect("failed to set intensity");
```

## Рисуем прямоугольник

Теперь нарисуем простую фигуру с помощью embedded-graphics. Вместо заливки всей области мы рисуем полый прямоугольник, так что включаются только пиксели по границе. Сначала я попробовал залитый прямоугольник, но он выглядел плохо, поэтому эту версию я закомментировал.

Сначала определяем стиль рисования. Этот стиль включает пиксели и задаёт толщину линии в один пиксель, что хорошо подходит для дисплея 8x8.

```rust
// ---- Рисуем прямоугольник ----
// let rect = Rectangle::new(Point::new(1, 1), Size::new(6, 6)).into_styled(
//     embedded_graphics::primitives::PrimitiveStyle::with_fill(BinaryColor::On),
// );
let hollow_rect_style = PrimitiveStyleBuilder::new()
    .stroke_color(BinaryColor::On) // Рисуем только границу
    .stroke_width(1) // Толщина границы — 1 пиксель
    .build();
```

Далее определяем сам прямоугольник. Левый верхний угол находится в точке x = 1 и y = 1, а прямоугольник занимает 6 пикселей по ширине и высоте. Это оставляет отступ в один пиксель от краёв, поэтому граница не касается крайних светодиодов.

```rust
let rect = Rectangle::new(Point::new(1, 1), Size::new(6, 6)).into_styled(hollow_rect_style);
```

Когда мы рисуем прямоугольник, он записывается в буфер дисплея. На этом этапе на аппаратуру ещё ничего не отправлено.

```rust
rect.draw(&mut display).expect("failed to draw the shape");
```

Чтобы фактически обновить светодиодную матрицу, выгружаем кадровый буфер (framebuffer). Это отправляет данные пикселей по SPI в MAX7219.

```rust
display.flush().expect("failed to send to the device");
```

## Полный код

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_time::Timer;

// Для MAX7219
use embedded_hal_bus::spi::ExclusiveDevice;
use max7219_display::led_matrix::display::SingleMatrix;

// Для рисования фигур
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};

// Для SPI
use embassy_rp::spi::{Config as SpiConfig, Spi};

// Для вывода CS
use embassy_rp::gpio::{Level, Output};

//Обработчик паники
use panic_probe as _;
// Логирование через defmt
use defmt_rtt as _;

/// Сообщаем Boot ROM о нашем приложении
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let cs_pin = Output::new(p.PIN_13, Level::High);

    let clk = p.PIN_14;
    let mosi = p.PIN_15;

    let spi_bus = Spi::new_blocking_txonly(p.SPI1, clk, mosi, SpiConfig::default());
    let spi_dev =
        ExclusiveDevice::new_no_delay(spi_bus, cs_pin).expect("Failed to get exclusive device");

    // Создаём экземпляр дисплея для одной светодиодной матрицы 8x8 (без последовательного соединения)
    let mut display = SingleMatrix::from_spi(spi_dev).expect("display count 1 should not panic");

    // Задаём яркость (уровень интенсивности) единственного устройства с индексом 0
    display
        .driver()
        .set_intensity(0, 1)
        .expect("failed to set intensity");

    // ---- Рисуем прямоугольник ----
    // let rect = Rectangle::new(Point::new(1, 1), Size::new(6, 6)).into_styled(
    //     embedded_graphics::primitives::PrimitiveStyle::with_fill(BinaryColor::On),
    // );
    let hollow_rect_style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On) // Рисуем только границу
        .stroke_width(1) // Толщина границы — 1 пиксель
        .build();
    let rect = Rectangle::new(Point::new(1, 1), Size::new(6, 6)).into_styled(hollow_rect_style);
    rect.draw(&mut display).expect("failed to draw the shape");

    display.flush().expect("failed to send to the device");

    loop {
        Timer::after_millis(100).await;
    }
}

// Метаданные программы для `picotool info`.
// Это не обязательно, но рекомендуется добавить эти минимальные записи.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"hello-max7219"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// Конец файла
```

## Клонируйте существующий проект

Можно клонировать проект, который я создал, и перейти в папку `hello-max7219`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/max7219/hello-max7219
```
