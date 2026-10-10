{{#title Пишем код на Rust для эффекта затухания светодиода на ESP32}}

# Пишем код на Rust для эффекта затухания светодиода на ESP32

Теперь начинается самое интересное: давайте займёмся программированием!

### Создание проекта с помощью esp-generate

Этот шаг вы уже выполнили в разделе «Быстрый старт».

Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 led-fader
```

Откроется экран с вопросом о выборе опций. В последних версиях esp-hal для ledc нужно явно включить нестабильные возможности.

- Поэтому выберите опцию «Enable unstable HAL features» («Включить нестабильные возможности HAL»).

Затем сохраните настройки, нажав на клавиатуре «s».

> [!Tip]
> В объяснении я не буду приводить необходимые импорты: они не особенно интересны и мало что объясняют. Всегда можно обратиться к разделу «Полный код» ниже или склонировать проект для сверки.


## Автоматически сгенерированный код

Когда вы создаёте проект с помощью esp-generate, он настраивает базовую структуру, конфигурирует тактовую частоту ЦП и включает стандартный код для периферии, так что вам не нужно набирать его каждый раз. В этой книге мы всегда будем создавать проекты с помощью esp-generate и строить поверх них.

```rust
let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
let peripherals = esp_hal::init(config);
```

## Вывод светодиода

Далее берём нужный GPIO из экземпляра `peripherals`. В нашем случае мы управляем встроенным светодиодом DevKit, подключённым к GPIO 2.

```rust
let led = peripherals.GPIO2;
```

### Настройка ШИМ

В этом упражнении мы будем использовать низкоскоростной канал ШИМ. Сначала нужно задать источник тактирования. Библиотека esp-hal определяет перечисление `LSGlobalClkSource` для низкоскоростного источника тактирования, в котором сейчас есть только одно значение: `APBClk`.

```rust
ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
```

Далее настраиваем таймер. Так как мы используем низкоскоростной канал ШИМ, нам, очевидно, нужен низкоскоростной таймер. Также нужно указать, какой низкоскоростной таймер использовать (от 0 до 3).

```rust
let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
```

Перед использованием таймера нужно выполнить ещё несколько настроек. Мы зададим частоту 24 кГц. Для этой частоты при тактировании от APB формула даёт максимальное разрешение 12 бит и минимальное 2 бита. В esp-hal для этой частоты используется 5-битное разрешение ШИМ, и мы будем использовать то же.

```rust
lstimer0
    .configure(timer::config::Config {
        duty: timer::config::Duty::Duty5Bit,
        clock_source: timer::LSClockSource::APBClk,
        frequency: Rate::from_khz(24),
    })
    .unwrap();
```
 
### Каналы ШИМ

Далее настраиваем канал ШИМ. Мы будем использовать channel0 и настроим его с выбранным таймером и начальным коэффициентом заполнения «10%». Кроме того, зададим конфигурацию вывода как PushPull.

```rust
let mut channel0 = ledc.channel(channel::Number::Channel0, led);
channel0
    .configure(channel::config::Config {
        timer: &lstimer0,
        duty_pct: 10,
        drive_mode: DriveMode::PushPull,
    })
    .unwrap();
```

### Затухание 

В esp-hal есть функция `start_duty_fade`, которая упрощает нашу работу. В противном случае нам пришлось бы вручную увеличивать и уменьшать коэффициент заполнения в цикле через равные промежутки времени. Эта функция плавно меняет коэффициент заполнения от одного процентного значения к другому. Она принимает и третий параметр, который задаёт, сколько времени должен занять переход от одного коэффициента заполнения к другому.

```rust
channel0.start_duty_fade(0, 100, 1000).unwrap();
```

Мы будем запускать это в цикле и использовать ещё одну функцию HAL, `is_duty_fade_running`: она возвращает логическое значение, показывая, завершено ли затухание.

```rust
while channel0.is_duty_fade_running() {}
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
use esp_hal::gpio::DriveMode;
use esp_hal::main;
use esp_hal::time::Rate;

// Для LEDC
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::ledc::timer::TimerIFace;
use esp_hal::ledc::{LSGlobalClkSource, Ledc, LowSpeed, channel, timer};

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

    let led = peripherals.GPIO2;
    // let led = peripherals.GPIO5;

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

    loop {
        channel0.start_duty_fade(0, 100, 1000).unwrap();
        while channel0.is_duty_fade_running() {}
        channel0.start_duty_fade(100, 0, 1000).unwrap();
        while channel0.is_duty_fade_running() {}
    }
}
```


## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `led-fader`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/led-fader
```

## Прошивка
Как только вы прошьёте код в ESP32, вы должны увидеть эффект затухания на встроенном светодиоде.

```rust
cargo run --release
```
