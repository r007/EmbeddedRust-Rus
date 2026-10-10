{{#title Пример мигания внешним светодиодом на Embassy для Raspberry Pi Pico 2 | impl Rust для RP2350}}

# Мигание внешним светодиодом на Raspberry Pi Pico на embedded Rust

Начнём с создания проекта. Будем использовать cargo-generate и шаблон, который мы подготовили для этой книги.

В терминале введите:

```sh
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

Программа задаст несколько вопросов:

1. Для имени проекта можно выбрать любое. Мы будем использовать `external-led`.

2. Затем она спросит «Which HAL?» (какой HAL использовать). Нужно выбрать `embassy`.

3. Потом предложит включить логирование `defmt`. Оно работает только при использовании отладочного зонда, поэтому выбирайте в зависимости от своей настройки. В этом упражнении мы всё равно не будем писать логи.

## Импорты

Большинство необходимых импортов уже есть в шаблоне проекта. Для этого упражнения нужно добавить только структуру `Output` и перечисление `Level` из модуля gpio:

```rust
use embassy_rp::gpio::{Level, Output};
```

Пока вы пишете основной код, редактор обычно сам предлагает недостающие импорты. Если подсказки нет или возникла ошибка, загляните в раздел с полным кодом и добавьте недостающие импорты оттуда.

## Основная логика

Код почти такой же, как в примере быстрого старта. Единственное отличие — мы используем GPIO 13 вместо GPIO 25. К GPIO 13 подключён светодиод (через резистор).

Добавим этот код в функцию main:

```rust
let mut led = Output::new(p.PIN_13, Level::Low);

loop {
    led.set_high(); // Включить светодиод
    Timer::after_millis(500).await;

    led.set_low(); // Выключить светодиод
    Timer::after_millis(500).await;
}
```

Здесь мы используем структуру Output, потому что хотим отправлять сигналы с Pico на светодиод. Настраиваем GPIO 13 как выходной вывод и запускаем его в низком состоянии (выключено).

> [!NOTE]
> Если вы хотите считывать сигналы с компонента (например, кнопки или датчика), вывод GPIO нужно настроить как Input.

Затем мы вызываем `set_high()` и `set_low()` для вывода с задержкой между ними. Это переключает вывод между высоким и низким уровнями, а значит — включает и выключает светодиод.

## Полный код

Вот полный код для справки:

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;

//Обработчик паники
use panic_probe as _;
// Логирование через defmt
use defmt_rtt as _;

/// Сообщаем Boot ROM о нашем приложении
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut led = Output::new(p.PIN_13, Level::Low);

    loop {
        led.set_high(); // Включить светодиод
        Timer::after_millis(500).await;

        led.set_low(); // Выключить светодиод
        Timer::after_millis(500).await;
    }
}

// Метаданные программы для `picotool info`.
// Это не обязательно, но рекомендуется добавить эти минимальные записи.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"external-led"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// Конец файла
```

## Клонируйте существующий проект

Можно клонировать проект, который я создал, и перейти в папку `external-led`:

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/external-led
```

## Как запустить?

См. раздел [«Запуск программы»](../running.md).
