{{#title Создаём BLE GATT-сервер на ESP32 на Rust с Trouble}}

# Модуль BLE

В этом модуле мы определим, какие данные ESP32 отправляет и получает, настроим и запустим сервер и будем обрабатывать подключения. Этот пример взят из официального репозитория Trouble и немного изменён. Больше примеров можно найти в [репозитории](https://github.com/embassy-rs/trouble/tree/main/examples/apps).

Создадим модуль BLE, обновив файл lib.rs:

```rust
#![no_std]
pub mod ble;
```

Далее будем работать в файле модуля `ble.rs`. Сначала добавим все необходимые импорты:

```rust
use defmt::{info, warn};

use embassy_futures::join::join;
use embassy_futures::select::select;

use embassy_time::Timer;

// BLE:
use trouble_host::prelude::*;
```

Let's define the maximum number of concurrent connections and L2CAP channels for the BLE stack:

```rust
/// Максимальное число подключений
const CONNECTIONS_MAX: usize = 1;

/// Максимальное число каналов L2CAP.
const L2CAP_CHANNELS_MAX: usize = 1;
```


## GATT-сервис

Мы используем макросы gatt_server и gatt_service, чтобы определить GATT-сервер с сенсорным сервисом, содержащим две характеристики.

```rust
// Определение GATT-сервера
#[gatt_server]
struct Server {
    sensor_service: SensorService,
}

/// Сервис батареи
#[gatt_service(uuid = "a9c81b72-0f7a-4c59-b0a8-425e3bcf0a0e")]
struct SensorService {
    #[characteristic(uuid = "13c0ef83-09bd-4767-97cb-ee46224ae6db", read, notify)]
    sensor_data: u8,

    #[characteristic(uuid = "c79b2ca7-f39d-4060-8168-816fa26737b7", write, read)]
    sensor_settings: bool,
}
```

Структура Server представляет наш GATT-сервер и содержит SensorService. SensorService определяет две характеристики:

- sensor_data: значение типа "u8", которое имитирует данные датчика в этом упражнении. Оно поддерживает операции read и notify, позволяя клиентам читать текущее значение датчика и получать уведомления при его изменении. Позже мы будем обновлять это значение в цикле, чтобы показать обновления в реальном времени.  Когда освоите BLE, можно попробовать подключить настоящий датчик и отправлять данные на мобильное устройство.

- sensor_settings: логическое значение, которое поддерживает и запись, и чтение. Это позволяет клиентам настраивать параметры датчика и читать текущую конфигурацию. Так мы показываем, что значение можно изменить со стороны клиента.
