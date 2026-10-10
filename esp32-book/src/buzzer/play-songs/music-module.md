{{#title Модуль music для песен на пищалке ESP32 на embedded Rust}}

# Модуль music (music.rs)

В модуле music мы определяем константы для распространённых нот и соответствующие им значения частот.

```rust
// Частоты нот в герцах (f64)
pub const NOTE_B0: f64 = 31.0;
pub const NOTE_C1: f64 = 33.0;
pub const NOTE_CS1: f64 = 35.0;
pub const NOTE_D1: f64 = 37.0;
pub const NOTE_DS1: f64 = 39.0;
pub const NOTE_E1: f64 = 41.0;
pub const NOTE_F1: f64 = 44.0;
pub const NOTE_FS1: f64 = 46.0;
pub const NOTE_G1: f64 = 49.0;
pub const NOTE_GS1: f64 = 52.0;
pub const NOTE_A1: f64 = 55.0;
pub const NOTE_AS1: f64 = 58.0;
pub const NOTE_B1: f64 = 62.0;
pub const NOTE_C2: f64 = 65.0;
pub const NOTE_CS2: f64 = 69.0;
pub const NOTE_D2: f64 = 73.0;
pub const NOTE_DS2: f64 = 78.0;
pub const NOTE_E2: f64 = 82.0;
pub const NOTE_F2: f64 = 87.0;
pub const NOTE_FS2: f64 = 93.0;
pub const NOTE_G2: f64 = 98.0;
pub const NOTE_GS2: f64 = 104.0;
pub const NOTE_A2: f64 = 110.0;
pub const NOTE_AS2: f64 = 117.0;
pub const NOTE_B2: f64 = 123.0;
pub const NOTE_C3: f64 = 131.0;
pub const NOTE_CS3: f64 = 139.0;
pub const NOTE_D3: f64 = 147.0;
pub const NOTE_DS3: f64 = 156.0;
pub const NOTE_E3: f64 = 165.0;
pub const NOTE_F3: f64 = 175.0;
pub const NOTE_FS3: f64 = 185.0;
pub const NOTE_G3: f64 = 196.0;
pub const NOTE_GS3: f64 = 208.0;
pub const NOTE_A3: f64 = 220.0;
pub const NOTE_AS3: f64 = 233.0;
pub const NOTE_B3: f64 = 247.0;
pub const NOTE_C4: f64 = 262.0;
pub const NOTE_CS4: f64 = 277.0;
pub const NOTE_D4: f64 = 294.0;
pub const NOTE_DS4: f64 = 311.0;
pub const NOTE_E4: f64 = 330.0;
pub const NOTE_F4: f64 = 349.0;
pub const NOTE_FS4: f64 = 370.0;
pub const NOTE_G4: f64 = 392.0;
pub const NOTE_GS4: f64 = 415.0;
pub const NOTE_A4: f64 = 440.0;
pub const NOTE_AS4: f64 = 466.0;
pub const NOTE_B4: f64 = 494.0;
pub const NOTE_C5: f64 = 523.0;
pub const NOTE_CS5: f64 = 554.0;
pub const NOTE_D5: f64 = 587.0;
pub const NOTE_DS5: f64 = 622.0;
pub const NOTE_E5: f64 = 659.0;
pub const NOTE_F5: f64 = 698.0;
pub const NOTE_FS5: f64 = 740.0;
pub const NOTE_G5: f64 = 784.0;
pub const NOTE_GS5: f64 = 831.0;
pub const NOTE_A5: f64 = 880.0;
pub const NOTE_AS5: f64 = 932.0;
pub const NOTE_B5: f64 = 988.0;
pub const NOTE_C6: f64 = 1047.0;
pub const NOTE_CS6: f64 = 1109.0;
pub const NOTE_D6: f64 = 1175.0;
pub const NOTE_DS6: f64 = 1245.0;
pub const NOTE_E6: f64 = 1319.0;
pub const NOTE_F6: f64 = 1397.0;
pub const NOTE_FS6: f64 = 1480.0;
pub const NOTE_G6: f64 = 1568.0;
pub const NOTE_GS6: f64 = 1661.0;
pub const NOTE_A6: f64 = 1760.0;
pub const NOTE_AS6: f64 = 1865.0;
pub const NOTE_B6: f64 = 1976.0;
pub const NOTE_C7: f64 = 2093.0;
pub const NOTE_CS7: f64 = 2217.0;
pub const NOTE_D7: f64 = 2349.0;
pub const NOTE_DS7: f64 = 2489.0;
pub const NOTE_E7: f64 = 2637.0;
pub const NOTE_F7: f64 = 2794.0;
pub const NOTE_FS7: f64 = 2960.0;
pub const NOTE_G7: f64 = 3136.0;
pub const NOTE_GS7: f64 = 3322.0;
pub const NOTE_A7: f64 = 3520.0;
pub const NOTE_AS7: f64 = 3729.0;
pub const NOTE_B7: f64 = 3951.0;
pub const NOTE_C8: f64 = 4186.0;
pub const NOTE_CS8: f64 = 4435.0;
pub const NOTE_D8: f64 = 4699.0;
pub const NOTE_DS8: f64 = 4978.0;
pub const REST: f64 = 0.0; // Без звука, для пауз
```

Далее создаём небольшую вспомогательную `struct`, которая представляет музыкальную композицию (Song), и функции для расчёта длительности нот на основе темпа.

У этой структуры есть единственное поле `whole_note`, в котором хранится длительность целой ноты в миллисекундах. Мы храним длительность в миллисекундах, потому что музыкальное время часто задаётся темпом (ударов в минуту, BPM), и нам нужно вычислить, сколько времени длится каждая нота.

```rust

pub struct Song {
    whole_note: u32,
}
```

Формула `(60_000 * 4) / tempo as u32` вычисляет длительность целой ноты в миллисекундах. Мы используем 60 000, потому что в минуте 60 000 миллисекунд, а умножаем на 4, так как целая нота обычно равна четырём долям.

```rust
impl Song {
    pub fn new(tempo: u16) -> Self {
        let whole_note = (60_000 * 4) / tempo as u32;
        Self { whole_note }
    }
```

### `calc_note_duration`
Функция `calc_note_duration` вычисляет длительность музыкальной ноты относительно целой ноты. Она принимает параметр `divider`, который может быть положительным или отрицательным, и возвращает длительность ноты в миллисекундах.


```rust
    pub fn calc_note_duration(&self, divider: i16) -> u32 {
        if divider > 0 {
            self.whole_note / divider as u32
        } else {
            let duration = self.whole_note / divider.unsigned_abs() as u32;
            (duration as f64 * 1.5) as u32
        }
    }
}
```

#### Логика:

1. **Когда `divider > 0`:**
   - Если `divider` положительный, функция вычисляет длительность ноты, деля длительность целой ноты на `divider`.
   - Например, если `divider = 4`, функция вычисляет длительность четвертной ноты, то есть 1/4 целой ноты.

2. **Когда `divider <= 0`:**
   - Если `divider` отрицательный, функция сначала переводит его в положительное значение с помощью `unsigned_abs()`.
   - Затем она делит длительность целой ноты на это абсолютное значение и умножает результат на **1,5**, чтобы учесть **ноты с точкой** (например, четвертную ноту с точкой или восьмую с точкой), которые звучат в 1,5 раза дольше обычной ноты.

Такая логика с положительными и отрицательными значениями — собственный подход (я ориентировался на пример для Arduino), чтобы отличать ноты с точкой. Она не связана со стандартной музыкальной логикой.
