# Настройка проекта

## Создание проекта

Начнём с создания минимального проекта в качестве отправной точки. Для таких случаев мы предоставляем инструмент [`esp-generate`][esp-generate]. Его можно установить командой:

```shell
cargo install --locked esp-generate
```

После установки `esp-generate` создадим пустой проект:

```shell
esp-generate --headless --chip=esp32c3 -o unstable-hal -o embassy no_std-training
```

Поскольку мы работаем с отладочной платой [ESP32-C3-DevKit-RUST-2], выбираем чип ESP32-C3. Мы будем использовать асинхронную поддержку через [Embassy], поэтому нужны обе опции: `embassy` и `unstable`.

Теперь в текущем каталоге должен появиться каталог `no_std-training/`.

[esp-generate]: https://github.com/esp-rs/esp-generate/
[esp32-c3-devkit-rust-2]: https://github.com/esp-rs/esp-rust-board
[Embassy]: https://github.com/embassy-rs/embassy

## Структура проекта

Сгенерированный проект должен выглядеть так:

```text
no_std-training/
├─ .cargo/
│  └─ config.toml
├─ .clippy.toml
├─ build.rs
├─ Cargo.toml
├─ rust-toolchain.toml
└─ src/
   ├─ bin/
   │   └─ main.rs
   └─ lib.rs
```

Рассмотрим, за что отвечает каждый из этих файлов:

- [`.cargo/config.toml`] содержит конфигурацию Cargo.
  - Указана [целевая платформа][build target], в данном случае `riscv32imc-unknown-none-elf`.
  - Переданы флаги компилятора Rust, которые принудительно включают frame pointers. Это необходимо для корректной работы бэктрейсов. Если вы не используете крейт `esp-backtrace`, это не требуется.
  - Кроме того, мы настраиваем [espflash] как раннер Cargo для проекта.
- [`.clippy.toml`] содержит конфигурацию [Clippy], линтера для кода на Rust.
- [`build.rs`] настраивает компилятор Rust так, чтобы подключались необходимые скрипты компоновщика.
  - Это можно было бы задать и в `.cargo/config.toml`, но такой подход даёт более понятные диагностические сообщения при ошибках сборки.
- `Cargo.toml` — [манифест Cargo][Cargo manifest], в котором хранятся метаданные проекта, зависимости и другая информация.
- [`rust-toolchain.toml`] задаёт, какой тулчейн Rust использовать, а также указывает нужные компоненты и целевую платформу.
- `src/lib.rs` содержит библиотечный код приложения. Сейчас он пуст, но будет наполнен в следующих главах.
- `src/bin/main.rs` — точка входа в приложение. Здесь будет находиться большая часть логики.

[`.cargo/config.toml`]: https://doc.rust-lang.org/cargo/reference/config.html
[build target]: https://doc.rust-lang.org/beta/rustc/targets/index.html
[espflash]: https://github.com/esp-rs/espflash/
[`.clippy.toml`]: https://doc.rust-lang.org/clippy/configuration.html
[Clippy]: https://doc.rust-lang.org/stable/clippy/index.html
[`build.rs`]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[Cargo manifest]: https://doc.rust-lang.org/cargo/reference/manifest.html
[`rust-toolchain.toml`]: https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file

## Зависимости

Сгенерированный `Cargo.toml` уже содержит ряд зависимостей. Их назначение кратко описано ниже:

- [`esp-hal`][esp-hal] — слой абстракции аппаратного обеспечения (HAL). Предоставляет доступ к встроенной периферии через высокоуровневые API.
- [`esp-rtos`][esp-rtos] — поддержка среды выполнения, необходимая для запуска асинхронного кода поверх `esp-hal`.
- [`esp-bootloader-esp-idf`][esp-bootloader-esp-idf] — поддержка загрузчика ESP-IDF.
- [`embassy-executor`][embassy-executor] — асинхронный исполнитель для встраиваемых устройств. Позволяет запускать одну или несколько асинхронных задач.
- [`embassy-time`][embassy-time] — асинхронная работа со временем: задержки и таймауты.
- [`critical-section`][critical-section] — критические секции для встраиваемых устройств.
- [`static_cell`][static_cell] — статически выделяемая ячейка, которая инициализируется во время выполнения.

Дополнительные зависимости будут добавлены и в последующих главах.

[esp-hal]: https://docs.espressif.com/projects/rust/esp-hal/1.1.1/esp32c3/esp_hal/index.html
[esp-rtos]: https://docs.espressif.com/projects/rust/esp-rtos/0.3.0/esp32c3/esp_rtos/index.html
[esp-bootloader-esp-idf]: https://docs.espressif.com/projects/rust/esp-bootloader-esp-idf/0.5.0/esp32c3/esp_bootloader_esp_idf/index.html
[embassy-executor]: https://docs.rs/embassy-executor/latest/embassy_executor/
[embassy-time]: https://docs.rs/embassy-time/latest/embassy_time/
[critical-section]: https://docs.rs/critical-section/latest/critical_section/
[static_cell]: https://docs.rs/static_cell/latest/static_cell/

## Минимальное приложение

Файл `src/bin/main.rs` содержит шаблонный код минимального приложения:

```rust
#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Создаёт дескриптор приложения по умолчанию, который требуется загрузчику ESP-IDF.
// Подробнее см.: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32c3/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    // TODO: запустить задачи
    let _ = spawner;

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }

    // для вдохновения посмотрите примеры по адресу https://github.com/esp-rs/esp-hal/tree/main/examples
}
```

В целом код понятен, однако есть несколько интересных моментов, которые мы кратко объясним.

### Application Description

Загрузчик ESP-IDF ожидает, что структура Application Description находится по определённому адресу. Если её не добавить, вы получите запутанные ошибки загрузчика.

Application Description создаётся с помощью такого вызова макроса:

```rust
esp_bootloader_esp_idf::esp_app_desc!();
```

Больше ничего делать не нужно. Подробнее см. в [документации ESP-IDF][ESP-IDF Documentation].

[ESP-IDF Documentation]: https://docs.espressif.com/projects/esp-idf/en/stable/esp32c3/api-reference/system/app_image_format.html#application-description

### Макрос `#[esp_rtos::main]`

Макрос [`#[esp_rtos::main]`][main] определяет точку входа приложения, обычно это функция `main`:

```rust
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
  // ...
}
```

Этот макрос создаёт новый экземпляр исполнителя Embassy `Executor` и запускает функцию `main` как асинхронную задачу.

[main]: https://docs.espressif.com/projects/rust/esp-rtos/0.3.0/esp32c3/esp_rtos/attr.main.html
