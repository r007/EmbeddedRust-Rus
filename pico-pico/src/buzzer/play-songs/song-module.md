# Пример мелодии: тема Game of Thrones

Этот раздел содержит фрагменты кода для модуля Rust `got`.

## Импорт определений музыки

Модуль `got` использует константы нот и вспомогательные типы, определённые в модуле `music`. Подключаем их следующим импортом:

```rust
use crate::music::*;
```

Это позволяет мелодии использовать константы нот, например `NOTE_E4` и `NOTE_A4`, напрямую, без записи имени модуля каждый раз.

## Темп

Задаём темп песни. Вы можете изменить это значение и посмотреть, как это влияет на скорость воспроизведения.

```rust
pub const TEMPO: u16 = 85;
```

## Массив мелодии

Мелодию темы Game of Thrones задаём массивом нот и длительностей. Каждый элемент — кортеж, который содержит частоту ноты и её длительность.

Длительность задаётся целым числом. Положительные значения обозначают обычные ноты, отрицательные — ноты с точкой.

```rust
pub const MELODY: [(f64, i16); 92] = [
    // Тема Game of Thrones
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_E4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_E4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_E4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_E4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, -4),
    (NOTE_C4, -4),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 4),
    (NOTE_C4, 4),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_D4, -1),
    (NOTE_F4, -4),
    (NOTE_AS3, -4),
    (NOTE_DS4, 16),
    (NOTE_D4, 16),
    (NOTE_F4, 4),
    (NOTE_AS3, -4),
    (NOTE_DS4, 16),
    (NOTE_D4, 16),
    (NOTE_C4, -1),
    // Повтор
    (NOTE_G4, -4),
    (NOTE_C4, -4),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 4),
    (NOTE_C4, 4),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_D4, -1),
    (NOTE_F4, -4),
    (NOTE_AS3, -4),
    (NOTE_DS4, 16),
    (NOTE_D4, 16),
    (NOTE_F4, 4),
    (NOTE_AS3, -4),
    (NOTE_DS4, 16),
    (NOTE_D4, 16),
    (NOTE_C4, -1),
    (NOTE_G4, -4),
    (NOTE_C4, -4),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 4),
    (NOTE_C4, 4),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_D4, -2),
    (NOTE_F4, -4),
    (NOTE_AS3, -4),
    (NOTE_D4, -8),
    (NOTE_DS4, -8),
    (NOTE_D4, -8),
    (NOTE_AS3, -8),
    (NOTE_C4, -1),
    (NOTE_C5, -2),
    (NOTE_AS4, -2),
    (NOTE_C4, -2),
    (NOTE_G4, -2),
    (NOTE_DS4, -2),
    (NOTE_DS4, -4),
    (NOTE_F4, -4),
    (NOTE_G4, -1),
];
```
