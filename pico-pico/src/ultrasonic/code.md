{{#title Ультразвуковой датчик HC-SR04 с Embassy на Raspberry Pi Pico 2 | Учебник}}

# Учебник по Rust: использование датчика HC-SR04 с Pico 2

Мы начнём с создания нового проекта на фреймворке Embassy. После этого повторим тот же проект на `rp-hal`. Как обычно, создайте проект из шаблона с помощью `cargo-generate`:

```sh
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

Когда появится запрос, задайте проекту имя, например `bat-beacon`, и выберите `embassy` в качестве HAL. Включите логирование `defmt`, если у вас есть отладочный зонд, чтобы также видеть логи.

## Дополнительные импорты

Помимо обычных импортов шаблона, добавьте в проект эти импорты. Редактор кода должен предложить автоматический импорт для большинства из них, кроме трейта `SetDutyCycle`, который придётся добавить вручную.

```rust
// Для GPIO
use embassy_rp::gpio::{Input, Level, Output, Pull};

// Для ШИМ
use embassy_rp::pwm::{Pwm, SetDutyCycle};

// Для вычисления времени
use embassy_time::Instant;
```

Нам нужны типы GPIO для управления выводами Trigger и Echo, ШИМ для управления яркостью светодиода и утилиты времени для измерения длительности ультразвукового импульса.

## Назначение выводов GPIO

К этому моменту вы уже должны знать ШИМ из раздела о плавном изменении яркости светодиода. Здесь мы создадим похожий эффект, но есть ключевое отличие. В главе о плавном изменении яркости мы заставляли светодиод многократно плавно загораться и гаснуть с помощью условий. Здесь же яркость светодиода будет расти только тогда, когда предмет приближается к датчику.

```rust
// Для встроенного светодиода
// let mut led = Pwm::new_output_b(p.PWM_SLICE4, p.PIN_25, Default::default());

// Для внешнего светодиода, подключённого к GPIO 3
let mut led = Pwm::new_output_b(p.PWM_SLICE1, p.PIN_3, Default::default());
```

Можно использовать как встроенный светодиод, так и внешний. Мне больше нравится внешний светодиод: плавные изменения яркости видны намного лучше.

Далее инициализируем светодиод выключенным и получим его максимальное значение скважности:

```rust
led.set_duty_cycle(0)
    .expect("duty cycle is within valid range");

let max_duty = led.max_duty_cycle();
// defmt::info!("Max duty cycle {}", max_duty);
```

Скважность определяет яркость светодиода: 0 — полностью выключен, а `max_duty` — полностью включен.

## Настройка выводов Trigger и Echo

Как вы знаете, Pico должен посылать сигнал на вывод trigger, поэтому вывод GPIO 17 (подключённый к выводу trigger) мы настраиваем как выход с начальным низким уровнем. Датчик сообщает расстояние импульсами на выводе echo, то есть отправляет сигналы в Pico (вход для Pico). Поэтому вывод GPIO 16 (подключённый к выводу echo) мы настраиваем как вход.

```rust
let mut trigger = Output::new(p.PIN_17, Level::Low);
let echo = Input::new(p.PIN_16, Pull::Down);
```

## Перевод расстояния в яркость светодиода

Нам нужна функция, которая переводит измеренное расстояние в подходящее значение скважности. Чем ближе предмет, тем выше скважность (ярче светодиод):

```rust
const MAX_DISTANCE_CM: f64 = 30.0;

fn calculate_duty_cycle(distance: f64, max_duty: u16) -> u16 {
    if distance < MAX_DISTANCE_CM && distance >= 2.0 {
        let normalized = (MAX_DISTANCE_CM - distance) / MAX_DISTANCE_CM;
        // defmt::info!("duty cycle :{}", (normalized * max_duty as f64) as u16);
        (normalized * max_duty as f64) as u16
    } else {
        0
    }
}
```

Функция принимает измеренное расстояние и максимальное значение скважности. Если расстояние находится между 2 см (минимальный диапазон датчика) и 30 см, мы нормируем его к диапазону от 0 до 1 и умножаем на максимальную скважность. Предметы ближе 2 см или дальше 30 см приводят к выключению светодиода (скважность 0).

## Измерение расстояния датчиком

Расстояние будем измерять, посылая ультразвуковой импульс и засекая, сколько времени он идёт до возврата:

```rust
const ECHO_TIMEOUT: Duration = Duration::from_millis(100);

async fn measure_distance(trigger: &mut Output<'_>, echo: &Input<'_>) -> Option<f64> {
    // Отправляем импульс trigger
    trigger.set_low();
    Timer::after_micros(2).await;
    trigger.set_high();
    Timer::after_micros(10).await;
    trigger.set_low();

    // Ждём, когда echo станет HIGH (датчик отвечает)
    let timeout = Instant::now();
    while echo.is_low() {
        if timeout.elapsed() > ECHO_TIMEOUT {
            defmt::warn!("Timeout waiting for HIGH");
            return None; // Досрочный выход по таймауту
        }
    }

    let start = Instant::now();

    // Ждём, когда echo станет LOW (импульс завершён)
    let timeout = Instant::now();
    while echo.is_high() {
        if timeout.elapsed() > ECHO_TIMEOUT {
            defmt::warn!("Timeout waiting for LOW");
            return None; // Досрочный выход по таймауту
        }
    }

    let end = Instant::now();

    // Вычисляем расстояние
    let time_elapsed = end.checked_duration_since(start)?.as_micros();
    let distance = time_elapsed as f64 * 0.0343 / 2.0;

    Some(distance)
}
```

Мы начинаем с того, что ненадолго переводим вывод trigger в низкий уровень, а затем на 10 микросекунд переводим его в высокий. Это создаёт импульс trigger, который велит датчику испустить ультразвуковой пакет. После этого ждём, пока вывод Echo поднимется. Время, в течение которого вывод Echo остаётся в высоком состоянии, — это время прохождения звуковой волны туда и обратно. Используя эту длительность, вычисляем итоговое расстояние и возвращаем его.

Мы также добавили таймаут на ожидание изменения состояния вывода echo, чтобы код не завис навечно. Если вывод не отвечает за допустимое время, мы считаем попытку неудачным измерением и возвращаем `None`, что позволяет остальной программе продолжать работать как обычно.

## Главный цикл

Наконец, создадим главный цикл, который постоянно считывает датчик и обновляет светодиод:

```rust
loop {
    Timer::after_millis(10).await;

    let distance = match measure_distance(&mut trigger, &echo).await {
        Some(d) => d,
        None => {
            Timer::after_secs(5).await;
            continue; // Переходим к следующей итерации
        }
    };

    let duty_cycle = calculate_duty_cycle(distance, max_duty);
    led.set_duty_cycle(duty_cycle)
        .expect("duty cycle is within valid range");
}
```

Каждые 10 миллисекунд мы измеряем расстояние. Если измерение прошло успешно, вычисляем подходящую яркость светодиода и применяем её. Если измерение не удалось (из-за таймаута или проблем с датчиком), ждём 5 секунд перед следующей попыткой.

## Полный код

Вот всё вместе:

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_time::{Duration, Timer};

//Обработчик паники
use panic_probe as _;
// Логирование через defmt
use defmt_rtt as _;

// Для GPIO
use embassy_rp::gpio::{Input, Level, Output, Pull};

// Для ШИМ
use embassy_rp::pwm::{Pwm, SetDutyCycle};

// Для вычисления времени
use embassy_time::Instant;

/// Сообщаем Boot ROM о нашем приложении
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Для встроенного светодиода
    // let mut led = Pwm::new_output_b(p.PWM_SLICE4, p.PIN_25, Default::default());

    // Для внешнего светодиода, подключённого к GPIO 3
    let mut led = Pwm::new_output_b(p.PWM_SLICE1, p.PIN_3, Default::default());

    let mut trigger = Output::new(p.PIN_17, Level::Low);
    let echo = Input::new(p.PIN_16, Pull::None);

    led.set_duty_cycle(0)
        .expect("duty cycle is within valid range");

    let max_duty = led.max_duty_cycle();
    // defmt::info!("Max duty cycle {}", max_duty);

    loop {
        Timer::after_millis(10).await;

        let distance = match measure_distance(&mut trigger, &echo).await {
            Some(d) => d,
            None => {
                Timer::after_secs(5).await;
                continue; // Переходим к следующей итерации
            }
        };

        let duty_cycle = calculate_duty_cycle(distance, max_duty);
        led.set_duty_cycle(duty_cycle)
            .expect("duty cycle is within valid range");
    }
}

const ECHO_TIMEOUT: Duration = Duration::from_millis(100);

async fn measure_distance(trigger: &mut Output<'_>, echo: &Input<'_>) -> Option<f64> {
    // Отправляем импульс trigger
    trigger.set_low();
    Timer::after_micros(2).await;
    trigger.set_high();
    Timer::after_micros(10).await;
    trigger.set_low();

    // Ждём, когда echo станет HIGH (датчик отвечает)
    let timeout = Instant::now();
    while echo.is_low() {
        if timeout.elapsed() > ECHO_TIMEOUT {
            defmt::warn!("Timeout waiting for HIGH");
            return None; // Досрочный выход по таймауту
        }
    }

    let start = Instant::now();

    // Ждём, когда echo станет LOW (импульс завершён)
    let timeout = Instant::now();
    while echo.is_high() {
        if timeout.elapsed() > ECHO_TIMEOUT {
            defmt::warn!("Timeout waiting for LOW");
            return None; // Досрочный выход по таймауту
        }
    }

    let end = Instant::now();

    // Вычисляем расстояние
    let time_elapsed = end.checked_duration_since(start)?.as_micros();
    let distance = time_elapsed as f64 * 0.0343 / 2.0;

    Some(distance)
}

const MAX_DISTANCE_CM: f64 = 30.0;

fn calculate_duty_cycle(distance: f64, max_duty: u16) -> u16 {
    if distance < MAX_DISTANCE_CM && distance >= 2.0 {
        let normalized = (MAX_DISTANCE_CM - distance) / MAX_DISTANCE_CM;
        // defmt::info!("duty cycle :{}", (normalized * max_duty as f64) as u16);
        (normalized * max_duty as f64) as u16
    } else {
        0
    }
}

// Метаданные программы для `picotool info`.
// Это не обязательно, но рекомендуется добавить эти минимальные записи.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"ultrasonic"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// Конец файла
```

## Клонируйте существующий проект

Можно клонировать проект, который я создал, и перейти в папку `ultrasonic`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/ultrasonic
```
