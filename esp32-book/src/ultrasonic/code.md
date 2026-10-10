{{#title Пишем код на Rust для ультразвукового датчика HC-SR04 с ESP32}}

# Пишем код на Rust для ультразвукового датчика HC-SR04 с ESP32

Мы начнём с создания проекта по шаблону, а затем изменим код под требования текущего проекта.


### Создание проекта с помощью esp-generate

Этот шаг вы уже выполнили в разделе «Быстрый старт».

Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 ultrasonic
```

Откроется экран с вопросом о выборе опций. В последних версиях esp-hal для ledc нужно явно включить нестабильные возможности.

- Поэтому выберите опцию «Enable unstable HAL features» («Включить нестабильные возможности HAL»).

Затем сохраните настройки, нажав на клавиатуре «s».

## Настройка вывода светодиода и ШИМ
Этот код вы уже должны понимать. Если нет, сначала пройдите раздел о [плавном затухании светодиода](../led/index.md).

Краткое напоминание: здесь мы настраиваем ШИМ для светодиода, что позволяет управлять яркостью, меняя коэффициент заполнения.

```rust
// let led = peripherals.GPIO2; // uses onboard LED
let led = peripherals.GPIO33;

// Настраиваем LEDC
let mut ledc = Ledc::new(peripherals.LEDC);
ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
lstimer0
    .configure(timer::config::Config {
        duty: timer::config::Duty::Duty5Bit,
        clock_source: timer::LSClockSource::APBClk,
        frequency: Rate::from_khz(24),
    })
    .unwrap();

let mut channel0 = ledc.channel(channel::Number::Channel0, led);
channel0
    .configure(channel::config::Config {
        timer: &lstimer0,
        duty_pct: 10,
        drive_mode: DriveMode::PushPull,
    })
    .unwrap();
```

## Настройка вывода Trig
Мы настроим GPIO 5 как выходной вывод с начальным состоянием LOW. Если интересно, почему он выходной: мы отправляем сигнал с ESP32 на ультразвуковой модуль. Этот вывод подключён к выводу Trig модуля.

```rust
let mut trig = Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default());
```

## Настройка вывода Echo
Мы настроим GPIO 18 как входной вывод, так как ультразвуковой модуль возвращает сигнал на ESP32. Начальное состояние этого вывода задаём как Pull Down, чтобы он начинал работу в низком состоянии.

```rust
let echo = Input::new(
    peripherals.GPIO18,
    InputConfig::default().with_pull(Pull::Down),
);
```

## 🦇 Включаем

### Шаг 1: отправляем импульс запуска
Сначала переводим вывод trig в LOW, чтобы начать с чистого состояния. Затем переводим вывод trig в HIGH на 10 микросекунд и снова в LOW. Это заставит модуль отправить ультразвуковые волны.

```rust
// Перед стартом убеждаемся, что вывод Trigger в низком состоянии
trig.set_low();
delay.delay_micros(2);

// Отправляем высокий импульс длительностью 10 микросекунд
trig.set_high();
delay.delay_micros(10);
trig.set_low();
```

### Шаг 2: измеряем ширину импульса

Далее используем два цикла. Первый выполняется, пока вывод echo находится в LOW. Как только он переходит в HIGH, мы записываем текущее время в переменную. Затем запускаем второй цикл, который продолжается, пока вывод echo остаётся в HIGH. Когда он возвращается в LOW, записываем текущее время в другую переменную. Разность этих двух значений даёт ширину импульса.

```rust
// Измеряем, как долго сигнал остаётся в высоком состоянии
while echo.is_low() {}
let time1 = rtc.current_time_us();
while echo.is_high() {}
let time2 = rtc.current_time_us();
let pulse_width = time2 - time1;
```

### Шаг 3: вычисляем расстояние

Чтобы вычислить расстояние, нужна ширина импульса. Она показывает, сколько времени ультразвуковым волнам понадобилось, чтобы дойти до препятствия и вернуться. Поскольку импульс отражает время туда и обратно, делим его на 2, чтобы учесть путь до препятствия и обратно.

Скорость звука в воздухе примерно равна 0,0343 см на микросекунду. Умножив время (в микросекундах) на это значение и разделив на 2, получим расстояние до препятствия в сантиметрах.

```rust
let distance = (pulse_width * 0.0343) / 2.0;
```

### Шаг 4: коэффициент заполнения ШИМ для светодиода
Наконец, регулируем яркость светодиода в зависимости от измеренного расстояния.

Коэффициент заполнения рассчитывается по нашей собственной логике, её можно изменить под свои задачи. Когда объект ближе 30 см, яркость светодиода растёт. Чем ближе объект к ультразвуковому модулю, тем выше получается рассчитанное отношение, а значит, тем сильнее меняется коэффициент заполнения. В результате яркость светодиода постепенно увеличивается по мере приближения объекта к датчику.

```rust
let duty_pct: u8 = if distance < 30.0 {
    let ratio = (30.0 - distance) / 30.0;
    let p = (ratio * 100.0) as u8;
    p.min(100)
} else {
    0
};

if let Err(e) = channel0.set_duty(duty_pct) {
    esp_println::println!("Failed to set duty cycle: {:?}", e);
}
```

### Полный код

```rust
#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::clock::CpuClock;
use esp_hal::main;

// LEDC
use esp_hal::gpio::DriveMode;
use esp_hal::gpio::{InputConfig, OutputConfig};
use esp_hal::ledc::{LSGlobalClkSource, LowSpeed};
use esp_hal::time::Rate;

use esp_hal::{
    delay::Delay,
    gpio::{Input, Level, Output, Pull},
    ledc::{
        Ledc,
        channel::{self, ChannelIFace},
        timer::{self, TimerIFace},
    },
    rtc_cntl::Rtc,
};

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

    // let led = peripherals.GPIO2; // использует встроенный светодиод
    let led = peripherals.GPIO33;

    // Настраиваем LEDC
    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty5Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(24),
        })
        .unwrap();

    let mut channel0 = ledc.channel(channel::Number::Channel0, led);
    channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            drive_mode: DriveMode::PushPull,
        })
        .unwrap();

    // Для ультразвукового датчика HC-SR04
    let mut trig = Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default());
    let echo = Input::new(
        peripherals.GPIO18,
        InputConfig::default().with_pull(Pull::Down),
    );

    let delay = Delay::new(); // Можно использовать, так как включены нестабильные возможности

    let rtc = Rtc::new(peripherals.LPWR);

    loop {
        delay.delay_millis(5);

        // Отправляем ультразвуковые волны
        trig.set_low();
        delay.delay_micros(2);
        trig.set_high();
        delay.delay_micros(10);
        trig.set_low();

        // Измеряем, как долго сигнал остаётся в высоком состоянии
        while echo.is_low() {}
        let time1 = rtc.current_time_us();
        while echo.is_high() {}
        let time2 = rtc.current_time_us();
        let pulse_width = time2 - time1;

        // Вычисляем расстояние по ширине импульса
        let distance = (pulse_width as f64 * 0.0343) / 2.0;
        // esp_println::println!("Pulse Width: {}", pulse_width);
        // esp_println::println!("Distance: {}", distance);

        // Наша собственная логика расчёта коэффициента заполнения по расстоянию
        let duty_pct: u8 = if distance < 30.0 {
            let ratio = (30.0 - distance) / 30.0;
            let p = (ratio * 100.0) as u8;
            p.min(100)
        } else {
            0
        };

        if let Err(e) = channel0.set_duty(duty_pct) {
            // esp_println::println!("Failed to set duty cycle: {:?}", e);
            panic!("Failed to set duty cycle: {:?}", e);
        }

        delay.delay_millis(60);
    }
}
```

## Клонирование существующего проекта
Вы можете клонировать (или посмотреть) созданный мной проект и перейти в папку `ultrasonic`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/ultrasonic
``` 
