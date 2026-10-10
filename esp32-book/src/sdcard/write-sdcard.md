{{#title Код на Rust: запись файла на SD-карту с ESP32}}

# Код на Rust для записи файла на SD-карту с помощью ESP32

В этом упражнении мы создадим (или перезапишем) файл на SD-карте и запишем в него «Hello, Ferris!».

## Создание проекта с помощью esp-generate
Мы включим поддержку async (Embassy) для этого проекта. Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 sdcard-write
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

## Для разбора времени
jiff = { version = "0.2.16", default-features = false, features = ["static"] }
```

## Источник времени с RTC

Прежде чем переходить к записи, создадим `TimeSource` для крейта sdmmc. В предыдущем упражнении мы только читали файл и использовали фиктивный источник времени. Однако здесь нам нужно создать файл и убедиться, что его метаданные времени обновляются правильно. Для этого создадим источник времени, достаточно близкий к реальному для наших задач.

Для этого используем встроенный RTC. Это не идеальное решение: RTC требует задать начальное время (мы передадим его через переменную окружения), и он сбрасывается при каждом перезапуске ESP32. Альтернативно можно получать текущее время с NTP-серверов по Wi-Fi.

Создадим структуру `SdTimeSource`, которая реализует трейт `TimeSource` и использует `Rtc` для получения текущего времени. Нужно указать, сколько лет прошло с 1970 года (эпохи Unix). Это значение получим, вычтя 1970 из текущего года. Также нужно указать месяц и день в формате с нумерацией с нуля, а время — как есть.

```rust
static TZ: jiff::tz::TimeZone = jiff::tz::get!("America/New_York");

struct SdTimeSource {
    timer: Rtc<'static>,
}

impl SdTimeSource {
    fn new(timer: Rtc<'static>) -> Self {
        Self { timer }
    }

    fn current_time(&self) -> u64 {
        self.timer.current_time_us()
    }
}

static TZ: jiff::tz::TimeZone = jiff::tz::get!("America/New_York");

impl TimeSource for SdTimeSource {
    fn get_timestamp(&self) -> Timestamp {
        let now_us = self.current_time();

        // Переводим в тип jiff Time
        let now = jiff::Timestamp::from_microsecond(now_us as i64).unwrap();
        let now = now.to_zoned(TZ.clone());

        Timestamp {
            year_since_1970: (now.year() - 1970).unsigned_abs() as u8,
            zero_indexed_month: now.month().wrapping_sub(1) as u8,
            zero_indexed_day: now.day().wrapping_sub(1) as u8,
            hours: now.hour() as u8,
            minutes: now.minute() as u8,
            seconds: now.second() as u8,
        }
    }
}
```

Далее обновим код инициализации SD-карты, чтобы использовать `SdTimeSource` в качестве источника времени. Текущее время мы передадим через переменную окружения, разберём его с помощью jiff и установим как текущее время для RTC.


```rust
// Таймер для SD-карты
let rtc = Rtc::new(peripherals.LPWR);
let current_time_us: u64 = env!("CURRENT_TIME_US")
    .parse()
    .expect("Invalid microseconds");
rtc.set_current_time_us(current_time_us);

let sd_timer = SdTimeSource::new(rtc);

println!("Init SD card controller and retrieve card size...");
let sd_size = sdcard.num_bytes().unwrap();
println!("card size is {} bytes\r\n", sd_size);

// Теперь найдём тома (они же разделы) на нашем блочном устройстве.
// Для этого нужен менеджер томов. Он возьмёт на себя владение блочным устройством.
let volume_mgr = VolumeManager::new(sdcard, sd_timer);

// Пробуем получить доступ к тому 0 (то есть к первому разделу).
// Объект тома хранит информацию о файловой системе на этом томе.
let volume0 = volume_mgr.open_volume(VolumeIdx(0)).unwrap();

let root_dir = volume0.open_root_dir().unwrap();
```

## Запись в файл

Откроем файл в режиме `ReadWriteCreateOrTruncate`. Если файла нет, он будет создан. Если он существует, его содержимое будет обрезано, то есть очищено.

```rust
let mut my_file = root_dir
.open_file_in_dir(
    "FERRIS.TXT",
    embedded_sdmmc::Mode::ReadWriteCreateOrTruncate,
)
.unwrap();
```

Когда файл открыт, можно записать в него сообщение и затем вызвать `flush`, чтобы гарантировать сохранение данных.

```rust
let line = "Hello, Ferris!";
if let Ok(()) = my_file.write(line.as_bytes()) {
    my_file.flush().unwrap();
    println!("Written Data");
} else {
    println!("Not wrote");
}
```

Чтобы проверить результат, можно либо подключить SD-карту к компьютеру, либо снова запустить программу чтения SD-карты, которую мы использовали раньше.

## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `sdcard-write`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/sdcard-write
```

## Как запустить?

Текущую дату и время мы передадим как переменную окружения. Для этого добавим перед `cargo run` команду оболочки, которая получит текущие дату и время и передаст их `cargo run`. Это может не работать в оболочках вроде Fish или в Windows.

```sh
CURRENT_DATETIME="$(date '+%Y-%m-%d %H:%M:%S')" cargo run --release
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
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_println::{self as _, println};

// SPI
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::spi::{self, master::Spi};
use esp_hal::time::Rate;

// Считыватель SD-карт
use embedded_sdmmc::{SdCard, TimeSource, Timestamp, VolumeIdx, VolumeManager};

// Для времени
use esp_hal::rtc_cntl::Rtc;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Создаёт дескриптор приложения по умолчанию, который требуется загрузчику esp-idf.
// Подробнее см.: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

struct SdTimeSource {
    timer: Rtc<'static>,
}

impl SdTimeSource {
    fn new(timer: Rtc<'static>) -> Self {
        Self { timer }
    }

    fn current_time(&self) -> u64 {
        self.timer.current_time_us()
    }
}

static TZ: jiff::tz::TimeZone = jiff::tz::get!("America/New_York");

impl TimeSource for SdTimeSource {
    fn get_timestamp(&self) -> Timestamp {
        let now_us = self.current_time();

        // Переводим в тип jiff Time
        let now = jiff::Timestamp::from_microsecond(now_us as i64).unwrap();
        let now = now.to_zoned(TZ.clone());

        Timestamp {
            year_since_1970: (now.year() - 1970).unsigned_abs() as u8,
            zero_indexed_month: now.month().wrapping_sub(1) as u8,
            zero_indexed_day: now.day().wrapping_sub(1) as u8,
            hours: now.hour() as u8,
            minutes: now.minute() as u8,
            seconds: now.second() as u8,
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

    // Таймер для SD-карты
    let rtc = Rtc::new(peripherals.LPWR);
    let current_time_us: u64 = env!("CURRENT_TIME_US")
        .parse()
        .expect("Invalid microseconds");
    rtc.set_current_time_us(current_time_us);

    let sd_timer = SdTimeSource::new(rtc);

    let sdcard = SdCard::new(spi_dev, Delay);

    println!("Init SD card controller and retrieve card size...");
    let sd_size = sdcard.num_bytes().unwrap();
    println!("card size is {} bytes\r\n", sd_size);

    // Теперь найдём тома (они же разделы) на нашем блочном устройстве.
    // Для этого нужен менеджер томов. Он возьмёт на себя владение блочным устройством.
    let volume_mgr = VolumeManager::new(sdcard, sd_timer);

    // Пробуем получить доступ к тому 0 (то есть к первому разделу).
    // Объект тома хранит информацию о файловой системе на этом томе.
    let volume0 = volume_mgr.open_volume(VolumeIdx(0)).unwrap();

    let root_dir = volume0.open_root_dir().unwrap();

    let my_file = root_dir
        .open_file_in_dir(
            "FERRIS.TXT",
            embedded_sdmmc::Mode::ReadWriteCreateOrTruncate,
        )
        .unwrap();

    let line = "Hello, Ferris!";
    if let Ok(()) = my_file.write(line.as_bytes()) {
        my_file.flush().unwrap();
        println!("Written Data");
    } else {
        println!("Not wrote");
    }

    loop {
        Timer::after(Duration::from_secs(30)).await;
    }
}
```
