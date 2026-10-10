{{#title Управление светодиодом кнопкой на Raspberry Pi Pico 2 | Embedded Rust}}

# Светодиод по нажатию кнопки

Создадим простой проект, который включает светодиод, когда нажата кнопка. Можно использовать внешний светодиод или встроенный. Просто измените номер вывода светодиода в коде, чтобы он соответствовал тому, который вы используете.

<div class="image-with-caption" style="text-align:center; ">
    <img src="./images/button-with-raspberry-pi-pico.png" alt="Кнопка с Raspberry Pi Pico 2" style="height:auto; display:block; margin:0 auto;"/>
    <div class="caption" style="font-size:0.9em; color:#555; margin-top:6px;">Кнопка с Raspberry Pi Pico 2</div>
</div>

Начнём с создания нового проекта с помощью cargo generate и нашего шаблона.

В терминале введите:

```sh
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

## Кнопка как вход

До сих пор мы использовали структуру `Output`, потому что Pico отправлял сигналы светодиоду. На этот раз Pico будет получать сигнал от кнопки, поэтому настроим вывод как `Input`.

```rust
let button = Input::new(p.PIN_15, Pull::Up);
```

Мы подключили одну сторону кнопки к GPIO 15, а другую — к земле. Это значит, что при нажатии кнопки вывод притягивается к низкому уровню (LOW). Как мы обсуждали ранее, без подтягивающего резистора вход окажется в плавающем состоянии и будет выдавать ненадёжные значения. Поэтому мы включаем внутренний подтягивающий резистор (pull-up), чтобы вывод по умолчанию оставался в высоком уровне (HIGH).

## Светодиод как выход

Настраиваем вывод светодиода как выход, начиная с низкого уровня (выключено). Если вы используете внешний светодиод, раскомментируйте первую строку для GPIO 16. Если используете встроенный светодиод Pico, используйте GPIO 25, как показано. Главное — чтобы ваша схема соответствовала выбранному выводу.

```rust
// let mut led = Output::new(p.PIN_16, Level::Low);
let mut led = Output::new(p.PIN_25, Level::Low);
```

## Главный цикл

Теперь в цикле мы постоянно проверяем, нажата ли кнопка, — то есть находится ли вывод в низком состоянии. Между проверками добавляем небольшую задержку 5 миллисекунд, чтобы не перегружать систему. Когда кнопка показывает LOW (нажата), мы переводим вывод светодиода в HIGH, чтобы включить его, и ждём 3 секунды, чтобы успеть его заметить. Задержку можно изменить по своему усмотрению.

```rust
loop {

    if button.is_low() {
        defmt::info!("Button pressed");
        led.set_high();
        Timer::after_secs(3).await;
    } else {
        led.set_low();
    }

    Timer::after_millis(5).await;
    
}
```

> [!NOTE]
> **Дребезг контактов (debounce):** если уменьшить задержку, можно заметить, что одно нажатие иногда даёт несколько срабатываний. Это называется «дребезгом кнопки». Когда вы нажимаете физическую кнопку, металлические контакты внутри на мгновение бьются друг о друга, создавая несколько электрических сигналов за несколько миллисекунд. В этом примере 3-секундная задержка светодиода скрывает эффект дребезга, но в приложениях, где нужно точно считать отдельные нажатия, потребуется логика устранения дребезга (debouncing).

Мы также выводим сообщение «Button pressed» с помощью `defmt`. Если вы используете отладочный зонд, чтобы увидеть эти логи в терминале, выполните команду `cargo embed --release`.

## Полный код

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::Pull;
use embassy_rp::{
    self as hal,
    gpio::{Input, Level, Output},
};
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

    let button = Input::new(p.PIN_15, Pull::Up);
    // let mut led = Output::new(p.PIN_16, Level::Low);
    let mut led = Output::new(p.PIN_25, Level::Low);

    loop {
        if button.is_low() {
            defmt::info!("Button pressed");
            led.set_high();
            Timer::after_secs(3).await;
        } else {
            led.set_low();
        }
        Timer::after_millis(5).await;
    }
}

// Метаданные программы для `picotool info`.
// Это не обязательно, но рекомендуется добавить эти минимальные записи.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"button"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// Конец файла
```

## Клонируйте существующий проект

Можно клонировать (или просто посмотреть) проект, который я создал, и перейти в папку `button`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/button
```
