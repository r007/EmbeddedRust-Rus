{{#title Обработка GATT-событий BLE на ESP32 на Rust}}

# GATT-события

Задача GATT-событий обрабатывает входящие запросы от подключённого central-устройства. Она выполняет операции чтения и записи характеристик и отвечает на них, пока соединение не закроется.

```rust
/// Обрабатывает события, пока соединение не закроется.
///
/// Эта функция обрабатывает GATT-события.
/// Так мы обрабатываем запросы на чтение и запись.
async fn gatt_events_task<P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
) -> Result<(), Error> {
    let sensor_data = server.sensor_service.sensor_data;

    let reason = loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => break reason,
            GattConnectionEvent::Gatt { event } => {
                match &event {
                    GattEvent::Read(event) => {
                        if event.handle() == sensor_data.handle {
                            let value = server.get(&sensor_data);
                            info!(
                                "[gatt] Read Event to Sensor Data Characteristic: {:?}",
                                value
                            );
                        }
                    }
                    GattEvent::Write(event) => {
                        if event.handle() == sensor_data.handle {
                            info!(
                                "[gatt] Write Event to Sensor Data Characteristic: {:?}",
                                event.data()
                            );
                        }
                    }
                    _ => {}
                };
                // Этот шаг также выполняется в drop(), но явная запись нужна, чтобы гарантировать отправку ответа.
                // (так гарантируется, что ответ будет отправлен).
                match event.accept() {
                    Ok(reply) => reply.send().await,
                    Err(e) => warn!("[gatt] error sending response: {:?}", e),
                };
            }
            _ => {} // игнорируем прочие события GATT-соединения
        }
    };

    info!("[gatt] disconnected: {:?}", reason);
    Ok(())
}
```

Разберём эту функцию по шагам.

## Цикл обработки событий

```rust
let sensor_data = server.sensor_service.sensor_data;

let reason = loop {
    match conn.next().await {
        GattConnectionEvent::Disconnected { reason } => break reason,
        GattConnectionEvent::Gatt { event } => {
            // ... обработка события
        }
        _ => {} // игнорируем прочие события GATT-соединения
    }
};
info!("[gatt] disconnected: {:?}", reason);
```

Сначала получаем ссылку на нашу характеристику данных датчика, затем входим в цикл, который ждёт событий соединения. Цикл продолжается, пока не произойдёт разрыв соединения; тогда он завершается и записывает причину разрыва в лог.

## Обработка событий чтения

```rust
GattEvent::Read(event) => {
    if event.handle() == sensor_data.handle {
        let value = server.get(&sensor_data);
        info!(
            "[gatt] Read Event to Sensor Data Characteristic: {:?}",
            value
        );
    }
}
```

Когда central-устройство читает характеристику, мы проверяем, совпадает ли её дескриптор (handle) с нашей характеристикой данных датчика. Если совпадает, получаем текущее значение с сервера и выводим его в лог.  Библиотека сама отправляет значение при вызове accept и send.

## Обработка событий записи

```rust
GattEvent::Write(event) => {
    if event.handle() == sensor_data.handle {
        info!(
            "[gatt] Write Event to Sensor Data Characteristic: {:?}",
            event.data()
        );
    }
}
```

Когда central-устройство пишет в характеристику, мы проверяем, пишет ли оно в нашу характеристику данных датчика. Если да, выводим в лог записываемые данные. Непосредственно запись обработается, когда мы вызовем `accept()` для события.


## Отправка ответов

```rust
match event.accept() {
    Ok(reply) => reply.send().await,
    Err(e) => warn!("[gatt] error sending response: {:?}", e),
};
```

После обработки каждого события нужно явно принять его и отправить ответ клиенту. Хотя это также происходит автоматически, когда событие уничтожается, явный вызов гарантирует, что ответ будет отправлен сразу. Если отправить ответ не удалось, мы выводим предупреждение в лог.
