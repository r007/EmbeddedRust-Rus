# Периферия как конечные автоматы

Периферию микроконтроллера можно рассматривать как набор конечных автоматов. Например, конфигурацию упрощённого [вывода GPIO] можно представить следующим деревом состояний:

[вывода GPIO]: https://en.wikipedia.org/wiki/General-purpose_input/output

* Отключён (Disabled)
* Включён (Enabled)
    * Настроен как выход (Configured as Output)
        * Выход: высокий уровень (Output: High)
        * Выход: низкий уровень (Output: Low)
    * Настроен как вход (Configured as Input)
        * Вход: высокий импеданс (Input: High Resistance)
        * Вход: подтянут к земле (Input: Pulled Low)
        * Вход: подтянут к питанию (Input: Pulled High)

Если периферия находится в режиме `Disabled`, то чтобы перейти в состояние `Input: High Resistance`, нужно выполнить следующие шаги:

1. Disabled
2. Enabled
3. Configured as Input
4. Input: High Resistance

Если мы хотим перейти из `Input: High Resistance` в `Input: Pulled Low`, нужно выполнить такие шаги:

1. Input: High Resistance
2. Input: Pulled Low

Аналогично, если мы хотим перевести вывод GPIO из состояния `Input: Pulled Low` в `Output: High`, нужно выполнить такие шаги:

1. Input: Pulled Low
2. Configured as Input
3. Configured as Output
4. Output: High

## Представление в аппаратуре

Обычно перечисленные выше состояния задаются записью значений в определённые регистры, отображённые на периферию GPIO. Чтобы проиллюстрировать это, определим воображаемый регистр конфигурации GPIO:

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

Мы _могли бы_ предоставить следующую структуру в Rust для управления этим GPIO:

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

    pub fn set_direction(&mut self, is_output: bool) {
        self.periph.modify(|_r, w| {
            w.direction().set_bit(is_output)
        });
    }

    pub fn set_input_mode(&mut self, variant: InputMode) {
        self.periph.modify(|_r, w| {
            w.input_mode().variant(variant)
        });
    }

    pub fn set_output_mode(&mut self, is_high: bool) {
        self.periph.modify(|_r, w| {
            w.output_mode.set_bit(is_high)
        });
    }

    pub fn get_input_status(&self) -> bool {
        self.periph.read().input_status().bit_is_set()
    }
}
```

Однако это позволит изменять некоторые регистры бессмысленным образом. Например, что произойдёт, если мы установим поле `output_mode`, когда наш GPIO настроен как вход?

В общем случае использование такой структуры позволило бы прийти к состояниям, не определённым нашим конечным автоматом выше: например, выход, подтянутый к земле, или вход, установленный в высокий уровень. Для некоторой аппаратуры это может не иметь значения. На другой аппаратуре это может привести к неожиданному или неопределённому поведению!

Хотя такой интерфейс удобно писать, он не обеспечивает соблюдения проектных контрактов, заданных нашей аппаратной реализацией.
