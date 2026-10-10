{{#title Чтение UID RFID-метки с RC522 и ESP32 на embedded Rust}}

# Чтение UID

Ну а теперь к интересной части! Начнём с простой программы, которая читает UID RFID-метки.

## Создание проекта с помощью esp-generate
Мы включим поддержку async (Embassy) для этого проекта. Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 rfid-uid
```

Откроется экран с вопросом о выборе опций.

- Выберите опцию «Enable unstable HAL features».
- Затем выберите опцию «Adds embassy framework support».

Сохраните настройки, нажав на клавиатуре «s».

## Дополнительные крейты

Добавьте в `Cargo.toml` эти крейты к уже существующим зависимостям.

```rust
mfrc522 = "0.8.0"
embedded-hal-bus = "0.2.0"
```

### Драйвер mfrc522
Мы будем использовать замечательный крейт «[mfrc522](https://crates.io/crates/mfrc522)». Он всё ещё в разработке, но для наших целей у него есть всё необходимое.


### embedded-hal-bus

Чтобы понять, зачем нужен крейт embedded-hal-bus, сначала разберёмся с Embedded HAL (Hardware Abstraction Layer). Embedded HAL предоставляет несколько трейтов, которые задают стандартный способ управления распространённой периферией микроконтроллеров, например GPIO, PWM и интерфейсами связи (такими как I2C, SPI и UART). Это позволяет драйверам работать на разных микроконтроллерах (например, ESP32, Raspberry Pi Pico).

Когда мы хотим обмениваться данными с RFID-меткой по SPI, embedded-hal предоставляет трейты `SpiBus` и `SpiDevice` для поддержки совместного использования шины. `SpiBus` представляет всю шину, а `SpiDevice` — устройство на этой шине. Специфичные для микроконтроллера HAL (например, esp-hal) обычно реализуют трейт `SpiBus`, а драйверы устройств, такие как mfrc522, реализуют трейт `SpiDevice`.

Поэтому нам нужно получить `SpiDevice` из `SpiBus`, чтобы работать с RFID-считывателем. Здесь помогает крейт embedded-hal-bus. Он предоставляет разные реализации `SpiDevice`, например `CriticalSectionDevice`, `ExclusiveDevice` и другие. Мы воспользуемся `ExclusiveDevice`, поскольку это самый простой способ получить `SpiDevice` из `SpiBus`, и он подходит, когда другие устройства не используют эту шину SPI.


### Настройка SPI для RFID-считывателя
Чтобы обмениваться данными с RFID-модулем, инициализируем экземпляр SPI на периферии SPI2. Здесь мы настроим тактовую частоту SPI на 5 МГц и назначим нужные выводы на GPIO для правильной связи.

```rust
let spi_bus = Spi::new(
    peripherals.SPI2,
    spi::master::Config::default()
        .with_frequency(Rate::from_mhz(5))
        .with_mode(spi::Mode::_0),
)
.unwrap()
.with_sck(peripherals.GPIO18)
.with_mosi(peripherals.GPIO23)
.with_miso(peripherals.GPIO19)
.into_async();

let sd_cs = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());
```

### Получение `SpiDevice` из SPI-шины
Чтобы работать с крейтом mfrc522, нам нужен `SpiDevice`. Поскольку у нас есть только шина SPI из esp-hal, мы используем крейт embedded_hal_bus, чтобы получить `SpiDevice` из шины SPI.

```rust
let delay = Delay::new();
let spi_dev = ExclusiveDevice::new(spi_bus, sd_cs, delay).unwrap();
```

### Инициализация mfrc522
Далее инициализируем драйвер MFRC522. Для этого оборачиваем экземпляр `SpiDevice` в обёртку `SpiInterface` из крейта mfrc522 и передаём её при инициализации `Mfrc522`:

```rust
let spi_interface = SpiInterface::new(spi_dev);
let mut rfid = Mfrc522::new(spi_interface).init().unwrap();
```


### Вспомогательная функция для вывода массива байтов в виде hex-строки

Эту вспомогательную функцию мы используем, чтобы превратить массив байтов `u8` (например, UID) в печатаемую hex-строку. Она будет использоваться во всех упражнениях с RFID для вывода данных. В зависимости от требований конкретного упражнения её может понадобиться немного изменить.

```rust
fn print_hex_bytes(data: &[u8]) {
    for &b in data.iter() {
        print!("{:02x} ", b);
    }
    println!("");
}
```

### Чтение UID и вывод
Основная логика чтения UID простая. Мы непрерывно отправляем команду REQA (Request A), чтобы проверить, есть ли поблизости карта или метка. Если карта присутствует, она отвечает ATQA (Answer To reQuest code A).

ATQA содержит информацию о типе метки, её возможностях и других деталях. Используя ответ ATQA, мы выбираем метку и получаем её UID.

```rust
loop {
    if let Ok(atqa) = rfid.reqa() {
        println!("Answer To reQuest code A");
        Timer::after(Duration::from_millis(50)).await;
        if let Ok(uid) = rfid.select(&atqa) {
            print_hex_bytes(uid.as_bytes());
            Timer::after(Duration::from_millis(500)).await;
        }
    }
}
```


## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `rfid-uid`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/rfid-uid
```


## Вывод UID
После прошивки кода в ESP32 поднесите RFID-метку к считывателю. Байты UID будут выведены в системную консоль в hex-формате. Затем попробуйте то же самое с брелоком: он должен вывести другой UID.

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
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;

// SPI
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::spi;
use esp_hal::spi::master::Spi;
use esp_hal::time::Rate;

// RFID-считыватель
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::delay::Delay;
use mfrc522::Mfrc522;
use mfrc522::comm::blocking::spi::SpiInterface;

use esp_println::{self as _, print, println};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Создаёт дескриптор приложения по умолчанию, который требуется загрузчику esp-idf.
// Подробнее см.: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.0.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    info!("Embassy initialized!");

    let _ = spawner;

    let spi_bus = Spi::new(
        peripherals.SPI2,
        spi::master::Config::default()
            .with_frequency(Rate::from_mhz(5))
            .with_mode(spi::Mode::_0),
    )
    .unwrap()
    .with_sck(peripherals.GPIO18)
    .with_mosi(peripherals.GPIO23)
    .with_miso(peripherals.GPIO19)
    .into_async();

    let sd_cs = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());

    let delay = Delay::new();
    let spi_dev = ExclusiveDevice::new(spi_bus, sd_cs, delay).unwrap();

    let spi_interface = SpiInterface::new(spi_dev);
    let mut rfid = Mfrc522::new(spi_interface).init().unwrap();

    loop {
        if let Ok(atqa) = rfid.reqa() {
            println!("Answer To reQuest code A");
            Timer::after(Duration::from_millis(50)).await;
            if let Ok(uid) = rfid.select(&atqa) {
                print_hex_bytes(uid.as_bytes());
                Timer::after(Duration::from_millis(500)).await;
            }
        }
    }
}

fn print_hex_bytes(data: &[u8]) {
    for &b in data.iter() {
        print!("{:02x} ", b);
    }
    println!("");
}
```
