{{#title Управление светодиодом ESP32 на Rust с помощью esp-hal}}

# Светодиод

Следующий шаг — настроить встроенный светодиод на плате DevKit v1. Как это сделать? Для этого нужно управлять выводом GPIO.

## GPIO 
GPIO расшифровывается как General Purpose Input/Output (универсальный вход/выход). Это выводы микроконтроллера, которые могут либо считывать цифровые сигналы (вход), либо отправлять их (выход).

- Хотите считать состояние кнопки? Используйте GPIO как вход.

- Хотите включить светодиод? Используйте GPIO как выход.

- Хотите общаться с датчиком или управлять мотором? Выводы GPIO — отправная точка.

GPIO — самый базовый и самый важный способ, которым ваша программа взаимодействует с внешним миром.

## К какому выводу подключён встроенный светодиод?

Если вы используете голый чип ESP32, встроенного светодиода нет.

Но мы работаем с отладочной платой ESP32 DevKit v1. На ней есть встроенный светодиод, подключённый к GPIO2. Именно этот вывод мы будем использовать дальше.

## Инициализация светодиода

Чтобы управлять встроенным светодиодом, настроим GPIO2 как выходной вывод. 

Добавьте необходимые импорты:

```rust
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::time::{Duration, Instant};
```

Добавьте следующую строку внутри функции `main`:

```rust
let mut led = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());
```

Выражение `peripherals.GPIO2` даёт нам доступ к выводу GPIO2. Мы задаём его начальное состояние `Level::High`, которое включает светодиод. `OutputConfig::default()` использует стандартные настройки, чтобы сделать вывод выходом. Обратите внимание: переменную `led` мы объявили изменяемой (`mut`), потому что позже будем менять её состояние. Метод `toggle()` переключает состояние светодиода между высоким и низким уровнем, благодаря чему светодиод мигает.

## Главный цикл

Главный цикл непрерывно переключает состояние светодиода. Мы вызываем метод `toggle()` у выходного вывода светодиода, и тот переключается между высоким и низким уровнями. Между переключениями мы делаем задержку 500 миллисекунд, чтобы мигание было заметно.

```rust
loop {
    led.toggle();
    blocking_delay(Duration::from_millis(500));
}
```

Функция `blocking_delay` создаёт простую задержку, активно ожидая, пока не пройдёт заданное время. Она запоминает текущее время и постоянно проверяет, прошло ли нужное время:

```rust
fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}
```

## Итоговый код

Вот полный код:

```rust
#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

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
