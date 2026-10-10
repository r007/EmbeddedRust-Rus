{{#title Управление сервоприводом с ШИМ на Embassy для Raspberry Pi Pico 2}}

# Управление сервоприводом на Raspberry Pi Pico с Embassy и Rust

В этом разделе мы создадим простую программу, которая переводит рычаг сервопривода из 0 в 90, затем в 180 и обратно в 0. Такого базового движения достаточно, чтобы понять, как ШИМ управляет сервоприводом. Когда идея станет понятна, можно экспериментировать дальше и создавать более интересные приложения.

Начнём с создания нового проекта на фреймворке Embassy. После этого мы повторим тот же проект на `rp-hal`. Как обычно, создайте проект из шаблона с помощью `cargo-generate`:

```sh
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

Когда появится запрос, задайте проекту имя, например «servo-motor», и выберите «embassy» в качестве HAL. Включите логирование `defmt`, если у вас есть отладочный зонд, чтобы также видеть логи.

## Дополнительные импорты

Помимо обычных импортов шаблона, добавьте в проект эти специфические импорты.

```rust
// Для ШИМ
use embassy_rp::pwm::{Config as PwmConfig, Pwm, SetDutyCycle};
```

## Настройка ШИМ

В главе о плавном изменении яркости светодиода мы оставляли настройки ШИМ по умолчанию. Там этого было достаточно, потому что важна была только скважность.

На этот раз так поступить нельзя. Для управления сервоприводом нужно самим задать значения TOP и делителя, чтобы частота ШИМ получилась 50 Гц, на основе значений, которые мы вычислили ранее.

Здесь я использую вручную вычисленные значения TOP и делителя прямо в коде, а не калькулятор-форму. Делитель, который я использую, — целое число, поэтому его можно просто преобразовать методом `into()`. Если бы у делителя была дробная часть, пришлось бы использовать крейт `fixed`, который мы рассматривали ранее. Чтобы всё было просто, пока остаёмся на целой версии.

```rust
const PWM_DIV_INT: u8 = 64;
const PWM_TOP: u16 = 46_874;
```

> [!Note]
> Можно попробовать и с дробным делителем. Мы уже рассматривали фрагмент кода для этого ранее, так что его можно повторно использовать и поэкспериментировать с дробными значениями.

Когда значения готовы, применяем их к конфигурации ШИМ:

```rust
let mut servo_config: PwmConfig = Default::default();
servo_config.top = PWM_TOP;
servo_config.divider = PWM_DIV_INT.into();
```

## Инициализация ШИМ

Когда конфигурация ШИМ готова, следующий шаг — создать выход ШИМ и привязать его к выводу GPIO, подключённому к сигнальному проводу сервопривода.

В нашем случае мы используем GPIO 15. Смело меняйте это, если ваша проводка отличается.

```rust
let mut servo = Pwm::new_output_b(p.PWM_SLICE7, p.PIN_15, servo_config);
```

## Главный цикл

Теперь переходим к главному циклу. Здесь мы просто меняем значение скважности, ждём короткую задержку, а затем переходим к следующему положению.

```rust
loop {
    // Переместить сервопривод в положение 0° (скважность 2,5 % = 25/1000)
    servo
        .set_duty_cycle_fraction(25, 1000)
        .expect("invalid min duty cycle");

    Timer::after_millis(1000).await;

    // Положение 90° (скважность 7,5 %)
    servo
        .set_duty_cycle_fraction(75, 1000)
        .expect("invalid half duty cycle");

    Timer::after_millis(1000).await;

    // Положение 180° (скважность 12 %)
    servo
        .set_duty_cycle_fraction(120, 1000)
        .expect("invalid max duty cycle");

    Timer::after_millis(1000).await;
}
```

Если всё работает, вы увидите, как рычаг сервопривода переходит в первое положение, ненадолго замирает, переходит в следующее, затем в последнее и потом возвращается обратно.

## Клонируйте существующий проект

Можно клонировать проект, который я создал, и перейти в папку `servo-motor`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/servo-motor
```

## Отладка

Если сервопривод не двигается, начните с проверки проводки. Убедитесь, что сигнальный провод подключён к правильному выводу GPIO, сервопривод получает нормальное питание и земля общая с Pico.

Затем дважды проверьте, что код был прошит правильно и программа действительно запущена на плате. Если вы используете отладочный зонд с включённым `defmt`, вывод логов поможет это подтвердить.

Если всё выглядит правильно, но сервопривод всё равно не двигается как ожидается, вероятнее всего, ваш сервопривод использует немного другие ширины импульсов для каждого положения. В этом случае обратитесь к даташиту для вашей модели сервопривода или проверьте сайт производителя или продавца, если там есть данные о таймингах. Возможно, придётся подстроить значения скважности под ваш сервопривод.

Не расстраивайтесь, если с первого раза всё не получится идеально. С этим я тоже бился, когда только начинал. Я постарался понятно объяснить расчёты и логику, стоящую за ними. Надеюсь, это поможет.

## Полный код

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_time::Timer;

//Обработчик паники
use panic_probe as _;
// Логирование через defmt
use defmt_rtt as _;

// ШИМ
use embassy_rp::pwm::{Config as PwmConfig, Pwm, SetDutyCycle};

/// Сообщаем Boot ROM о нашем приложении
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

const PWM_DIV_INT: u8 = 64;
const PWM_TOP: u16 = 46_874;

// Альтернативный метод:
// const TOP: u16 = PWM_TOP + 1;
// const MIN_DUTY: u16 = (TOP as f64 * (2.5 / 100.)) as u16;
// const HALF_DUTY: u16 = (TOP as f64 * (7.5 / 100.)) as u16;
// const MAX_DUTY: u16 = (TOP as f64 * (12. / 100.)) as u16;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut servo_config: PwmConfig = Default::default();
    servo_config.top = PWM_TOP;
    servo_config.divider = PWM_DIV_INT.into();

    let mut servo = Pwm::new_output_b(p.PWM_SLICE7, p.PIN_15, servo_config);

    loop {
        // Переместить сервопривод в положение 0° (скважность 2,5 % = 25/1000)
        servo
            .set_duty_cycle_fraction(25, 1000)
            .expect("invalid min duty cycle");
        Timer::after_millis(1000).await;

        // Положение 90° (скважность 7,5 %)
        servo
            .set_duty_cycle_fraction(75, 1000)
            .expect("invalid half duty cycle");
        Timer::after_millis(1000).await;

        // Положение 180° (скважность 12 %)
        servo
            .set_duty_cycle_fraction(120, 1000)
            .expect("invalid max duty cycle");
        Timer::after_millis(1000).await;
    }

    // Альтернативный метод
    // loop {
    //     servo
    //         .set_duty_cycle(MIN_DUTY)
    //         .expect("invalid min duty cycle");
    //     Timer::after_millis(1000).await;

    //     servo
    //         .set_duty_cycle(HALF_DUTY)
    //         .expect("invalid half duty cycle");
    //     Timer::after_millis(1000).await;

    //     servo
    //         .set_duty_cycle(MAX_DUTY)
    //         .expect("invalid max duty cycle");
    //     Timer::after_millis(1000).await;
    // }
}

// Метаданные программы для `picotool info`.
// Это не обязательно, но рекомендуется добавить эти минимальные записи.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"servo-motor"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// Конец файла
```
