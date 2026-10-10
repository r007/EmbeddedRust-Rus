{{#title Пишем код на Rust: светодиод включается в темноте с фоторезистором и ESP32}}

# Включаем светодиод в темноте с фоторезистором и ESP32 на Rust

## Создание проекта с помощью esp-generate

Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 ldr-dracula
```

Откроется экран с вопросом о выборе опций. В последних версиях esp-hal для ADC нужно явно включить нестабильные возможности.

- Поэтому выберите опцию «Enable unstable HAL features» («Включить нестабильные возможности HAL»).

В этом упражнении мы будем выводить сообщения в системную консоль. Для этого нужно включить функцию логирования.

- Прокрутите список до пункта «Flashing, logging and debugging (espflash)» и нажмите Enter.

- Затем выберите «Use defmt to print messages» («Использовать defmt для вывода сообщений»).


Сохраните настройки, нажав на клавиатуре «s».

## Обновляем Cargo.toml

Крейт nb упрощает неблокирующий ввод-вывод (например, чтение датчиков или данных UART): он возвращает `nb::Result` с ошибкой `WouldBlock`, когда операция ещё не готова, что позволяет повторить её позже без блокировки.

Если интересно, зачем он нужен: функция `read_oneshot` у АЦП возвращает `nb::Result` и может вернуть `nb::Error::WouldBlock`, если результат ещё не готов. Обернув её в `nb::block`, мы заставляем код повторять попытки, пока АЦП не закончит работу и не вернёт нормальный результат.

```toml
nb = "1.1.0"
```

### Настройка светодиода

Мы уже делали это раньше: настроим GPIO 33, к которому подключён светодиод, как выходной вывод и инициализируем его в состоянии Low.  

```rust
let mut led = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());
```

### Настройка АЦП 

Настроим GPIO 4 как вход АЦП — это один из каналов ADC2. Применим ослабление 11 дБ, которое позволяет АЦП измерять входное напряжение от 150 мВ до ~2450 мВ.

```rust
let adc_pin = peripherals.GPIO4;
let mut adc2_config = AdcConfig::new();
let mut pin = adc2_config.enable_pin(adc_pin, Attenuation::_11dB);
let mut adc2 = Adc::new(peripherals.ADC2, adc2_config);
```

### Одиночное чтение (oneshot)

Функция `read_oneshot` запускает одно преобразование АЦП на заданном выводе. Она неблокирующая и возвращает 16-битное значение, обёрнутое в `Result`. Однако мы будем использовать `nb::block`, чтобы дождаться завершения преобразования, и только потом продолжить.
 
```rust
let pin_value: u16 = nb::block!(adc2.read_oneshot(&mut pin)).unwrap();
```

### Переключение светодиода
Получив цифровое значение, включить или выключить светодиод очень просто. Мы включим светодиод, переведя его в High, если значение вывода больше 3500 (порог можно менять по необходимости). В противном случае выключим его. Порог 3500 обычно достигается, когда в комнате темно и сопротивление LDR (R2) высокое, из-за чего напряжение выше.

```rust
if pin_value > 3500 {
    led.set_high();
} else {
    led.set_low();
}
```
Если непонятно, почему высокое сопротивление LDR приводит к более высокому напряжению, перечитайте раздел [«Как это работает»](../how-it-works.md) и поэкспериментируйте с симулятором.

### Итоговый код

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
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_println as _;

// ADC
use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};

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

    let mut led = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());

    let adc_pin = peripherals.GPIO4;
    let mut adc2_config = AdcConfig::new();
    let mut pin = adc2_config.enable_pin(adc_pin, Attenuation::_11dB);
    let mut adc2 = Adc::new(peripherals.ADC2, adc2_config);
    let delay = Delay::new();

    loop {
        let pin_value: u16 = nb::block!(adc2.read_oneshot(&mut pin)).unwrap();
        esp_println::println!("{}", pin_value);

        if pin_value > 3500 {
            led.set_high();
        } else {
            led.set_low();
        }

        delay.delay_millis(500);
    }
}
```

## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `ldr-dracula`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/ldr-dracula
```
