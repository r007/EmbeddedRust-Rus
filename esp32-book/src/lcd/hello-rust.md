{{#title Как вывести текст на LCD1602 с ESP32 и Rust}}

# «Hello, Rust!» на ЖК-дисплее

В этой программе мы просто выведем текст «Hello, Rust!» на ЖК-дисплей. 

## Драйверы HD44780
Изучая вопрос, я нашёл много крейтов Rust для управления ЖК-дисплеем, но выделялись два, которые хорошо работают. В этой программе мы начнём с использования крейта `hd44780-driver`.
- [hd44780-driver](https://crates.io/crates/hd44780-driver) 
- [liquid_crystal](https://crates.io/crates/liquid_crystal) 


## Создание проекта с помощью esp-generate
Мы включим поддержку async (Embassy) для этого проекта. Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 lcd-hello
```

Откроется экран с вопросом о выборе опций. 

- Сначала выберите опцию «Enable unstable HAL features.». Только после этого вы сможете выбрать поддержку фреймворка Embassy.
- Выберите опцию «Adds embassy framework support.»

Сохраните настройки, нажав на клавиатуре «s».

## Обновляем Cargo.toml

Мы будем использовать крейт hd44780-driver и возьмём самую свежую версию прямо из репозитория GitHub, указав нужный коммит Git с помощью атрибута `rev`. Чтобы включить поддержку async, можно указать feature `embedded-hal-async`.


```toml
# hd44780-driver = "0.4.0"
hd44780-driver = { git = "https://github.com/JohnDoneth/hd44780-driver", rev = "9009f2c24771ba0a20f8f7534471c9869188f76c", features = [
    "embedded-hal-async",
] }
```

## Импорты

Сначала добавим необходимые импорты:

```rust
// I2C
use esp_hal::i2c::master::Config as I2cConfig; // для удобства импортируем под псевдонимом
use esp_hal::i2c::master::I2c;
use esp_hal::time::Rate;

// Драйвер HD44780
use hd44780_driver::HD44780;
use hd44780_driver::memory_map::MemoryMap1602;
use hd44780_driver::setup::DisplayOptionsI2C;
```

### Инициализация I2C

Мы настроили интерфейс I2C для соединения ESP32 с ЖК-модулем. Шину I2C мы запускаем на частоте 400 кГц. Вывод GPIO18 назначен на SCL (Serial Clock Line, линия тактирования), а GPIO23 — на SDA (Serial Data Line, линия данных). Также мы включили асинхронную работу.

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

### Инициализация дисплея

Нужно указать адрес I2C ЖК-модуля. Обычно он приведён в спецификации модуля. Самый распространённый — 0x27. Лучше проверить его по даташиту или сканированием I2C-устройств.

```rust
let i2c_address = 0x27;

let Ok(mut lcd) = HD44780::new(
    DisplayOptionsI2C::new(MemoryMap1602::new()).with_i2c_bus(i2c_bus, i2c_address),
    &mut embassy_time::Delay,
) else {
    panic!("failed to initialize display");
};
```


### Подготовка дисплея
Сбросим дисплей в состояние по умолчанию, переведём курсор в начало и очистим символы, которые есть на дисплее.

```rust
// Сбрасываем сдвиг дисплея и устанавливаем курсор в позицию 0
lcd.reset(&mut embassy_time::Delay).unwrap();

// Очищаем имеющиеся символы
lcd.clear(&mut embassy_time::Delay).unwrap();
```

### Вывод текста на ЖК-дисплей
Выведем два текста на обеих строках ЖК-модуля.

```rust

// Выводим следующую строку
lcd.write_str("impl Rust", &mut embassy_time::Delay)
    .unwrap();

// Перемещаем курсор на вторую строку
lcd.set_cursor_xy((0, 1), &mut embassy_time::Delay).unwrap();

// Выводим строку на второй строке
lcd.write_str("Hello, Ferris!", &mut embassy_time::Delay)
    .unwrap();
```

## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `lcd-hello`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/lcd-hello/
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

// Драйвер HD44780
use hd44780_driver::HD44780;
use hd44780_driver::memory_map::MemoryMap1602;
use hd44780_driver::setup::DisplayOptionsI2C;

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

    let i2c_address = 0x27;

    let Ok(mut lcd) = HD44780::new(
        DisplayOptionsI2C::new(MemoryMap1602::new()).with_i2c_bus(i2c_bus, i2c_address),
        &mut embassy_time::Delay,
    ) else {
        panic!("failed to initialize display");
    };

    // Сбрасываем сдвиг дисплея и устанавливаем курсор в позицию 0
    lcd.reset(&mut embassy_time::Delay).unwrap();

    // Очищаем имеющиеся символы
    lcd.clear(&mut embassy_time::Delay).unwrap();

    // Выводим следующую строку
    lcd.write_str("impl Rust", &mut embassy_time::Delay)
        .unwrap();

    // Перемещаем курсор на вторую строку
    lcd.set_cursor_xy((0, 1), &mut embassy_time::Delay).unwrap();

    // Выводим строку на второй строке
    lcd.write_str("Hello, Ferris!", &mut embassy_time::Delay)
        .unwrap();

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }
}
```
