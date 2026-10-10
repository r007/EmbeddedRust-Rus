{{#title Проект: светодиод ESP32, управляемый по Wi-Fi, на Rust}}

# Пишем код на Rust для управления светодиодом ESP32 по Wi-Fi

Веб-сервер на ESP32 может получать команды через веб-запросы, что позволяет управлять подключёнными устройствами или выполнять определённые действия. Например, с помощью браузера на компьютере или телефоне можно отправлять команды: включить или выключить светодиод, изменить скорость мотора или получить данные с датчика.

В этом разделе мы создадим простую веб-страницу, с помощью которой можно включать и выключать светодиод.

## Базовый проект
На этот раз проект с помощью esp-generate создавать не будем. Вместо этого скопируем проект webserver-base и будем работать на его основе. 

Рекомендую сначала прочитать следующие разделы, прежде чем двигаться дальше: так мы избежим ненужных повторений кода и объяснений.
- [Создание веб-сервера](../web-server/index.md) 
- [Статический IP-адрес](../static-ip.md)
 

```sh
git clone https://github.com/ImplFerris/esp32-projects
cp -r esp32-projects/webserver-base ~/YOUR_PROJECT_FOLDER/wifi-led
```

**Структура проекта**:

```
├── build.rs
├── Cargo.toml
├── rust-toolchain.toml
├── src
│   ├── bin
│   │   └── main.rs
│   ├── index.html
│   ├── led.rs
│   ├── lib.rs
│   ├── web.rs
│   └── wifi.rs
```

## Serde 
Serde — это крейт Rust для сериализации и десериализации структур данных. Мы будем использовать его для обработки JSON-данных, которыми обмениваются бэкенд и фронтенд.

Обновите Cargo.toml следующим образом:
```toml
serde = { version = "1.0.228", default-features = false, features = ["derive"] }

# Включаем функцию «json» 
picoserve = { version = "0.17.1", features = ["embassy", "json"] }
```

## Задача для светодиода 

Этот код я разместил в модуле «led.rs». 

Сначала создадим задачу Embassy, которая переключает состояние встроенного светодиода в зависимости от значения переменной LED_STATE, имеющей тип AtomicBool. «Атомарные типы обеспечивают примитивный обмен данными между потоками через общую память и являются основой для других конкурентных типов». Чтобы узнать больше об атомарных типах, обратитесь к документации стандартной библиотеки Rust по [atomics](https://doc.rust-lang.org/beta/core/sync/atomic/index.html) или к книге [Rust Atomics and Locks](https://marabos.nl/atomics/).

```rust
use core::sync::atomic::{AtomicBool, Ordering};

use embassy_time::{Duration, Timer};
use esp_hal::gpio::Output;

pub static LED_STATE: AtomicBool = AtomicBool::new(false);

#[embassy_executor::task]
pub async fn led_task(mut led: Output<'static>) {
    loop {
        if LED_STATE.load(Ordering::Relaxed) {
            led.set_high();
        } else {
            led.set_low();
        }
        Timer::after(Duration::from_millis(50)).await;
    }
}
```

В функции led_task мы принимаем вывод светодиода как аргумент и в цикле постоянно проверяем значение переменной LED_STATE. Значение читаем методом load с Ordering::Relaxed. Если значение true, включаем светодиод, иначе выключаем его.

В функции main мы запускаем led_task в фоновом режиме. Передаём вывод GPIO 2 (если хотите использовать внешний светодиод, замените его на вывод, к которому подключили светодиод), то есть встроенный светодиод, и задаём начальное состояние светодиода Low.

```rust
// Задача для светодиода
let led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
spawner.must_spawn(lib::led::led_task(led));
```
















