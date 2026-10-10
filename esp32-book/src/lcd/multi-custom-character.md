{{#title Как вывести крупные пользовательские символы ЖК-дисплея на ESP32 на Rust}}

# Несколько пользовательских символов

В этом упражнении мы выведем изображение Ferris, созданное с помощью генератора нескольких символов. Шаги создания проекта повторять не будем. Можно либо выполнить шаги из предыдущей главы, либо клонировать проект предыдущей главы и продолжить работу на его основе.

Я создал изображение Ferris с помощью 6 соседних сеток, используя генератор с предыдущей страницы. Вот код на Rust, который использует эти байтовые массивы.

<img style="display: block; margin: auto;width:400px;" alt="пользовательские символы, объединённые из нескольких сеток" src="./images/ferris-with-6-grids-on-lcd-display.png"/>

### Сгенерированные байтовые массивы для символов
```rust
const SYMBOL1: [u8; 8] = [
    0b00110, 0b01000, 0b01110, 0b01000, 0b00100, 0b00011, 0b00100, 0b01000,
];

const SYMBOL2: [u8; 8] = [
    0b00000, 0b00000, 0b00000, 0b10001, 0b10001, 0b11111, 0b00000, 0b00000,
];

const SYMBOL3: [u8; 8] = [
    0b01100, 0b00010, 0b01110, 0b00010, 0b00100, 0b11000, 0b00100, 0b00010,
];

const SYMBOL4: [u8; 8] = [
    0b01000, 0b01000, 0b00100, 0b00011, 0b00001, 0b00010, 0b00101, 0b01000,
];

const SYMBOL5: [u8; 8] = [
    0b00000, 0b00000, 0b00000, 0b11111, 0b01010, 0b10001, 0b00000, 0b00000,
];

const SYMBOL6: [u8; 8] = [
    0b00010, 0b00010, 0b00100, 0b11000, 0b10000, 0b01000, 0b10100, 0b00010,
];
```

### Объявляем их как символы
```rust
lcd.custom_char(&mut Delay, &SYMBOL1, 0);
lcd.custom_char(&mut Delay, &SYMBOL2, 1);
lcd.custom_char(&mut Delay, &SYMBOL3, 2);
lcd.custom_char(&mut Delay, &SYMBOL4, 3);
lcd.custom_char(&mut Delay, &SYMBOL5, 4);
lcd.custom_char(&mut Delay, &SYMBOL6, 5);
```

### Вывод
Выведем первые 3 сетки в первую строку, а вторую половину — во вторую строку ЖК-дисплея.

```rust
lcd.set_cursor(&mut Delay, 0, 4)
    .write(&mut Delay, CustomChar(0))
    .write(&mut Delay, CustomChar(1))
    .write(&mut Delay, CustomChar(2));

lcd.set_cursor(&mut Delay, 1, 4)
    .write(&mut Delay, CustomChar(3))
    .write(&mut Delay, CustomChar(4))
    .write(&mut Delay, CustomChar(5));
```


## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `lcd-custom-multi`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/lcd-custom-multi/
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
use embassy_time::Delay;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;

// I2C
use esp_hal::i2c::master::Config as I2cConfig; // для удобства импортируем под псевдонимом
use esp_hal::i2c::master::I2c;
use esp_hal::time::Rate;

// Драйвер HD44780
use liquid_crystal::I2C;
use liquid_crystal::LiquidCrystal;
use liquid_crystal::prelude::*;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Создаёт дескриптор приложения по умолчанию, который требуется загрузчику esp-idf.
// Подробнее см.: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const SYMBOL1: [u8; 8] = [
    0b00110, 0b01000, 0b01110, 0b01000, 0b00100, 0b00011, 0b00100, 0b01000,
];

const SYMBOL2: [u8; 8] = [
    0b00000, 0b00000, 0b00000, 0b10001, 0b10001, 0b11111, 0b00000, 0b00000,
];

const SYMBOL3: [u8; 8] = [
    0b01100, 0b00010, 0b01110, 0b00010, 0b00100, 0b11000, 0b00100, 0b00010,
];

const SYMBOL4: [u8; 8] = [
    0b01000, 0b01000, 0b00100, 0b00011, 0b00001, 0b00010, 0b00101, 0b01000,
];

const SYMBOL5: [u8; 8] = [
    0b00000, 0b00000, 0b00000, 0b11111, 0b01010, 0b10001, 0b00000, 0b00000,
];

const SYMBOL6: [u8; 8] = [
    0b00010, 0b00010, 0b00100, 0b11000, 0b10000, 0b01000, 0b10100, 0b00010,
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

    let mut i2c_interface = I2C::new(i2c_bus, 0x27);
    let mut lcd = LiquidCrystal::new(&mut i2c_interface, Bus4Bits, LCD16X2);
    lcd.begin(&mut Delay);

    // Определяем символы
    lcd.custom_char(&mut Delay, &SYMBOL1, 0);
    lcd.custom_char(&mut Delay, &SYMBOL2, 1);
    lcd.custom_char(&mut Delay, &SYMBOL3, 2);
    lcd.custom_char(&mut Delay, &SYMBOL4, 3);
    lcd.custom_char(&mut Delay, &SYMBOL5, 4);
    lcd.custom_char(&mut Delay, &SYMBOL6, 5);

    lcd.set_cursor(&mut Delay, 0, 4)
        .write(&mut Delay, CustomChar(0))
        .write(&mut Delay, CustomChar(1))
        .write(&mut Delay, CustomChar(2));

    lcd.set_cursor(&mut Delay, 1, 4)
        .write(&mut Delay, CustomChar(3))
        .write(&mut Delay, CustomChar(4))
        .write(&mut Delay, CustomChar(5));

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
```
