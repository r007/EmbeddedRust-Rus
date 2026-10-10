# Ferris на OLED

В этом упражнении мы выведем `ferris.bmp` ([скачать](../images/ferris.bmp) и поместить в папку проекта) на OLED-экран.

<img style="display: block; margin: auto;" alt="Ferris" src="../images/ferris.bmp"/>

Следуйте тем же инструкциям, что и в программе «Hello Rust», с небольшими изменениями.

## Новая зависимость

Нам понадобится крейт tinybmp для загрузки BMP-файлов. Чтобы подключить его к проекту, используйте команду cargo:

```sh
cargo add tinybmp@0.6.0
```

## Дополнительные импорты

```rust
use embedded_graphics::image::Image;
use tinybmp::Bmp;
```

## Отличия

После инициализации дисплея загрузим файл ferris.bmp с помощью крейта tinybmp и затем выведем изображение.
```rust
let bmp = Bmp::from_slice(include_bytes!("../ferris.bmp")).unwrap();
let im = Image::new(&bmp, Point::new(32, 0));
im.draw(&mut display).unwrap();
```

## Полный код

```rust
```

## Клонируйте существующий проект

Можно клонировать (или посмотреть) проект, который я создал, и перейти в папку `ferris-oled`.

```sh
git clone https://github.com/ImplFerris/pico2-rp-projects
cd pico2-rp-projects/ferris-oled
```

## Полезные инструменты

- Конвертер растрового изображения в массив Rust: [Image2Bytes Generator](https://implferris.github.io/image2bytes/)
