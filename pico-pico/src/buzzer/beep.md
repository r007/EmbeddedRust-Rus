# Сигнал зуммера с помощью ШИМ на Raspberry Pi Pico на embedded Rust

В этом упражнении мы сгенерируем звуковой сигнал с помощью зуммера. Идея похожа на мигание светодиодом, но вместо того, чтобы переключать вывод GPIO между HIGH и LOW, мы управляем скважностью ШИМ.

Будем многократно переключать скважность ШИМ между 50 процентами и 0 процентами с задержкой между переключениями. Когда скважность составляет 50 процентов, зуммер издаёт звук. Когда 0 процентов — звук прекращается. Повторение этого создаёт отчётливый сигнал.

Можно попробовать и без изменения частоты ШИМ. В этом примере мы задаём частоту ШИМ 440,0 Гц. Это соответствует музыкальной ноте `a4` (ля первой октавы) и тональному сигналу телефонии (call progress tone), который используется в Северной Америке и большей части Европы. Знать что-либо о нотах для этого не нужно. Важно лишь то, что мы генерируем тон фиксированной частоты и включаем или выключаем звук, меняя скважность.

## Создание проекта из шаблона

Начнём с создания нового проекта на основе фреймворка Embassy. Как обычно, создайте проект из шаблона с помощью `cargo-generate`:

```sh
cargo generate --git https://github.com/ImplFerris/pico2-template.git --tag v0.3.2
```

Когда появится запрос, задайте проекту имя, например «buzzer-beep», и выберите «embassy» как HAL. Включите логирование `defmt`, если у вас есть отладочный зонд, чтобы также видеть логи.

## Дополнительные импорты

Как и раньше, импортируем трейт `SetDutyCycle`, а также типы `Pwm` и `Config` для настройки ШИМ.

```rust
use embassy_rp::pwm::{Config as PwmConfig, Pwm, SetDutyCycle};
```

## Вычисление TOP

Значение TOP можно вычислить вручную или воспользоваться [калькулятором](../core-concepts/pwm/frequency-to-pwm-top-divider.md) из предыдущей главы. На этот раз пойдём другим путём.

Зафиксируем делитель на 64 и используем `const fn`, чтобы вычислить значение TOP. Поскольку мы не используем режим phase-correct и дробный делитель, расчёт простой.

```rust
const fn get_top(freq: f64, div_int: u8) -> u16 {
    assert!(div_int != 0, "Divider must not be 0");

    let result = 150_000_000. / (freq * div_int as f64);

    assert!(result >= 1.0, "Frequency too high");
    assert!(
        result <= 65535.0,
        "Frequency too low: TOP exceeds 65534 max"
    );

    result as u16 - 1
}

const PWM_DIV_INT: u8 = 64;
const PWM_TOP: u16 = get_top(440., PWM_DIV_INT);
```

## Основная логика

Сначала настраиваем ШИМ с вычисленным значением TOP и фиксированным делителем. Затем создаём выход ШИМ для зуммера. Внутри цикла переключаем скважность между 50 процентами и 0 процентами с задержкой между переключениями, что даёт повторяющийся звуковой сигнал.

```rust
let mut pwm_config = PwmConfig::default();
pwm_config.top = PWM_TOP;
pwm_config.divider = PWM_DIV_INT.into();

let mut buzzer = Pwm::new_output_b(p.PWM_SLICE7, p.PIN_15, pwm_config);

loop {
    buzzer
        .set_duty_cycle_percent(50)
        .expect("50 is valid duty percentage");
    Timer::after_millis(1000).await;

    buzzer
        .set_duty_cycle_percent(0)
        .expect("0 is valid duty percentage");
    Timer::after_millis(1000).await;
}
```

Если хотите, чтобы сигнал был короче или быстрее, измените значения задержек.

## Клонируйте существующий проект

Можно клонировать (или посмотреть) проект, который я создал, и перейти в папку `buzzer-beep`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/buzzer-beep
```

## Версия на rp-hal

Если хотите посмотреть тот же пример, реализованный с помощью rp-hal, его можно найти здесь.

```sh
git clone https://github.com/ImplFerris/pico2-rp-projects
cd pico2-rp-projects/buzzer-beep
```
