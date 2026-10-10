# Проектные контракты

В предыдущей главе мы написали интерфейс, который *не* обеспечивал соблюдение проектных контрактов. Давайте ещё раз посмотрим на наш воображаемый регистр конфигурации GPIO:

| Имя          | Номер бита(ов) | Значение | Смысл     | Примечания |
| ---:         | ------------: | ----: | ------:   | ----: |
| enable       | 0             | 0     | отключён  | Отключает GPIO |
|              |               | 1     | включён   | Включает GPIO |
| direction    | 1             | 0     | вход      | Задаёт направление: вход |
|              |               | 1     | выход     | Задаёт направление: выход |
| input_mode   | 2..3          | 00    | hi-z      | Задаёт вход с высоким сопротивлением |
|              |               | 01    | pull-low  | Входной вывод подтянут к земле |
|              |               | 10    | pull-high | Входной вывод подтянут к питанию |
|              |               | 11    | н/д       | Недопустимое состояние. Не устанавливать |
| output_mode  | 4             | 0     | set-low   | Выходной вывод управляется низким уровнем |
|              |               | 1     | set-high  | Выходной вывод управляется высоким уровнем |
| input_status | 5             | x     | in-val    | 0, если вход < 1,5 В; 1, если вход >= 1,5 В |

Если бы вместо этого мы проверяли состояние перед использованием базовой аппаратуры, обеспечивая соблюдение проектных контрактов во время выполнения, мы могли бы написать такой код:

```rust,ignore
/// Интерфейс GPIO
struct GpioConfig {
    /// Структура конфигурации GPIO, сгенерированная svd2rust
    periph: GPIO_CONFIG,
}

impl GpioConfig {
    pub fn set_enable(&mut self, is_enabled: bool) {
        self.periph.modify(|_r, w| {
            w.enable().set_bit(is_enabled)
        });
    }

    pub fn set_direction(&mut self, is_output: bool) -> Result<(), ()> {
        if self.periph.read().enable().bit_is_clear() {
            // Чтобы задать направление, вывод должен быть включён
            return Err(());
        }

        self.periph.modify(|r, w| {
            w.direction().set_bit(is_output)
        });

        Ok(())
    }

    pub fn set_input_mode(&mut self, variant: InputMode) -> Result<(), ()> {
        if self.periph.read().enable().bit_is_clear() {
            // Чтобы задать режим входа, вывод должен быть включён
            return Err(());
        }

        if self.periph.read().direction().bit_is_set() {
            // Направление должно быть входом
            return Err(());
        }

        self.periph.modify(|_r, w| {
            w.input_mode().variant(variant)
        });

        Ok(())
    }

    pub fn set_output_status(&mut self, is_high: bool) -> Result<(), ()> {
        if self.periph.read().enable().bit_is_clear() {
            // Чтобы задать состояние выхода, вывод должен быть включён
            return Err(());
        }

        if self.periph.read().direction().bit_is_clear() {
            // Направление должно быть выходом
            return Err(());
        }

        self.periph.modify(|_r, w| {
            w.output_mode.set_bit(is_high)
        });

        Ok(())
    }

    pub fn get_input_status(&self) -> Result<bool, ()> {
        if self.periph.read().enable().bit_is_clear() {
            // Чтобы получить состояние, вывод должен быть включён
            return Err(());
        }

        if self.periph.read().direction().bit_is_set() {
            // Направление должно быть входом
            return Err(());
        }

        Ok(self.periph.read().input_status().bit_is_set())
    }
}
```

Поскольку нам нужно обеспечивать соблюдение ограничений аппаратуры, мы в итоге много проверяем во время выполнения. Это тратит время и ресурсы, а такой код гораздо менее приятно использовать разработчику.

## Типовые состояния (Type States)

Но что, если вместо этого мы будем использовать систему типов Rust, чтобы обеспечивать соблюдение правил переходов между состояниями? Посмотрите на этот пример:

```rust,ignore
/// Интерфейс GPIO
struct GpioConfig<ENABLED, DIRECTION, MODE> {
    /// Структура конфигурации GPIO, сгенерированная svd2rust
    periph: GPIO_CONFIG,
    enabled: ENABLED,
    direction: DIRECTION,
    mode: MODE,
}

// Типовые состояния для MODE в GpioConfig
struct Disabled;
struct Enabled;
struct Output;
struct Input;
struct PulledLow;
struct PulledHigh;
struct HighZ;
struct DontCare;

/// Эти функции можно использовать для любого вывода GPIO
impl<EN, DIR, IN_MODE> GpioConfig<EN, DIR, IN_MODE> {
    pub fn into_disabled(self) -> GpioConfig<Disabled, DontCare, DontCare> {
        self.periph.modify(|_r, w| w.enable.disabled());
        GpioConfig {
            periph: self.periph,
            enabled: Disabled,
            direction: DontCare,
            mode: DontCare,
        }
    }

    pub fn into_enabled_input(self) -> GpioConfig<Enabled, Input, HighZ> {
        self.periph.modify(|_r, w| {
            w.enable.enabled()
             .direction.input()
             .input_mode.high_z()
        });
        GpioConfig {
            periph: self.periph,
            enabled: Enabled,
            direction: Input,
            mode: HighZ,
        }
    }

    pub fn into_enabled_output(self) -> GpioConfig<Enabled, Output, DontCare> {
        self.periph.modify(|_r, w| {
            w.enable.enabled()
             .direction.output()
             .input_mode.set_high()
        });
        GpioConfig {
            periph: self.periph,
            enabled: Enabled,
            direction: Output,
            mode: DontCare,
        }
    }
}

/// Эту функцию можно использовать для выхода
impl GpioConfig<Enabled, Output, DontCare> {
    pub fn set_bit(&mut self, set_high: bool) {
        self.periph.modify(|_r, w| w.output_mode.set_bit(set_high));
    }
}

/// Эти методы можно использовать для любого включённого входа GPIO
impl<IN_MODE> GpioConfig<Enabled, Input, IN_MODE> {
    pub fn bit_is_set(&self) -> bool {
        self.periph.read().input_status.bit_is_set()
    }

    pub fn into_input_high_z(self) -> GpioConfig<Enabled, Input, HighZ> {
        self.periph.modify(|_r, w| w.input_mode().high_z());
        GpioConfig {
            periph: self.periph,
            enabled: Enabled,
            direction: Input,
            mode: HighZ,
        }
    }

    pub fn into_input_pull_down(self) -> GpioConfig<Enabled, Input, PulledLow> {
        self.periph.modify(|_r, w| w.input_mode().pull_low());
        GpioConfig {
            periph: self.periph,
            enabled: Enabled,
            direction: Input,
            mode: PulledLow,
        }
    }

    pub fn into_input_pull_up(self) -> GpioConfig<Enabled, Input, PulledHigh> {
        self.periph.modify(|_r, w| w.input_mode().pull_high());
        GpioConfig {
            periph: self.periph,
            enabled: Enabled,
            direction: Input,
            mode: PulledHigh,
        }
    }
}
```

Теперь посмотрим, как выглядел бы код, который это использует:

```rust,ignore
/*
 * Пример 1: от неконфигурированного к входу High-Z
 */
let pin: GpioConfig<Disabled, _, _> = get_gpio();

// Так нельзя, вывод не включён!
// pin.into_input_pull_down();

// Теперь переводим вывод из неконфигурированного состояния во вход High-Z
let input_pin = pin.into_enabled_input();

// Читаем с вывода
let pin_state = input_pin.bit_is_set();

// Так нельзя: у входных выводов нет такого интерфейса!
// input_pin.set_bit(true);

/*
 * Пример 2: вход High-Z → вход с подтяжкой к земле
 */
let pulled_low = input_pin.into_input_pull_down();
let pin_state = pulled_low.bit_is_set();

/*
 * Пример 3: вход с подтяжкой к земле → выход, установленный в высокий уровень
 */
let output_pin = pulled_low.into_enabled_output();
output_pin.set_bit(true);

// Так нельзя: у выходных выводов нет такого интерфейса!
// output_pin.into_input_pull_down();
```

Это, безусловно, удобный способ хранить состояние вывода. Но зачем делать это именно так? Почему это лучше, чем хранить состояние в виде `enum` внутри нашей структуры `GpioConfig`?

## Функциональная безопасность на этапе компиляции

Поскольку мы обеспечиваем соблюдение наших проектных ограничений полностью на этапе компиляции, это не требует никаких затрат во время выполнения. Невозможно задать режим выхода, когда вывод находится в режиме входа. Вместо этого нужно пройти по состояниям: преобразовать вывод в выходной и только после этого задать режим выхода. Поэтому нет штрафа во время выполнения за проверку текущего состояния перед вызовом функции.

Кроме того, поскольку эти состояния обеспечиваются системой типов, у потребителей этого интерфейса не остаётся возможности ошибиться. Если они попытаются выполнить недопустимый переход между состояниями, код не скомпилируется!
