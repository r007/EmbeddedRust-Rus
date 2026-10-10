{{#title Чтение файлов с SD-карты на Raspberry Pi Pico 2 во встраиваемом Rust}}

# Чтение SD-карты с Raspberry Pi Pico

В этой главе мы прочитаем файл с microSD-карты и выведем его содержимое. Прежде чем продолжить, нужна SD-карта, уже отформатированная в FAT32, и на ней должен быть файл с именем RUST.TXT. Для этого примера можно записать в него текст Ferris.

## Выберите способ вывода

В этом разделе приведены два способа вывода. Какой из них использовать, зависит от того, есть ли у вас отладочный зонд.

С точки зрения логики и поведения разницы нет. Программа работает одинаково в обоих случаях. Основное отличие — в том, как выводятся данные.

Если вы используете отладочный зонд, мы выводим сообщения через defmt по RTT. Если используете USB Serial, мы выводим сообщения через крейт log.

Настройка USB Serial такая же, как в главе о USB Serial. Возможно, потребуется выполнить те же шаги для настройки USB-логгера. Здесь я их повторять не буду.

С точки зрения кода различие в основном сводится к следующему:

- Поскольку с отладочным зондом мы используем defmt, логирование выполняется через `defmt::info!`
- А при USB Serial мы используем крейт log, поэтому логирование выполняется через `log::info!`

Выберите вкладку ниже в зависимости от вашей настройки:

{{#tabs global="log-method" }}
{{#tab name="Отладочный зонд" }}
В этой схеме содержимое файла выводится через defmt по RTT.
{{#endtab }}
{{#tab name="USB Serial" }}
В этой схеме содержимое файла выводится через логирование по USB Serial.
{{#endtab }}
{{#endtabs }}

## Проект из шаблона

Начнём с создания нового проекта из шаблона.

```sh
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

{{#tabs global="log-method" }}
{{#tab name="Отладочный зонд" }}
Когда появится запрос, введите имя проекта, например «read-sdcard», и выберите «Embassy» в качестве HAL. Обязательно включите «defmt» для логирования.
{{#endtab }}
{{#tab name="USB Serial" }}
Когда появится запрос, введите имя проекта, например «read-sdcard», и выберите «Embassy» в качестве HAL. Для этой схемы включать defmt не нужно.
{{#endtab }}
{{#endtabs }}

## Требуемые дополнительные крейты

Обновите Cargo.toml и добавьте эти дополнительные крейты вместе с существующими зависимостями.

{{#tabs global="log-method" }}
{{#tab name="Отладочный зонд" }}

```toml
# Для преобразования шины SPI в SpiDevice
embedded-hal-bus = "0.3.0"

# Драйвер SD-карты
embedded-sdmmc = "0.9.0"
```

{{#endtab }}
{{#tab name="USB Serial" }}

```toml
# Для преобразования шины SPI в SpiDevice
embedded-hal-bus = "0.3.0"

# Драйвер SD-карты
embedded-sdmmc = "0.9.0"

embassy-usb-logger = "0.5.1"
log = "0.4"
```

{{#endtab }}
{{#endtabs }}

Крейт embedded-sdmmc — драйвер для чтения и записи файлов на SD-картах с файловой системой FAT.

## Дополнительные импорты

```rust
// Для SPI
use embassy_rp::spi;
use embassy_rp::spi::Spi;
use embassy_time::Delay;
use embedded_hal_bus::spi::ExclusiveDevice;

// Для вывода CS
use embassy_rp::gpio::{Level, Output};

// Для SdCard
use embedded_sdmmc::{SdCard, TimeSource, Timestamp, VolumeIdx, VolumeManager};
```

## Фиктивный TimeSource

Код файловой системы SD-карты требует реализации TimeSource, которая предоставляет отметки времени для метаданных файлов. Хотя в этом примере мы не создаём и не изменяем файлы, трейт всё равно нужно реализовать.

Поскольку временные метки здесь не важны, мы используем фиктивную реализацию, которая всегда возвращает нулевые значения.

```rust
/// Код взят из https://github.com/rp-rs/rp-hal-boards/blob/main/boards/rp-pico/examples/pico_spi_sd_card.rs
/// Фиктивный источник времени, который важен главным образом при создании файлов.
#[derive(Default)]
pub struct DummyTimesource();

impl TimeSource for DummyTimesource {
    // Теоретически здесь можно использовать RTC rp2040, если есть
    // какое-либо внешнее устройство синхронизации времени.
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

Теперь настроим периферию SPI и выводы GPIO, которые используются для связи со считывателем SD-карт.

```rust
let miso = p.PIN_4;
let cs_pin = Output::new(p.PIN_5, Level::High);
let clk = p.PIN_6;
let mosi = p.PIN_7;

let mut config = spi::Config::default();
config.frequency = 400_000;

let spi_bus = Spi::new_blocking(p.SPI0, clk, mosi, miso, config);
```

## Создание SpiDevice из шины SPI

Драйвер SD-карты ожидает SpiDevice, а не «голую» шину SPI.

```rust
let spi_device =
    ExclusiveDevice::new(spi_bus, cs_pin, Delay).expect("Failed to get exclusive device");
```

## Настройка драйвера SD-карты

Когда устройство SPI готово, можно создать экземпляр драйвера SD-карты.

```rust
let sdcard = SdCard::new(spi_device, Delay);
```

## Чтение размера SD-карты

Прежде чем открывать какие-либо файлы, мы читаем общий размер карты, чтобы убедиться, что инициализация прошла успешно.

{{#tabs global="log-method" }}
{{#tab name="Отладочный зонд" }}

```rust
defmt::info!("Init SD card controller and retrieve card size...");
let sd_size = sdcard.num_bytes().expect("failed to get sdcard size");
defmt::info!("card size is {} bytes", sd_size);
```

{{#endtab }}
{{#tab name="USB Serial" }}

```rust
log::info!("Init SD card controller and retrieve card size...");
let sd_size = sdcard.num_bytes().expect("failed to get sdcard size");
log::info!("card size is {} bytes", sd_size);
```

{{#endtab }}
{{#endtabs }}

## Открываем том и корневой каталог

Далее создаём менеджер томов и открываем первый том на карте.

```rust
let volume_mgr = VolumeManager::new(sdcard, DummyTimesource::default());
let volume0 = volume_mgr
    .open_volume(VolumeIdx(0))
    .expect("failed to open volume");

let root_dir = volume0.open_root_dir().expect("failed to open root dir");
```

## Открываем файл

Когда корневой каталог доступен, открываем файл, который хотим прочитать.

```rust
let my_file = root_dir
    .open_file_in_dir("RUST.TXT", embedded_sdmmc::Mode::ReadOnly)
    .expect("failed to open RUST.TXT file");
```

## Чтение содержимого файла

Наконец, читаем файл небольшими порциями и выводим его содержимое.

{{#tabs global="log-method" }}
{{#tab name="Отладочный зонд" }}

```rust
while !my_file.is_eof() {
    let mut buffer = [0u8; 32];

    if let Ok(n) = my_file.read(&mut buffer) {
        if let Ok(s) = core::str::from_utf8(&buffer[..n]) {
            defmt::info!("{}", s);
        } else {
            defmt::info!("{:02x}", &buffer[..n]);
        }
    }
}
```

{{#endtab }}
{{#tab name="USB Serial" }}

```rust
while !my_file.is_eof() {
    let mut buffer = [0u8; 32];

    if let Ok(n) = my_file.read(&mut buffer) {
        if let Ok(s) = core::str::from_utf8(&buffer[..n]) {
            log::info!("{}", s);
        } else {
            log::info!("{:02x}", &buffer[..n]);
        }
    }
}
```

{{#endtab }}
{{#endtabs }}

## Клонируйте существующий проект

{{#tabs global="log-method" }}
{{#tab name="Отладочный зонд" }}
Можно клонировать проект, который я создал, и перейти в папку `read-sdcard`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/sdcard/read-sdcard/
```

{{#endtab }}
{{#tab name="USB Serial" }}

Можно клонировать проект, который я создал, и перейти в папку `print-over-usb`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/sdcard/print-over-usb/
```

{{#endtab }}
{{#endtabs }}

Когда программа запущена, в выводе будет показано содержимое файла RUST.TXT.
