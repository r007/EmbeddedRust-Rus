# Управление сервоприводом на Raspberry Pi Pico с помощью rp-hal

В этом упражнении мы повторяем тот же пример управления сервоприводом, но на этот раз с rp-hal вместо Embassy. Общая идея остаётся точно такой же. Основное отличие — мы будем использовать дробный делитель вместо делителя-целого.

Для этого мы будем опираться на [форму-калькулятор](../core-concepts/pwm/frequency-to-pwm-top-divider.md), которая сгенерирует значение TOP и целую и дробную части делителя.

```sh
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

Когда появится запрос, задайте проекту имя, например «servo-motor», и выберите «rp-hal» в качестве HAL. Включите логирование `defmt`, если у вас есть отладочный зонд, чтобы также видеть логи.

## Дополнительные импорты

Помимо обычного шаблонного кода rp-hal, нужно подключить трейт, который позволяет обновлять коэффициент заполнения ШИМ.

```rust
// Для ШИМ
use embedded_hal::pwm::SetDutyCycle;
```

## Инициализация слайса ШИМ

Далее инициализируем периферию ШИМ и выбираем слайс, который будем использовать. В нашем случае это слайс ШИМ 7 (поскольку мы используем GPIO 15).

```rust
let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);
let pwm = &mut pwm_slices.pwm7;
```

## Настройка делителя и TOP

Теперь применяем значение TOP и делитель, которые мы получили с помощью формы-калькулятора. На этот раз мы явно задаём и целую, и дробную части делителя.

```rust
pwm.set_div_int(45);
pwm.set_div_frac(13);
pwm.set_top(65483);
pwm.enable();
```

## Подключение канала ШИМ к GPIO

```rust
let servo = &mut pwm.channel_b;
servo.output_to(pins.gpio15);
```

## Главный цикл

Наконец, в главном цикле мы меняем коэффициент заполнения, чтобы перемещать сервопривод между разными положениями. Как и в примере на Embassy, используем `set_duty_cycle_fraction`.

```rust
loop {
    // Переместить сервопривод в положение 0° (коэффициент заполнения 2,5 % = 25/1000)
    servo
        .set_duty_cycle_fraction(25, 1000)
        .expect("invalid min duty cycle");
    timer.delay_ms(1000);

    // Положение 90° (коэффициент заполнения 7,5 %)
    servo
        .set_duty_cycle_fraction(75, 1000)
        .expect("invalid half duty cycle");
    timer.delay_ms(1000);

    // Положение 180° (коэффициент заполнения 12 %)
    servo
        .set_duty_cycle_fraction(120, 1000)
        .expect("invalid max duty cycle");
    timer.delay_ms(1000);
}
```

## Клонируйте существующий проект

Можно клонировать проект, который я создал, и перейти в папку `servo-motor`.

```sh
git clone https://github.com/ImplFerris/pico2-rp-projects
cd pico2-rp-projects/servo-motor
```

## Полный код

```rust
#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use hal::block::ImageDef;
use rp235x_hal as hal;

use embedded_hal::pwm::SetDutyCycle;

//Обработчик паники
use panic_probe as _;
// Логирование через defmt
use defmt_rtt as _;

/// Сообщаем Boot ROM о нашем приложении
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();
/// Внешний высокоскоростной кварц на плате Raspberry Pi Pico 2 работает на 12 МГц.
/// Измените значение, если на вашей плате другая частота
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[hal::entry]
fn main() -> ! {
    // Получаем наши синглтон-объекты
    let mut pac = hal::pac::Peripherals::take().unwrap();

    // Настраиваем драйвер сторожевого таймера — он нужен для настройки тактирования
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Настраиваем тактирование
    //
    // По умолчанию генерируется системная частота 125 МГц
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Блок SIO с однотактным доступом управляет нашими выводами GPIO
    let sio = hal::Sio::new(pac.SIO);

    // Настраиваем выводы в соответствии с их функциями на этой плате
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);
    let pwm = &mut pwm_slices.pwm7;

    pwm.set_div_int(45);
    pwm.set_div_frac(13);
    pwm.set_top(65483);
    pwm.enable();

    let servo = &mut pwm.channel_b;
    servo.output_to(pins.gpio15);

    loop {
        // Переместить сервопривод в положение 0° (коэффициент заполнения 2,5% = 25/1000)
        servo
            .set_duty_cycle_fraction(25, 1000)
            .expect("invalid min duty cycle");
        timer.delay_ms(1000);

        // Положение 90° (коэффициент заполнения 7,5%)
        servo
            .set_duty_cycle_fraction(75, 1000)
            .expect("invalid half duty cycle");
        timer.delay_ms(1000);

        // Положение 180° (коэффициент заполнения 12%)
        servo
            .set_duty_cycle_fraction(120, 1000)
            .expect("invalid max duty cycle");
        timer.delay_ms(1000);
    }
}

// Метаданные программы для `picotool info`.
// Это не обязательно, но рекомендуется добавить эти минимальные записи.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 5] = [
    hal::binary_info::rp_cargo_bin_name!(),
    hal::binary_info::rp_cargo_version!(),
    hal::binary_info::rp_program_description!(c"your program description"),
    hal::binary_info::rp_cargo_homepage_url!(),
    hal::binary_info::rp_program_build_attribute!(),
];

// Конец файла
```
