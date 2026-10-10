{{#title Чтение значений АЦП джойстика на ESP32 на embedded Rust}}

# Вывод значений АЦП при движении джойстика

В этой программе мы будем наблюдать в реальном времени, как движение джойстика влияет на значения АЦП. Подключим ESP32 к джойстику.

Когда вы двигаете джойстик, соответствующие значения АЦП выводятся в системную консоль. Их можно сравнить с [предыдущей диаграммой «Движение и значения АЦП»](./movement-and-12-bit-adc-value.md): они должны примерно совпадать. Нажатие на ручку джойстика выведет **«Button Pressed»** вместе с текущими координатами.


## Создание проекта с помощью esp-generate
Мы включим поддержку async (Embassy) для этого проекта. Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 joystick-movement
```

Откроется экран с вопросом о выборе опций.

- Выберите опцию «Enable unstable HAL features».
- Затем выберите опцию «Adds embassy framework support».

Сохраните настройки, нажав на клавиатуре «s».


## Обновляем Cargo.toml

Крейт nb упрощает неблокирующий ввод-вывод (например, чтение датчиков или данных UART): он возвращает `nb::Result` с ошибкой `WouldBlock`, когда операция ещё не готова, что позволяет повторить её позже без блокировки.

Если интересно, зачем он нужен: функция `read_oneshot` у АЦП возвращает `nb::Result` и может вернуть `nb::Error::WouldBlock`, если результат ещё не готов. Обернув её в `nb::block`, мы заставляем код повторять попытки, пока АЦП не закончит работу и не вернёт нормальный результат.

```toml
nb = "1.1.0"
```

### Настройка АЦП
Настроим АЦП: GPIO 13 и GPIO 14 соответствуют выводам VRX и VRY джойстика.

```rust
let mut adc2_config = AdcConfig::new();
let mut vrx_pin = adc2_config.enable_pin(peripherals.GPIO13, Attenuation::_11dB);
let mut vry_pin = adc2_config.enable_pin(peripherals.GPIO14, Attenuation::_11dB);

let mut adc2 = Adc::new(peripherals.ADC2, adc2_config);

```

Также настроим GPIO 32 как вход с подтягивающим резистором (pull-up) для кнопки:

```rust
let btn = Input::new(
    peripherals.GPIO32,
    InputConfig::default().with_pull(Pull::Up),
);
```

### Вывод координат

Мы хотим выводить координаты, только когда значения vrx или vry меняются больше определённого порога. Так мы избегаем постоянного вывода ненужных значений.

Для этого инициализируем переменные, которые хранят предыдущие значения, и флаг, определяющий, когда выводить:

```rust
let mut prev_vrx: u16 = 0;
let mut prev_vry: u16 = 0;
let mut prev_btn_state = false;
let mut print_vals = true;
```

**Чтение значений АЦП:**

Сначала считываем значения АЦП для vrx и vry. Если при чтении возникает ошибка, мы игнорируем её и продолжаем цикл:

```rust
let Ok(vry): Result<u16, _> = nb::block!(adc2.read_oneshot(&mut vry_pin)) else {
    continue;
};
let Ok(vrx): Result<u16, _> = nb::block!(adc2.read_oneshot(&mut vrx_pin)) else {
    continue;
};
```

**Проверка изменения по порогу:**

Далее проверяем, превышает ли абсолютная разность между текущим и предыдущим значением vrx или vry порог (например, 100). Если да, обновляем предыдущие значения и устанавливаем флаг `print_vals` в `true`:

```rust
if vrx.abs_diff(prev_vrx) > 100 {
    prev_vrx = vrx;
    print_vals = true;
}

if vry.abs_diff(prev_vry) > 100 {
    prev_vry = vry;
    print_vals = true;
}
```
Порог отсекает мелкие колебания АЦП, исключает ненужный вывод и обеспечивает обновления только при значимых изменениях.

**Вывод координат**

Если `print_vals` равен `true`, сбрасываем его в `false` и выводим координаты X и Y через USB-serial:

```rust
if print_vals {
    print_vals = false;
    println!("X: {} Y: {}\r\n", vrx, vry);
}
```

### Определение нажатия кнопки по переходу состояния
Кнопка обычно находится в высоком состоянии. Когда вы нажимаете на кнопку ручки, она переключается из высокого состояния в низкое. Однако поскольку программа работает в цикле, простая проверка на низкий уровень может привести к многократному срабатыванию на одно нажатие. Чтобы этого избежать, мы регистрируем нажатие только один раз, фиксируя переход из высокого состояния в низкое, который означает, что кнопка нажата.

Для этого мы запоминаем предыдущее состояние кнопки и сравниваем его с текущим перед выводом сообщения «Button Pressed». Если кнопка сейчас в низком состоянии (нажата), а раньше была в высоком (не нажата), мы считаем это новым нажатием и выводим сообщение. Затем обновляем предыдущее состояние до текущего, чтобы корректно отслеживать будущие переходы.

```rust
let btn_state = btn.is_low();
if btn_state && !prev_btn_state {
    println!("Button Pressed");
    print_vals = true;
}
prev_btn_state = btn_state;
```



## Клонирование существующего проекта
Вы можете клонировать (или посмотреть) созданный мной проект и перейти в папку `joystick-movement`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/joystick-movement/
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

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_println::{self as _, println};

use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::gpio::{Input, InputConfig, Pull};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Создаёт дескриптор приложения по умолчанию, который требуется загрузчику esp-idf.
// Подробнее см.: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.0.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    info!("Embassy initialized!");

    let _ = spawner;

    let btn = Input::new(
        peripherals.GPIO32,
        InputConfig::default().with_pull(Pull::Up),
    );

    let mut adc2_config = AdcConfig::new();
    let mut vrx_pin = adc2_config.enable_pin(peripherals.GPIO13, Attenuation::_11dB);
    let mut vry_pin = adc2_config.enable_pin(peripherals.GPIO14, Attenuation::_11dB);

    let mut adc2 = Adc::new(peripherals.ADC2, adc2_config);

    let mut prev_vrx: u16 = 0;
    let mut prev_vry: u16 = 0;
    let mut prev_btn_state = false;
    let mut print_vals = true;

    loop {
        let Ok(vry): Result<u16, _> = nb::block!(adc2.read_oneshot(&mut vry_pin)) else {
            continue;
        };
        let Ok(vrx): Result<u16, _> = nb::block!(adc2.read_oneshot(&mut vrx_pin)) else {
            continue;
        };

        if vrx.abs_diff(prev_vrx) > 100 {
            prev_vrx = vrx;
            print_vals = true;
        }

        if vry.abs_diff(prev_vry) > 100 {
            prev_vry = vry;
            print_vals = true;
        }

        let btn_state = btn.is_low();
        if btn_state && !prev_btn_state {
            println!("Button Pressed");
            print_vals = true;
        }
        prev_btn_state = btn_state;

        if print_vals {
            print_vals = false;

            println!("X: {} Y: {}\r\n", vrx, vry);
        }

        Timer::after(Duration::from_millis(50)).await;
    }
}
```
