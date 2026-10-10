{{#title Реклама Bluetooth и обнаружение устройств на ESP32 на Rust}}

# Реклама BLE (advertising)

Реклама BLE (advertising) — это способ, которым устройство сообщает о себе ближайшим устройствам (например, телефонам или компьютерам) и передаёт информацию, чтобы к нему можно было подключиться. Устройство рассылает своё имя, доступные сервисы и возможности.

Когда central-устройство инициирует подключение, функция принимает его и возвращает экземпляр GATT-соединения, который используется для обмена данными с подключённым устройством.

```rust
/// Создаёт advertiser для подключения к BLE Central и ждёт, пока тот подключится.
async fn advertise<'values, 'server, C: Controller>(
    name: &'values str,
    peripheral: &mut Peripheral<'values, C, DefaultPacketPool>,
    server: &'server Server<'values>,
) -> Result<GattConnection<'values, 'server, DefaultPacketPool>, BleHostError<C::Error>> {
    let mut advertiser_data = [0; 31];
    let len = AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::CompleteLocalName(name.as_bytes()),
        ],
        &mut advertiser_data[..],
    )?;
    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data: &advertiser_data[..len],
                scan_data: &[],
            },
        )
        .await?;
    info!("[adv] advertising");
    let conn = advertiser.accept().await?.with_attribute_server(server)?;
    info!("[adv] connection established");
    Ok(conn)
}
```

Разберём эту функцию по шагам.

## Подготовка данных рекламы

```rust
let mut advertiser_data = [0; 31];
 
let len = AdStructure::encode_slice(
    &[
        AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
        AdStructure::CompleteLocalName(name.as_bytes()),
    ],
    &mut advertiser_data[..],
)?;
```

Создаём буфер и кодируем в него две части информации:

- Flags: `LE_GENERAL_DISCOVERABLE` делает устройство видимым для всех сканеров, а `BR_EDR_NOT_SUPPORTED` указывает, что это устройство только для BLE (без классического Bluetooth). Подробнее о флагах — [здесь](../ble/gap.html#флаги-рекламы).
- Имя устройства: полное локальное имя, которое появляется при поиске устройств

Функция `encode_slice` упаковывает эти данные в правильный формат рекламы BLE и возвращает фактически использованную длину.


## Запуск рекламы

```rust
let advertiser = peripheral
    .advertise(
        &Default::default(),
        Advertisement::ConnectableScannableUndirected {
            adv_data: &advertiser_data[..len],
            scan_data: &[],
        },
    )
    .await?;
```

Запускаем рекламу в режиме `ConnectableScannableUndirected`, а это значит:
- Connectable: central-устройства могут подключиться к нам
- Scannable: central-устройства могут запросить дополнительную информацию (хотя здесь мы передаём пустой `scan_data`)
- Undirected: мы вещаем всем, а не нацеливаемся на конкретное устройство

Теперь устройство видно ближайшим BLE-сканерам.


## Приём подключений

```rust
let conn = advertiser.accept().await?.with_attribute_server(server)?;
Ok(conn)
```

Вызов `accept()` ждёт, пока central-устройство инициирует подключение. После подключения мы преобразуем соединение в `GattConnection` с помощью `with_attribute_server()`, которая подключает наш GATT-сервер (содержащий наши сервисы и характеристики) к соединению.
