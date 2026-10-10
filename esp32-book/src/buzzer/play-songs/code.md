{{#title Проигрыватель песен на пищалке ESP32 с ШИМ на embedded Rust}}

# Код

### Создание проекта с помощью esp-generate

Этот шаг вы уже выполнили в разделе «Быстрый старт».

Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 buzzer-song
```

Откроется экран с вопросом о выборе опций. В последних версиях esp-hal для ledc нужно явно включить нестабильные возможности.

- Поэтому выберите опцию «Enable unstable HAL features» («Включить нестабильные возможности HAL»).

Затем сохраните настройки, нажав на клавиатуре «s».

### Вывод пищалки

Мы зададим GPIO 33 как выходной вывод. К нему подключён положительный контакт пищалки.

```rust
let mut buzzer = peripherals.GPIO33;
let ledc = Ledc::new(peripherals.LEDC);
```

### Экземпляр Song
Создаём экземпляр структуры `Song` с темпом песни, которую будем проигрывать.
```rust
let song = Song::new(pink_panther::TEMPO);
```

### Воспроизведение нот с помощью ШИМ

Мы будем перебирать ноты песни, используя частоту и длительность каждой ноты. Также к длительности каждой ноты добавляем паузу в 10%. Константы частот определены как `f64`, мы преобразуем их в `u64` и используем с Hz. Настройки таймера и канала ШИМ стандартные, с одним отличием: в таймере мы задаём частоту, соответствующую текущей ноте. Коэффициент заполнения задаём 50%.
 
```rust
 for (note, duration_type) in pink_panther::MELODY {
    // получаем длительность ноты
    let note_duration = song.calc_note_duration(duration_type) as u64;
    let pause_duration = note_duration / 10; // 10% от длительности ноты
    if note == music::REST {
        blocking_delay(Duration::from_millis(note_duration));
        continue;
    }

    // Инициализируем LEDC
    let freq = Rate::from_hz(note as u32);
    let mut hstimer0 = ledc.timer::<HighSpeed>(timer::Number::Timer0);
    hstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty10Bit,
            clock_source: timer::HSClockSource::APBClk,
            frequency: freq,
        })
        .unwrap();

    // Инициализируем канал LEDC с коэффициентом заполнения 50%
    let mut channel0 = ledc.channel(channel::Number::Channel0, buzzer.reborrow());
    channel0
        .configure(channel::config::Config {
            timer: &hstimer0,
            duty_pct: 50,
            drive_mode: DriveMode::PushPull,
        })
        .unwrap();

    blocking_delay(Duration::from_millis(note_duration - pause_duration)); // играем 90%
    channel0.set_duty(0).unwrap();
    blocking_delay(Duration::from_millis(pause_duration)); // пауза 10%
}
```


## Клонирование существующего проекта
Вы можете клонировать (или посмотреть) созданный мной проект и перейти в папку `buzzer-song`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/buzzer-song
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
use esp_hal::main;
use esp_hal::time::{Duration, Instant};

use buzzer_song::{
    music::{self, Song},
    pink_panther,
};

// LEDC
use esp_hal::gpio::DriveMode;
use esp_hal::ledc::HighSpeed;
use esp_hal::ledc::timer;
use esp_hal::{
    ledc::{
        Ledc,
        channel::{self, ChannelIFace},
        timer::TimerIFace,
    },
    time::Rate,
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

    let mut buzzer = peripherals.GPIO33;

    let ledc = Ledc::new(peripherals.LEDC);

    let song = Song::new(pink_panther::TEMPO);

    for (note, duration_type) in pink_panther::MELODY {
        let note_duration = song.calc_note_duration(duration_type) as u64;
        let pause_duration = note_duration / 10; // 10% of note_duration
        if note == music::REST {
            blocking_delay(Duration::from_millis(note_duration));
            continue;
        }
        let freq = Rate::from_hz(note as u32);

        let mut hstimer0 = ledc.timer::<HighSpeed>(timer::Number::Timer0);
        hstimer0
            .configure(timer::config::Config {
                duty: timer::config::Duty::Duty10Bit,
                clock_source: timer::HSClockSource::APBClk,
                frequency: freq,
            })
            .unwrap();

        let mut channel0 = ledc.channel(channel::Number::Channel0, buzzer.reborrow());
        channel0
            .configure(channel::config::Config {
                timer: &hstimer0,
                duty_pct: 50,
                drive_mode: DriveMode::PushPull,
            })
            .unwrap();

        blocking_delay(Duration::from_millis(note_duration - pause_duration)); // play 90%

        channel0.set_duty(0).unwrap();
        blocking_delay(Duration::from_millis(pause_duration)); // Pause for 10%
    }

    loop {
        blocking_delay(Duration::from_millis(5));
    }
}

fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}
```
