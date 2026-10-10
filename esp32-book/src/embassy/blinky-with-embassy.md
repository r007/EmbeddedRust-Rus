{{#title Асинхронное мигание светодиодом на ESP32 с Embassy}}

# Мигание светодиодом с ESP RTOS (Embassy) на ESP32 на Rust

Давайте заново настроим проект мигания, но уже с поддержкой Embassy.

### Создание проекта

Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 blinky-embassy
```

На этот раз нужно выбрать «Add Embassy framework support» («Добавить поддержку фреймворка Embassy»). Поскольку для этого требуется нестабильная функция, сначала выберите «Enable unstable HAL features» («Включить нестабильные возможности HAL»), а затем — поддержку Embassy. Наконец, нажмите «s», чтобы сохранить сгенерированный проект с поддержкой Embassy.

## Ключевые моменты

Если присмотреться, функция `main` теперь помечена как `async`, и в коде есть и другие изменения. Однако основная логика мигания светодиодом осталась прежней.

Ключевое дополнение — вот эта часть:

```rust
let timg0 = TimerGroup::new(peripherals.TIMG0);
esp_rtos::start(timg0.timer0);
```
Она настраивает таймер, который нужен Embassy для обработки асинхронных задач, например задержек. Мы создаём группу таймеров на аппаратном таймере TIMG0, затем передаём один из его таймеров в `esp_rtos::start`, чтобы Embassy использовал его для операций, связанных со временем.


## Полный код

```rust
#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::timer::timg::TimerGroup;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Создаёт дескриптор приложения по умолчанию, который требуется загрузчику esp-idf.
// Подробнее см.: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    // generator version: 1.0.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    let mut led = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());

    loop {
        led.toggle();
        Timer::after(Duration::from_secs(1)).await;
    }
}
```

 
## Клонирование существующего проекта

Вы можете клонировать (или посмотреть) созданный мной проект и перейти в папку `async-projects/blinky-embassy`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/async-projects/blinky-embassy
```
