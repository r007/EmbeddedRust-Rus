# Код на embedded Rust для управления светодиодом по уровню освещённости

Когда схема собрана на макетной плате, напишем код.

## Проект из шаблона

Как обычно, начнём с создания нового проекта из шаблона.

```sh
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

Когда появится запрос, задайте проекту имя, например «ldr-dracula», и выберите `embassy` в качестве HAL.

## Дополнительные импорты

```rust
// Для привязки прерываний
use embassy_rp::adc::InterruptHandler;
use embassy_rp::bind_interrupts;

// Для АЦП
use embassy_rp::adc::{Adc, Channel, Config as AdcConfig};

// Для светодиода
use embassy_rp::gpio::{Level, Output, Pull};
```

## Обработчик прерывания для АЦП

В АЦП RP2350 есть буфер FIFO (First-In-First-Out), который хранит результаты преобразований (то есть измеренное напряжение, преобразованное в дискретное значение АЦП). Когда в этот FIFO поступают данные, аппаратура генерирует сигнал прерывания `ADC_IRQ_FIFO`.

В библиотеке Embassy логика обработки прерываний АЦП уже реализована. Нам нужно лишь привязать это аппаратное прерывание, чтобы драйвер АЦП мог его использовать.

Перед тем как начать читать значения АЦП, нужно настроить эту привязку прерывания.

```rust
bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});
```

## Порог АЦП

Прежде чем писать основную логику, нужно решить, что считать слабым освещением. Для этого зададим пороговое значение показаний АЦП.

```rust
const LDR_THRESHOLD: u16 = 200;
```

Этот порог задаёт уровень освещённости, при котором должен включаться светодиод. Когда показание АЦП опускается ниже этого значения, мы считаем, что в помещении темно. Когда показание выше — светло.

Возможно, порог придётся подстроить в зависимости от освещения в комнате, используемого LDR и номиналов резисторов в делителе напряжения.

## Создание экземпляра АЦП

Создадим экземпляр АЦП.

```rust
let mut adc = Adc::new(p.ADC, Irqs, AdcConfig::default());
```

Здесь мы передаём конструктору АЦП три вещи: саму периферию АЦП, привязку прерываний, которую мы определили ранее, и конфигурацию по умолчанию.

> [!Note]
> Интересный факт: HAL на самом деле не использует `Irqs` во время выполнения, когда вы передаёте его в конструктор АЦП. Он нужен только на этапе компиляции, чтобы убедиться, что вы объявили привязку прерывания АЦП. Если вы посмотрите на новый метод, то увидите, что параметр называется `_irq`, — это явно показывает, что он не используется.

## Настройка вывода АЦП

Далее выбираем, какой вывод GPIO будет использоваться как вход АЦП.

```rust
let mut adc_pin = Channel::new_pin(p.PIN_28, Pull::None);
```

## Настройка выхода для светодиода

Теперь настраиваем вывод GPIO, который управляет светодиодом.

```rust
let mut led = Output::new(p.PIN_15, Level::Low);
```

## Главный цикл

Логика простая: читаем значение АЦП, и если оно меньше заданного порога, включаем светодиод, иначе выключаем.

```rust
loop {
    let adc_reading = adc
        .read(&mut adc_pin)
        .await
        .expect("Unable to read the adc value");
    defmt::info!("ADC value: {}", adc_reading);

    if adc_reading < LDR_THRESHOLD {
        led.set_high();
    } else {
        led.set_low();
    }

    Timer::after_secs(1).await;
}
```

## Полный код

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_time::Timer;

// Привязка прерываний
use embassy_rp::adc::InterruptHandler;
use embassy_rp::bind_interrupts;

// АЦП
use embassy_rp::adc::{Adc, Channel, Config as AdcConfig};

// Для светодиода
use embassy_rp::gpio::{Level, Output, Pull};

//Обработчик паники
use panic_probe as _;
// Логирование через defmt
use defmt_rtt as _;

/// Сообщаем Boot ROM о нашем приложении
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

const LDR_THRESHOLD: u16 = 200;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut adc = Adc::new(p.ADC, Irqs, AdcConfig::default());

    let mut adc_pin = Channel::new_pin(p.PIN_28, Pull::None);
    let mut led = Output::new(p.PIN_15, Level::Low);

    loop {
        let adc_reading = adc
            .read(&mut adc_pin)
            .await
            .expect("Unable to read the adc value");
        defmt::info!("ADC value: {}", adc_reading);
        if adc_reading < LDR_THRESHOLD {
            led.set_high();
        } else {
            led.set_low();
        }
        Timer::after_secs(1).await;
    }
}

// Метаданные программы для `picotool info`.
// Это не обязательно, но рекомендуется добавить эти минимальные записи.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"ldr-dracula"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// Конец файла
```

## Клонируйте существующий проект

Можно клонировать проект, который я создал, и перейти в папку `ldr-dracula`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/ldr-dracula/
```
