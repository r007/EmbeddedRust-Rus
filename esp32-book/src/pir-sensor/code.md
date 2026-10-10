{{#title Пишем код на Rust для обнаружения движения с PIR-датчиком и ESP32}}

# Пишем код на Rust для обнаружения движения с PIR-датчиком и ESP32

Давайте напишем простую программу, которая выводит сообщение каждый раз, когда обнаружено движение. Это поможет точнее настроить PIR-датчик и разобраться в базовых понятиях. Затем мы соберём полноценную имитацию охранной сигнализации с пищалкой и светодиодом (встроенным или внешним, на ваш выбор), чтобы сделать её интереснее.

### Создание проекта с помощью esp-generate

Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 pir-sensor
```

Откроется экран с вопросом о выборе опций.

В этом упражнении мы будем выводить сообщения в системную консоль. Для этого нужно включить функцию логирования.

Прокрутите список до пункта «Flashing, logging and debugging (espflash)» и нажмите Enter.

Выберите «Use defmt to print messages» («Использовать defmt для вывода сообщений»).

Сохраните настройки, нажав на клавиатуре «s».


## Подключение выхода датчика ко входу ESP32

Мы настроим GPIO 33 как входной вывод с начальным состоянием pull-down. Этот вывод подключён к выходу PIR-датчика, который переходит в HIGH каждый раз, когда обнаружено движение.

```rust
let sensor_pin = Input::new(
    peripherals.GPIO33,
    InputConfig::default().with_pull(Pull::Down),
);
```

## Логика
Идея простая: мы постоянно проверяем выход датчика в цикле. Когда выход датчика переходит в HIGH, мы выводим сообщение «Motion detected» и делаем короткую задержку.

```rust
loop {
    if sensor_pin.is_high() {
        info!("Motion detected");
        blocking_delay(Duration::from_millis(100));
    }
    blocking_delay(Duration::from_millis(100));
}
```

## Клонирование существующего проекта
Вы можете клонировать (или посмотреть) созданный мной проект и перейти в папку `pir-sensor`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/pir-sensor
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
use esp_hal::gpio::{Input, InputConfig, Pull};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println as _;

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

    let sensor_pin = Input::new(
        peripherals.GPIO33,
        InputConfig::default().with_pull(Pull::Down),
    );

    loop {
        if sensor_pin.is_high() {
            info!("Motion detected");
            blocking_delay(Duration::from_millis(100));
        }
        blocking_delay(Duration::from_millis(100));
    }
}

fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}
```
