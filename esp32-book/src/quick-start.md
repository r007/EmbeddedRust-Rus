{{#title Быстрый старт с ESP32 и Rust: мигаем светодиодом}}

# Быстрый старт — Hello Embedded World!

Прежде чем погружаться в теорию и разбираться, как всё устроено, давайте сразу перейдём к делу. Используйте этот простой код, чтобы включить встроенный светодиод ESP32 DevKit.

### Создание проекта

Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 esp32-quick
```

Откроется экран с вопросом о выборе опций. Сейчас ничего выбирать не нужно. Просто сохраните настройки, нажав на клавиатуре «s».

> Также можно выбрать опцию интеграции с редактором и затем указать конкретный редактор, которым вы пользуетесь.

Затем перейдите в папку проекта:
```sh
cd esp32-quick
```

## Полный код

Откройте файл `src/bin/main.rs`. Внутри есть простая программа «Hello, World». Мы заменим её другим вариантом «Hello, World» для встраиваемого мира: заставим светодиод на плате мигать. Просто скопируйте код ниже и вставьте его в файл main.rs.

Пока не беспокойтесь о коде: всё объясним в следующей главе. Сейчас нам важно увидеть что-то интересное!

```rust
#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Создаёт дескриптор приложения по умолчанию, который требуется загрузчику esp-idf.
// Подробнее см.: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut led = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());

    loop {
        led.toggle();

        blocking_delay(Duration::from_millis(500));
    }

}

fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}

```

## Клонирование готового проекта
Если столкнётесь с проблемами, можете клонировать (или посмотреть) проект, который я создал.

```sh
git clone https://github.com/ImplFerris/esp32-quick
cd esp32-quick/
```

## Прошивка — «Беги, Rust, беги!»
Осталось загрузить код на устройство и посмотреть, как всё работает! Встроенный светодиод должен начать мигать.

Выполните следующую команду из папки проекта:
```rust
cargo run
```

Чтобы запустить в release-режиме:
```rust
cargo run --release
```
