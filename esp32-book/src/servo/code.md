{{#title Плавный поворот сервопривода с ESP32 на Rust}}

# Пишем код на Rust для плавного поворота сервопривода с ESP32

## Создание проекта с помощью esp-generate

Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 servo-motor
```
Откроется экран с вопросом о выборе опций. В последних версиях esp-hal для ledc нужно явно включить нестабильные возможности.

- Поэтому выберите опцию «Enable unstable HAL features» («Включить нестабильные возможности HAL»).

Затем сохраните настройки, нажав на клавиатуре «s».


## Обновление зависимостей
Добавьте в файл `Cargo.toml` следующий крейт:
```toml
embedded-hal = "1.0.0"
```

Затем импортируйте трейт `SetDutyCycle` в файле `main.rs`. Он нужен, если мы хотим использовать функции `max_duty_cycle` и `set_duty_cycle`:
```rust
use embedded_hal::pwm::SetDutyCycle;
```

## Таймер и канал ШИМ

Инициализируем таймер с частотой 50 Гц и 12-битным разрешением и настроим канал.

```rust
let mut servo = peripherals.GPIO33;
let ledc = Ledc::new(peripherals.LEDC);

let mut hstimer0 = ledc.timer::<HighSpeed>(timer::Number::Timer0);
hstimer0
    .configure(timer::config::Config {
        duty: timer::config::Duty::Duty12Bit,
        clock_source: timer::HSClockSource::APBClk,
        frequency: Rate::from_hz(50),
    })
    .unwrap();

let mut channel0 = ledc.channel(channel::Number::Channel0, servo.reborrow());
channel0
    .configure(channel::config::Config {
        timer: &hstimer0,
        duty_pct: 10,
        drive_mode: DriveMode::PushPull,
    })
    .unwrap();
```

## Вспомогательная функция

Код из предыдущей главы мы уже разобрали.

```rust
    let max_duty_cycle = channel0.max_duty_cycle() as u32;

    // Минимальный коэффициент заполнения (2,5%)
    // Для 12 бит -> 25 * 4096 /1000 => ~ 102
    let min_duty = (25 * max_duty_cycle) / 1000;
    // Максимальный коэффициент заполнения (12,5%)
    // Для 12 бит -> 125 * 4096 /1000 => 512
    let max_duty = (125 * max_duty_cycle) / 1000;
    // 512 - 102 => 410
    let duty_gap = max_duty - min_duty;
    
    fn duty_from_angle(deg: u32, min_duty: u32, duty_gap: u32) -> u16 {
        let duty = min_duty + ((deg * duty_gap) / 180);
        duty as u16
    }
```

## Поворот

В главном цикле мы сначала поворачиваем сервопривод с 0 до 180 градусов. Между шагами делаем паузу 10 мс: сервопривод успевает дойти до положения, потому что движемся маленькими шагами. Затем с помощью `rev()` меняем диапазон на обратный, чтобы вернуться от 180 к 0.

```rust
loop {
    for deg in 0..=180 {
        let duty = duty_from_angle(deg, min_duty, duty_gap);
        channel0.set_duty_cycle(duty).unwrap();
        delay.delay_millis(10);
    }
    delay.delay_millis(500);

    for deg in (0..=180).rev() {
        let duty = duty_from_angle(deg, min_duty, duty_gap);
        channel0.set_duty_cycle(duty).unwrap();
        delay.delay_millis(10);
    }
    delay.delay_millis(500);
}
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

use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;

// LEDC
use esp_hal::gpio::DriveMode;
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::ledc::timer::TimerIFace;
use esp_hal::ledc::{HighSpeed, Ledc, channel, timer};
use esp_hal::time::Rate;

use embedded_hal::pwm::SetDutyCycle;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Создаёт дескриптор приложения по умолчанию, который требуется загрузчику esp-idf.
// Подробнее см.: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 1.0.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut servo = peripherals.GPIO33;
    let ledc = Ledc::new(peripherals.LEDC);

    let mut hstimer0 = ledc.timer::<HighSpeed>(timer::Number::Timer0);
    hstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty12Bit,
            clock_source: timer::HSClockSource::APBClk,
            frequency: Rate::from_hz(50),
        })
        .unwrap();

    let mut channel0 = ledc.channel(channel::Number::Channel0, servo.reborrow());
    channel0
        .configure(channel::config::Config {
            timer: &hstimer0,
            duty_pct: 10,
            drive_mode: DriveMode::PushPull,
        })
        .unwrap();

    let delay = Delay::new();

    let max_duty_cycle = channel0.max_duty_cycle() as u32;

    // Минимальный коэффициент заполнения (2,5%)
    // Для 12 бит -> 25 * 4096 /1000 => ~ 102
    let min_duty = (25 * max_duty_cycle) / 1000;
    // Максимальный коэффициент заполнения (12,5%)
    // Для 12 бит -> 125 * 4096 /1000 => 512
    let max_duty = (125 * max_duty_cycle) / 1000;
    // 512 - 102 => 410
    let duty_gap = max_duty - min_duty;

    loop {
        for deg in 0..=180 {
            let duty = duty_from_angle(deg, min_duty, duty_gap);
            channel0.set_duty_cycle(duty).unwrap();
            delay.delay_millis(10);
        }
        delay.delay_millis(500);

        for deg in (0..=180).rev() {
            let duty = duty_from_angle(deg, min_duty, duty_gap);
            channel0.set_duty_cycle(duty).unwrap();
            delay.delay_millis(10);
        }
        delay.delay_millis(500);
    }
}

fn duty_from_angle(deg: u32, min_duty: u32, duty_gap: u32) -> u16 {
    let duty = min_duty + ((deg * duty_gap) / 180);
    duty as u16
}
```


## Клонирование существующего проекта
Вы можете клонировать (или посмотреть) созданный мной проект и перейти в папку `servo-motor`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/servo-motor
```
