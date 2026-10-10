{{#title Чтение значений АЦП джойстика на Raspberry Pi Pico 2 с Embassy}}

# Определяем положение джойстика по значениям АЦП Raspberry Pi Pico

В этой главе мы будем выводить значения АЦП джойстика каждый раз, когда они меняются. Мы считаем два аналоговых сигнала, vrx и vry, которые подключены к осям джойстика, а также выведем значения при нажатии кнопки джойстика.

Цель проста: нужно увидеть, как физическое движение джойстика превращается в числовые значения АЦП. Когда вы поймёте, как значения меняются при движении ручки, эту идею можно использовать в проектах: управление роботами, радиоуправляемыми машинками, дронами, поворотными механизмами камеры и многое другое.

## Проект из шаблона

Начнём с создания нового проекта из шаблона.

```sh
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

Когда появится запрос, задайте проекту имя, например joystick-adc, и выберите embassy в качестве HAL.

Если у вас есть отладочный зонд, можно включить defmt, и логирование будет простым. Если отладочного зонда нет, отключите defmt и используйте вывод через USB Serial, как в предыдущей главе. Здесь эти шаги повторять не будем, чтобы сосредоточиться на настройке джойстика и АЦП.

## Дополнительные импорты

Добавьте следующие импорты в файл main.rs. Они нужны для доступа к АЦП, входу GPIO и обработке прерываний.

```rust
// Для АЦП
use embassy_rp::adc::{Adc, Channel, Config as AdcConfig};
use embassy_rp::gpio::{Input, Pull};

// Привязка прерываний
use embassy_rp::adc;
use embassy_rp::bind_interrupts;
```

## Привязка прерывания АЦП

Далее привязываем прерывание FIFO АЦП.

```rust
bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => adc::InterruptHandler;
});
```

## Настройка АЦП и джойстика

Теперь инициализируем периферию АЦП и настроим выводы джойстика. Сигналы vrx и vry подключены к выводам GPIO, которые поддерживают АЦП. Кнопку джойстика мы настраиваем как обычный вход.

```rust
// Настройка АЦП
let mut adc = Adc::new(p.ADC, Irqs, AdcConfig::default());

let mut vrx_pin = Channel::new_pin(p.PIN_27, Pull::None);
let mut vry_pin = Channel::new_pin(p.PIN_26, Pull::None);
let button = Input::new(p.PIN_15, Pull::Up);
```

## Хранение предыдущих значений

Перед входом в главный цикл сохраняем несколько переменных с предыдущими показаниями. Это позволяет замечать, что значение действительно изменилось, а не выводить значения непрерывно.

```rust
let mut prev_vrx: u16 = 0;
let mut prev_vry: u16 = 0;
let mut print_vals = true;
let mut prev_btn_state = false;
```

Мы храним предыдущие показания по осям X и Y, держим флаг, который управляет тем, когда нужно выводить значения, а также запоминаем предыдущее состояние кнопки, чтобы определить событие нажатия.

## Главный цикл

Внутри главного цикла мы непрерывно считываем значения АЦП с двух выводов джойстика. Если чтение АЦП завершилось ошибкой, мы пропускаем итерацию и пробуем снова.

```rust
loop {
    let Ok(vry) = adc.read(&mut vry_pin).await else {
        continue;
    };
    let Ok(vrx) = adc.read(&mut vrx_pin).await else {
        continue;
    };
```

### Определение значимых изменений

Аналоговые джойстики не бывают идеально стабильными. Даже без прикосновения показания могут немного колебаться. Чтобы вывод оставался читаемым, мы реагируем только тогда, когда изменение достаточно большое.

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

Когда любая из осей меняется больше порога, мы обновляем сохранённое значение и отмечаем, что показания нужно вывести.

### Вывод значений АЦП

Если установлен флаг print_vals, мы выводим текущее положение джойстика и сбрасываем флаг.

```rust
    if print_vals {
        print_vals = false;

        info!("X: {} Y: {}", vrx, vry);
    }
```

### Обработка кнопки

Затем считываем кнопку джойстика. Поскольку она использует подтягивающий резистор, нажатая кнопка читается как низкий уровень.

```rust
    let btn_state = button.is_low();
    if btn_state && !prev_btn_state {
        info!("Button Pressed");

        print_vals = true;
    }
    prev_btn_state = btn_state;
```

Когда кнопка нажата, мы помечаем значения АЦП для вывода. Так можно увидеть точное положение джойстика в момент нажатия.

В конце цикла добавим короткую задержку.

```rust
    Timer::after_millis(100).await;
}
```

## Полный код

```rust
#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_time::Timer;

// Для АЦП
use embassy_rp::adc::{Adc, Channel, Config as AdcConfig};
use embassy_rp::gpio::{Input, Pull};

// Привязка прерываний
use embassy_rp::adc;
use embassy_rp::bind_interrupts;

//Обработчик паники
use panic_probe as _;
// Логирование через defmt
use defmt_rtt as _;

/// Сообщаем Boot ROM о нашем приложении
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => adc::InterruptHandler;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Настройка АЦП
    let mut adc = Adc::new(p.ADC, Irqs, AdcConfig::default());

    let mut vrx_pin = Channel::new_pin(p.PIN_27, Pull::None);
    let mut vry_pin = Channel::new_pin(p.PIN_26, Pull::None);
    let button = Input::new(p.PIN_15, Pull::Up);

    let mut prev_vrx: u16 = 0;
    let mut prev_vry: u16 = 0;
    let mut print_vals = true;
    let mut prev_btn_state = false;

    loop {
        let Ok(vry) = adc.read(&mut vry_pin).await else {
            continue;
        };
        let Ok(vrx) = adc.read(&mut vrx_pin).await else {
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

        if print_vals {
            print_vals = false;

            info!("X: {} Y: {}", vrx, vry);
        }

        let btn_state = button.is_low();
        if btn_state && !prev_btn_state {
            info!("Button Pressed");

            print_vals = true;
        }
        prev_btn_state = btn_state;

        Timer::after_millis(100).await;
    }
}

// Метаданные программы для `picotool info`.
// Это не обязательно, но рекомендуется добавить эти минимальные записи.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"joystick-adc"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// Конец файла
```

## Клонируйте существующий проект

Можно клонировать проект, который я создал, и перейти в папку `joystick-adc`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/joystick/joystick-adc
```
