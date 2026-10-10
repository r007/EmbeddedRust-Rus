{{#title Писк активной пищалкой на ESP32 с embedded Rust}}

# Писк активной пищалкой на ESP32 с Rust

Поскольку вы уже знаете, что активная пищалка проста в использовании, достаточно подать на неё питание, и она запищит. В этом упражнении мы заставим её пищать, написав совсем немного кода.


### Аппаратные требования
- **Активная пищалка**
- **Провода-перемычки «female-to-male»** или **«male-to-male»** (в зависимости от вашей схемы)

### Создание проекта с помощью esp-generate

Этот шаг вы уже выполнили в разделе «Быстрый старт».

Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 active-buzzer
```

Откроется экран с вопросом о выборе опций. Сейчас ничего выбирать не нужно. Просто сохраните настройки, нажав на клавиатуре «s».

## Код

Мы настроим GPIO 33 как выходной вывод с начальным состоянием Low. К этому выводу подключён положительный контакт пищалки.

```rust
let mut buzzer = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());
```

Логика простая: в цикле переводим вывод в High на 500 миллисекунд, затем в Low ещё на 500 миллисекунд. Так пищалка издаёт прерывистый писк.

```rust
   loop {
        buzzer.set_high();
        blocking_delay(Duration::from_millis(500));
        buzzer.set_low();
        blocking_delay(Duration::from_millis(500));
    }
```

## Вспомогательная функция задержки
Она ждёт, пока не пройдёт заданное время.

```rust
fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}
```

## Клонирование существующего проекта
Вы можете клонировать (или посмотреть) созданный мной проект и перейти в папку `active-buzzer`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/active-buzzer
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
    // generator version: 1.0.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut buzzer = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());

    loop {
        buzzer.set_high();
        blocking_delay(Duration::from_millis(500));
        buzzer.set_low();
        blocking_delay(Duration::from_millis(500));
    }
}

fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}
```
