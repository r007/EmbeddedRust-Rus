{{#title Как написать код на Rust для Bluetooth Low Energy (BLE) на ESP32}}

# Как написать код на Rust для Bluetooth Low Energy (BLE) на ESP32

Создадим простую программу, которая демонстрирует Bluetooth Low Energy (BLE).  В этом упражнении мы будем использовать крейт `Trouble` вместе с Embassy. Определим GATT-сервис с двумя характеристиками:

- Одна характеристика поддерживает и чтение, и запись.  
- Вторая характеристика позволяет только чтение.  

Я сгенерировал UUID для этих атрибутов (сервисов и характеристик), и вы можете использовать те же самые.

> Больше примеров смотрите в репозитории Trouble: [https://github.com/embassy-rs/trouble/tree/main/examples](https://github.com/embassy-rs/trouble/tree/main/examples)

## Подключение к Bluetooth ESP32

Для взаимодействия с Bluetooth ESP32 будем использовать приложение nRF Connect for Mobile:  

🔗 [nRF Connect for Mobile](https://www.nordicsemi.com/Products/Development-tools/nRF-Connect-for-mobile)  

С помощью этого приложения можно читать и записывать данные, которые предоставляет ESP32.


## Создание проекта с помощью esp-generate

Для этого проекта включим поддержку async (Embassy).  Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 bluetooth-low-energy
```

Откроется экран с вопросом о выборе опций. 

- Сначала выберите опцию «Enable unstable HAL features».
- Выберите опцию «Enable allocations via the esp-alloc crate».
- Выберите опцию «Adds embassy framework support».
- Теперь можно включить «Enable BLE via the esp-radio crate (embassy-trouble)».

Сохраните настройки, нажав на клавиатуре «s».

## Обновляем зависимости

```toml
embassy-futures = "0.1.1"

# Необходимо для работы макроса gatt_server
embassy-sync = { version = "0.7" }
```

## Инициализация контроллера Wi-Fi

ESP32 использует один радиомодуль и для Wi-Fi, и для Bluetooth. Чтобы инициализировать Bluetooth, мы будем использовать тот же контроллер, что и для Wi-Fi.
 
```rust
let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
```

Инициализируем Bluetooth-коннектор. 

```rust
// больше примеров: https://github.com/embassy-rs/trouble/tree/main/examples/esp32
let transport = BleConnector::new(&radio_init, peripherals.BT, Default::default()).unwrap();
let ble_controller = ExternalController::<_, 20>::new(transport);

// let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> =
//     HostResources::new();
// let _stack = trouble_host::new(ble_controller, &mut resources);
```

После создания контроллера BLE HCI вызовем функцию `run`, которую определим чуть позже, чтобы запустить стек BLE.

```rust
ble::run(ble_controller).await;

loop {
    Timer::after(Duration::from_secs(5)).await;
}
```


## Полный код main.rs

Это полный код файла `main.rs`. Далее создадим модуль `ble`, в котором реализуем функцию run вместе с вспомогательными функциями для рекламы и обработки событий.

```rust
#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use bt_hci::controller::ExternalController;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;
use esp_radio::ble::controller::BleConnector;

// Наш модуль
use bluetooth_low_energy as lib;
use lib::ble;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// Создаёт дескриптор приложения по умолчанию, который требуется загрузчику esp-idf.
// Подробнее см.: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    // generator version: 1.0.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 98767);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    info!("Embassy initialized!");

    let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    // больше примеров: https://github.com/embassy-rs/trouble/tree/main/examples/esp32
    let transport = BleConnector::new(&radio_init, peripherals.BT, Default::default()).unwrap();
    let ble_controller = ExternalController::<_, 20>::new(transport);
    // let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> =
    //     HostResources::new();
    // let _stack = trouble_host::new(ble_controller, &mut resources);

    ble::run(ble_controller).await;

    loop {
        Timer::after(Duration::from_secs(5)).await;
    }
}
```
