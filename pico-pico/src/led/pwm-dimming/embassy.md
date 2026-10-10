{{#title Плавное изменение яркости светодиода с ШИМ на Embassy для Raspberry Pi Pico 2 | impl Rust для RP2350}}

# Плавное изменение яркости светодиода на Raspberry Pi Pico с Embassy

Создадим эффект плавного изменения яркости светодиода с помощью ШИМ на Raspberry Pi Pico с Embassy.

## Создание проекта с помощью cargo-generate

К этому моменту вы уже должны знать эти шаги. Мы используем команду cargo-generate с нашим шаблоном и, когда программа попросит, выбираем Embassy в качестве HAL.

```sh
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

## Обновление импортов

Добавьте импорт ниже, чтобы подключить типы ШИМ:

```rust
use embassy_rp::pwm::{Pwm, SetDutyCycle};
```

## Инициализация ШИМ

Настроим ШИМ для светодиода. Используйте первую строку для встроенного светодиода или раскомментируйте вторую, если хотите использовать внешний светодиод на GPIO 16.

```rust
// Для встроенного светодиода
let mut pwm = Pwm::new_output_b(p.PWM_SLICE4, p.PIN_25, Default::default());

// Для внешнего светодиода, подключённого к GPIO 16
// let mut pwm = Pwm::new_output_a(p.PWM_SLICE0, p.PIN_16, Default::default());
```

## Основная логика

В главном цикле мы создаём эффект затухания и нарастания, увеличивая скважность от 0 до 100 процентов, а затем уменьшая её обратно. Небольшая задержка между шагами делает изменение яркости плавным. Вы можете изменить задержку и посмотреть, как меняется скорость изменения яркости.

```rust
loop {
    for i in 0..=100 {
        Timer::after_millis(8).await;
        let _ = pwm.set_duty_cycle_percent(i);
    }

    for i in (0..=100).rev() {
        Timer::after_millis(8).await;
        let _ = pwm.set_duty_cycle_percent(i);
    }

    Timer::after_millis(500).await;
}
```

## Полный код

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_rp::pwm::{Pwm, SetDutyCycle};
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

    // Для встроенного светодиода
    let mut pwm = Pwm::new_output_b(p.PWM_SLICE4, p.PIN_25, Default::default());

    // Для внешнего светодиода, подключённого к GPIO 16
    // let mut pwm = Pwm::new_output_a(p.PWM_SLICE0, p.PIN_16, Default::default());

    loop {
        for i in 0..=100 {
            Timer::after_millis(8).await;
            let _ = pwm.set_duty_cycle_percent(i);
        }
        for i in (0..=100).rev() {
            Timer::after_millis(8).await;
            let _ = pwm.set_duty_cycle_percent(i);
        }
        Timer::after_millis(500).await;
    }
}

// Метаданные программы для `picotool info`.
// Это не обязательно, но рекомендуется добавить эти минимальные записи.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"led-dimming"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// Конец файла
```

## Клонируйте существующий проект

Можно клонировать проект, который я создал, и перейти в папку `led-dimming`:

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/led-dimming
```
