{{#title Управление сервоприводом через MCPWM ESP32 на embedded Rust}}

# Управление сервоприводом с помощью периферии MCPWM ESP32

В предыдущем упражнении мы использовали периферию LEDC ESP32 для управления сервоприводом. В этом упражнении сделаем то же самое с помощью MCPWM. Введение в MCPWM, принцип его работы и соответствующие функции esp-hal описаны [здесь](../core-concepts/pwm/mcpwm.md). Прочитайте эту главу, прежде чем двигаться дальше.


### Создание проекта с помощью esp-generate

Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 servo-mcpwm
```

Откроется экран с вопросом о выборе опций. В последних версиях esp-hal для mcpwm нужно явно включить нестабильные возможности.

- Поэтому выберите опцию «Enable unstable HAL features» («Включить нестабильные возможности HAL»).

Затем сохраните настройки, нажав на клавиатуре «s».

### Настройка тактирования
Создадим экземпляр тактирования периферии. Настроим тактирование на 32 МГц. Функция сама вычислит предделитель и поделит входную частоту, которая составляет 160 МГц.

```rust
let clock_cfg = PeripheralClockConfig::with_frequency(Rate::from_mhz(32)).unwrap();
```

Для сервопривода нам нужна итоговая частота ШИМ 50 Гц, поэтому базовую частоту тактирования желательно сделать как можно меньше. Можно опуститься до 625 кГц при максимальном значении предделителя 255, но мы оставим 1 МГц, чтобы расчёты были проще.

### Настройка MCPWM и вывода

Мы будем использовать периферию MCPWM0, выберем timer0 и operator0. Затем настроим её на GPIO33 и зададим, чтобы ШИМ-сигнал оставался в высоком состоянии до значения временной метки (timestamp), которое мы укажем в цикле ШИМ.

```rust
let mut mcpwm = McPwm::new(peripherals.MCPWM0, clock_cfg);
// подключаем operator0 к timer0
mcpwm.operator0.set_timer(&mcpwm.timer0);
// подключаем operator0 к выводу
let mut pwm_pin = mcpwm
    .operator0
    .with_pin_a(peripherals.GPIO33, PwmPinConfig::UP_ACTIVE_HIGH);
```

### Настройка таймера

Чтобы получить ШИМ-сигнал 50 Гц для сервопривода при тактовой частоте 1 МГц, таймер должен отсчитать в сумме 20 000 тактов. Поскольку таймер считает от 0 до 19 999, период задаём равным 19_999, что даёт в итоге 20 000 тактов.

```rust
let timer_clock_cfg = clock_cfg
    .timer_clock_with_frequency(19_999, PwmWorkingMode::Increase, Rate::from_hz(50))
    .unwrap();
mcpwm.timer0.start(timer_clock_cfg);
```

### Поворот качалки сервопривода

Чтобы повернуть качалку сервопривода, мы меняем временную метку (timestamp) ШИМ-сигнала. Значения временной метки соответствуют нужным углам:

- Для 0 градусов задаём временную метку 500 (2,5% от 20 000).
- Для 90 градусов — 1500 (7,5% от 20 000).
- Для 180 градусов — 2500 (12,5% от 20 000).

После каждой установки делаем достаточную задержку, чтобы сервопривод успел дойти до указанного положения.

```rust
loop {
    // 0 градусов (2,5% от 20_000 => 500)
    pwm_pin.set_timestamp(500);
    delay.delay_millis(1500);

    // 90 градусов (7,5% от 20_000 => 1500)
    pwm_pin.set_timestamp(1500);
    delay.delay_millis(1500);

    // 180 градусов (12,5% от 20_000 => 2500)
    pwm_pin.set_timestamp(2500);
    delay.delay_millis(1500);
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

// MCPWM
use esp_hal::mcpwm::operator::PwmPinConfig;
use esp_hal::mcpwm::timer::PwmWorkingMode;
use esp_hal::mcpwm::{McPwm, PeripheralClockConfig};
use esp_hal::time::Rate;

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

    let delay = Delay::new();

    let clock_cfg = PeripheralClockConfig::with_frequency(Rate::from_mhz(32)).unwrap();
    let mut mcpwm = McPwm::new(peripherals.MCPWM0, clock_cfg);

    // подключаем operator0 к timer0
    mcpwm.operator0.set_timer(&mcpwm.timer0);
    // подключаем operator0 к выводу
    let mut pwm_pin = mcpwm
        .operator0
        .with_pin_a(peripherals.GPIO33, PwmPinConfig::UP_ACTIVE_HIGH);

    // запускаем таймер со значениями временной метки в диапазоне 0..=19999 и частотой
    // 50 Гц
    let timer_clock_cfg = clock_cfg
        .timer_clock_with_frequency(19_999, PwmWorkingMode::Increase, Rate::from_hz(50))
        .unwrap();
    mcpwm.timer0.start(timer_clock_cfg);

    loop {
        // 0 градусов (2,5% от 20_000 => 500)
        pwm_pin.set_timestamp(500);
        delay.delay_millis(1500);

        // 90 градусов (7,5% от 20_000 => 1500)
        pwm_pin.set_timestamp(1500);
        delay.delay_millis(1500);

        // 180 градусов (12,5% от 20_000 => 2500)
        pwm_pin.set_timestamp(2500);
        delay.delay_millis(1500);
    }
}
```

## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `servo-mcpwm`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/servo-mcpwm
```
