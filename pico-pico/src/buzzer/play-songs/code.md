## Воспроизведение мелодии Game of Thrones

В этом разделе мы собираем всё вместе в файле `main.rs`.

К этому моменту у нас уже есть частоты нот, логика тайминга песни и данные мелодии. Здесь мы просто связываем их между собой с помощью ШИМ и таймеров.

## Импорты

Добавьте необходимые импорты для ШИМ, таймеров и работы с песнями.

```rust
// Для ШИМ
use embassy_rp::pwm::{Config as PwmConfig, Pwm, SetDutyCycle};

use crate::music::Song;
```

## ШИМ и пищалка

В отличие от предыдущего примера, мы клонируем `PwmConfig`. Конструктор `Pwm` забирает объект конфигурации, а внутри цикла нам нужно менять `top`, чтобы задавать частоту ШИМ для каждой музыкальной ноты.

```rust
let mut pwm_config = PwmConfig::default();
pwm_config.top = PWM_TOP;
pwm_config.divider = PWM_DIV_INT.into();

let mut buzzer = Pwm::new_output_b(p.PWM_SLICE7, p.PIN_15, pwm_config.clone());
```

## Создание объекта Song

Создайте `Song` с темпом, заданным для темы Game of Thrones.

```rust
let song = Song::new(got::TEMPO);
```

## Воспроизведение нот

Мелодия проигрывается циклом по массиву `MELODY`. Каждый элемент содержит частоту ноты и значение длительности.

```rust
// Один раз проигрываем мелодию
for (note, duration_type) in got::MELODY {
    let top = get_top(note, PWM_DIV_INT);
    pwm_config.top = top;
    buzzer.set_config(&pwm_config);

    let note_duration = song.calc_note_duration(duration_type);
    let pause_duration = note_duration / 10; // 10% от note_duration

    buzzer
        .set_duty_cycle_percent(50)
        .expect("50 is valid duty percentage"); // Ставим коэффициент заполнения 50%, чтобы сыграть ноту

    Timer::after_millis(note_duration - pause_duration).await; // Играем 90%

    buzzer
        .set_duty_cycle_percent(0)
        .expect("50 is valid duty percentage"); // Останавливаем тон
    Timer::after_millis(pause_duration).await; // Пауза 10%
}
```

Для каждой ноты частота ШИМ обновляется установкой нового значения `top`. Это заставляет пищалку издавать нужную высоту тона.

Длительность ноты рассчитывается по темпу песни. Большую часть времени нота звучит, а небольшая часть остаётся тихой. Эта короткая тишина помогает отделить ноты друг от друга, благодаря чему мелодия звучит чище.

Пищалка включается установкой коэффициента заполнения 50 процентов и выключается установкой нуля.

### Поддержание работы программы

После окончания мелодии этот цикл просто удерживает программу активной.

```rust
loop {
    Timer::after_millis(100).await;
}
```

## Клонируйте существующий проект

Можно клонировать (или использовать как образец) проект, который я создал, и перейти в папку `buzzer-song`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/buzzer-song
```

## Версия на rp-hal

Если хотите увидеть тот же пример, реализованный с помощью rp-hal, его можно найти здесь.

```sh
git clone https://github.com/ImplFerris/pico2-rp-projects
cd pico2-rp-projects/got-buzzer
```
