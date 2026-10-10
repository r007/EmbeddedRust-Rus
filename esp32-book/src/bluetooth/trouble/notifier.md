{{#title Отправка уведомлений Bluetooth BLE с ESP32 на Rust}}

# Уведомления (Notifier)

Эта задача показывает, как отправлять данные подключённому central-устройству. Представьте, что это данные датчика, которые вы периодически собираете и отправляете на мобильное устройство или другое устройство. В этом примере мы имитируем показания датчика: каждые 2 секунды увеличиваем счётчик и отправляем уведомления подключённому устройству.


```rust
/// Пример задачи, использующей интерфейс уведомлений BLE.
/// Задача отправляет подключённому central значение счётчика каждые 2 секунды.
/// Также каждые 2 секунды читает значение RSSI.
/// и завершается, когда central закрывает соединение или происходит ошибка.
async fn custom_task<C: Controller, P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
    stack: &Stack<'_, C, P>,
) {
    let mut tick: u8 = 0;
    let sensor_data = server.sensor_service.sensor_data;
    loop {
        tick = tick.wrapping_add(1);
        info!("[custom_task] notifying connection of tick {}", tick);
        if sensor_data.notify(conn, &tick).await.is_err() {
            info!("[custom_task] error notifying connection");
            break;
        };
        // читаем RSSI (индикатор уровня принятого сигнала) соединения.
        if let Ok(rssi) = conn.raw().rssi(stack).await {
            info!("[custom_task] RSSI: {:?}", rssi);
        } else {
            info!("[custom_task] error getting RSSI");
            break;
        };
        Timer::after_secs(2).await;
    }
}
```

## Отправка уведомлений

```rust
let mut tick: u8 = 0;
let sensor_data = server.sensor_service.sensor_data;
loop {
    tick = tick.wrapping_add(1);
    info!("[custom_task] notifying connection of tick {}", tick);
    if sensor_data.notify(conn, &tick).await.is_err() {
        info!("[custom_task] error notifying connection");
        break;
    };
    // ...
    Timer::after_secs(2).await;
}
```

Мы ведём счётчик, который увеличивается на каждой итерации цикла. Метод `notify()` отправляет текущее значение счётчика подключённому central-устройству. Если уведомление не удалось отправить (обычно потому, что соединение разорвано), выходим из цикла.

## Чтение уровня сигнала
```rust
if let Ok(rssi) = conn.raw().rssi(stack).await {
    info!("[custom_task] RSSI: {:?}", rssi);
} else {
    info!("[custom_task] error getting RSSI");
    break;
};
```

RSSI (Received Signal Strength Indicator, индикатор уровня принятого сигнала) измеряет качество соединения в дБм. Мы читаем это значение, чтобы отслеживать силу соединения. Если прочитать RSSI не удалось, обычно это значит, что соединение потеряно, и мы завершаем задачу.
