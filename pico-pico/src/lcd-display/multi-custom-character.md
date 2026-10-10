{{#title Многоячеистый пользовательский символ в embedded-graphics для OLED на Raspberry Pi Pico 2}}

# Многоячеистый пользовательский символ

В этом разделе мы создадим Ferris из шести соседних сеток с помощью генератора с предыдущей страницы. Ниже приведён код на Rust, который использует сгенерированные массивы байтов, чтобы вывести глиф на ЖК-дисплей.

<img style="display: block; margin: auto;width:400px;" alt="Ferris из шести ячеек на LCD-дисплее" src="./images/ferris-with-6-grids-on-lcd-display.png"/>

Здесь мы сосредоточимся только на составлении глифа. Настройка проекта и инициализация ЖК-дисплея остаются такими же, как раньше, и в этом разделе повторяться не будут.

### Сгенерированные массивы байтов для символов

```rust
const SYMBOL1: [u8; 8] = [
    0b00110, 0b01000, 0b01110, 0b01000, 0b00100, 0b00011, 0b00100, 0b01000,
];

const SYMBOL2: [u8; 8] = [
    0b00000, 0b00000, 0b00000, 0b10001, 0b10001, 0b11111, 0b00000, 0b00000,
];

const SYMBOL3: [u8; 8] = [
    0b01100, 0b00010, 0b01110, 0b00010, 0b00100, 0b11000, 0b00100, 0b00010,
];

const SYMBOL4: [u8; 8] = [
    0b01000, 0b01000, 0b00100, 0b00011, 0b00001, 0b00010, 0b00101, 0b01000,
];

const SYMBOL5: [u8; 8] = [
    0b00000, 0b00000, 0b00000, 0b11111, 0b01010, 0b10001, 0b00000, 0b00000,
];

const SYMBOL6: [u8; 8] = [
    0b00010, 0b00010, 0b00100, 0b11000, 0b10000, 0b01000, 0b10100, 0b00010,
];
```

### Объявляем их как символы

Каждый глиф хранится в отдельной ячейке CGRAM. В этом примере мы используем ячейки с 0 по 5.

```rust
lcd.custom_char(&mut Delay, &SYMBOL1, 0);
lcd.custom_char(&mut Delay, &SYMBOL2, 1);
lcd.custom_char(&mut Delay, &SYMBOL3, 2);
lcd.custom_char(&mut Delay, &SYMBOL4, 3);
lcd.custom_char(&mut Delay, &SYMBOL5, 4);
lcd.custom_char(&mut Delay, &SYMBOL6, 5);
```

### Вывод

Первые три глифа выводим в первой строке, а оставшиеся три — во второй строке, выравнивая их так, чтобы получился один составной символ.

```rust
lcd.set_cursor(&mut Delay, 0, 4)
    .write(&mut Delay, CustomChar(0))
    .write(&mut Delay, CustomChar(1))
    .write(&mut Delay, CustomChar(2));

lcd.set_cursor(&mut Delay, 1, 4)
    .write(&mut Delay, CustomChar(3))
    .write(&mut Delay, CustomChar(4))
    .write(&mut Delay, CustomChar(5));
```

## Клонируйте существующий проект

Можно клонировать проект, который я создал, и перейти в папку `mutli-glyph`.

```sh
git clone https://github.com/ImplFerris/pico2-embassy-projects
cd pico2-embassy-projects/lcd/mutli-glyph/
```

## Версия на rp-hal

Версия с использованием rp-hal также доступна:

```sh
git clone https://github.com/ImplFerris/pico2-rp-projects
cd pico2-rp-projects/lcd/mutli-glyph/
```
