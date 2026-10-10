{{#title Чтение файлов с SD-карты на ESP32 на embedded Rust}}

# Пишем код на Rust для чтения файла с SD-карты через ESP32

Создадим простую программу, которая читает файл с SD-карты и выводит его содержимое в системную консоль. Убедитесь, что SD-карта отформатирована в FAT32 и содержит файл для чтения (например, "FERRIS.TXT" с содержимым "Hello, World!").

## Создание проекта с помощью esp-generate
Мы включим поддержку async (Embassy) для этого проекта. Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 sdcard-read
```

Откроется экран с вопросом о выборе опций.

- Выберите опцию «Enable unstable HAL features»
- Затем выберите опцию «Adds embassy framework support».

Сохраните настройки, нажав на клавиатуре «s».

## Дополнительные крейты
Добавьте в `Cargo.toml` эти крейты к уже существующим зависимостям.

```toml
# драйвер SD-карты
embedded-sdmmc = "0.9.0"
# для преобразования шины SPI в SpiDevice
embedded-hal-bus = "0.3.0"
```


### embedded-sdmmc

Мы будем использовать этот крейт, чтобы читать и записывать файлы на SD-карте для встраиваемых устройств. Он не использует alloc и коллекции, чтобы минимизировать потребление памяти.


### embedded-hal-bus

Чтобы понять, зачем нужен крейт embedded-hal-bus, сначала разберёмся с Embedded HAL (Hardware Abstraction Layer). Embedded HAL предоставляет несколько трейтов, которые задают стандартный способ управления распространённой периферией микроконтроллеров, например GPIO, PWM и интерфейсами связи (I2C, SPI, UART). Это позволяет драйверам работать на разных микроконтроллерах (например, ESP32, Raspberry Pi Pico).

Когда мы хотим обмениваться данными с SD-картой по SPI, embedded-hal предоставляет трейты `SpiBus` и `SpiDevice` для поддержки совместного использования шины. `SpiBus` представляет всю шину, а `SpiDevice` — устройство на этой шине. Специфичные для микроконтроллера HAL (например, esp-hal) обычно реализуют трейт `SpiBus`, а драйверы устройств, такие как sdmmc, реализуют трейт `SpiDevice`.

Поэтому нам нужно получить `SpiDevice` из `SpiBus`, чтобы использовать его с SD-картой. Здесь помогает крейт embedded-hal-bus. Он предоставляет разные реализации `SpiDevice`, например `CriticalSectionDevice`, `ExclusiveDevice` и другие. Мы используем `ExclusiveDevice`, поскольку это самый простой способ получить `SpiDevice` из `SpiBus`, и он подходит, когда шину SPI не делят другие устройства.

## Фиктивный источник времени

Когда вы работаете с файлами на компьютере, вы могли заметить, что у файлов и каталогов есть время создания и изменения, которые фиксируют, когда были сделаны изменения. С SD-картами то же самое: при создании или изменении файлов обновляется их отметка времени.

Драйверу sdmmc нужен источник времени, чтобы получать текущее время для этих обновлений. Он предоставляет трейт `TimeSource`, который нужно реализовать и передать в `VolumeManager` при инициализации.

Так как мы только читаем файлы и эту функцию не используем, создадим `DummyTimesource`, который реализует трейт `TimeSource`.

```rust
/// Код из https://github.com/rp-rs/rp-hal-boards/blob/main/boards/rp-pico/examples/pico_spi_sd_card.rs
/// Фиктивный источник времени, который важен главным образом при создании файлов.
#[derive(Default)]
pub struct DummyTimesource();

impl TimeSource for DummyTimesource {
    // Теоретически здесь можно использовать RTC rp2040, если бы было
    // внешнее устройство синхронизации времени.
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}
```

## Настройка SPI для считывателя SD-карт
Чтобы обмениваться данными со считывателем SD-карт, инициализируем экземпляр SPI на периферии SPI2. SD-карты требуют, чтобы тактовая частота SPI работала в диапазоне от 100 кГц до 400 кГц. В этой настройке мы зададим тактовую частоту SPI 400 кГц и назначим нужные выводы на GPIO для правильной связи.

Линия SCK (Serial Clock) будет назначена на GPIO18, MOSI (Master Out, Slave In) — на GPIO23, а MISO (Master In, Slave Out) — на GPIO19. Кроме того, настроим линию CS (Chip Select) на GPIO5 и зададим её начальное состояние High.

```rust
let spi_bus = Spi::new(
    peripherals.SPI2,
    spi::master::Config::default()
        .with_frequency(Rate::from_khz(400))
        .with_mode(spi::Mode::_0),
)
.unwrap()
.with_sck(peripherals.GPIO18)
.with_mosi(peripherals.GPIO23)
.with_miso(peripherals.GPIO19)
.into_async();

let sd_cs = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());
```

Когда SPI настроен, создадим SpiDevice для считывателя SD-карт. Для этого используем `ExclusiveDevice` из крейта embedded-hal-bus. Наконец, инициализируем SD-карту.

```rust
let spi_dev = ExclusiveDevice::new(spi_bus, sd_cs, Delay).unwrap();
let sdcard = SdCard::new(spi_dev, Delay);
```

##  Вывод размера SD-карты

Далее с помощью менеджера томов получим размер SD-карты в байтах и выведем его.

```rust
println!("Init SD card controller and retrieve card size...");
let sd_size = sdcard.num_bytes().unwrap();
println!("card size is {} bytes\r\n", sd_size);
```

## Менеджер томов

Следующий шаг — инициализировать менеджер томов, который будет управлять разделами и файловыми системами на SD-карте.

```rust
// Теперь найдём тома (они же разделы) на нашем блочном устройстве.
// Для этого нужен менеджер томов. Он возьмёт на себя владение блочным устройством.
let volume_mgr = VolumeManager::new(sdcard, DummyTimesource::default());
```

## Открываем каталог

Через менеджер томов открываем первый основной раздел (`VolumeIdx(0)`) на SD-карте. После этого получаем доступ к корневому каталогу этого раздела.

```rust
// Пробуем получить доступ к тому 0 (то есть к первому разделу).
// Объект тома хранит информацию о файловой системе на этом томе.
let volume0 = volume_mgr.open_volume(VolumeIdx(0)).unwrap();

let root_dir = volume0.open_root_dir().unwrap();
```


### Открываем файл

Откроем файл "FERRIS.TXT" в режиме только для чтения. Убедитесь, что вы заранее добавили этот файл на SD-карту с каким-нибудь содержимым.

```rust
let mut my_file = root_dir
    .open_file_in_dir("FERRIS.TXT", embedded_sdmmc::Mode::ReadOnly)
    .unwrap();
```

### Вывод содержимого файла

Будем читать файл до конца. Каждый байт преобразуем в символ и выведем его.

```rust
while !my_file.is_eof() {
    let mut buffer = [0u8; 32];

    if let Ok(n) = my_file.read(&mut buffer) {
        for b in &buffer[..n] {
            print!("{}", *b as char);
        }
    }
}
```


## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `sdcard-read`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/sdcard-read
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

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Timer};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::spi;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{clock::CpuClock, spi::master::Spi};
use esp_println::{self as _, print, println};

// Считыватель SD-карт
use embedded_sdmmc::{SdCard, TimeSource, Timestamp, VolumeIdx, VolumeManager};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Создаёт дескриптор приложения по умолчанию, который требуется загрузчику esp-idf.
// Подробнее см.: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

/// Код из https://github.com/rp-rs/rp-hal-boards/blob/main/boards/rp-pico/examples/pico_spi_sd_card.rs
/// Фиктивный источник времени, который важен главным образом при создании файлов.
#[derive(Default)]
pub struct DummyTimesource();

impl TimeSource for DummyTimesource {
    // Теоретически здесь можно использовать RTC rp2040, если бы было
    // внешнее устройство синхронизации времени.
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

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
            .with_frequency(Rate::from_khz(400))
            .with_mode(spi::Mode::_0),
    )
    .unwrap()
    .with_sck(peripherals.GPIO18)
    .with_mosi(peripherals.GPIO23)
    .with_miso(peripherals.GPIO19)
    .into_async();

    let sd_cs = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());
    let spi_dev = ExclusiveDevice::new(spi_bus, sd_cs, Delay).unwrap();
    let sdcard = SdCard::new(spi_dev, Delay);

    println!("Init SD card controller and retrieve card size...");
    let sd_size = sdcard.num_bytes().unwrap();
    println!("card size is {} bytes\r\n", sd_size);

    // Теперь найдём тома (они же разделы) на нашем блочном устройстве.
    // Для этого нужен менеджер томов. Он возьмёт на себя владение блочным устройством.
    let volume_mgr = VolumeManager::new(sdcard, DummyTimesource::default());

    // Пробуем получить доступ к тому 0 (то есть к первому разделу).
    // Объект тома хранит информацию о файловой системе на этом томе.
    let volume0 = volume_mgr.open_volume(VolumeIdx(0)).unwrap();

    let root_dir = volume0.open_root_dir().unwrap();

    let my_file = root_dir
        .open_file_in_dir("FERRIS.TXT", embedded_sdmmc::Mode::ReadOnly)
        .unwrap();

    while !my_file.is_eof() {
        let mut buffer = [0u8; 32];

        if let Ok(n) = my_file.read(&mut buffer) {
            for b in &buffer[..n] {
                print!("{}", *b as char);
            }
        }
    }

    loop {
        Timer::after(Duration::from_secs(30)).await;
    }
}
```
