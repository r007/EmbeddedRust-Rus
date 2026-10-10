# Подготовка

В этой главе описаны учебные материалы, необходимое оборудование и инструкции по установке.

## Значки и форматирование

В книге мы используем значки, чтобы выделять разные виды информации:
* ✅ Призыв к действию.
* ⚠️ Предупреждения и детали, требующие особого внимания.
* 🔎 Сведения, которые углубляются в тему, но не обязательны для понимания дальнейшего материала.
* 💡 Подсказки, которые могут помочь при выполнении упражнений.

> Пример заметки: заметки такого вида содержат полезную информацию.

## Аннотации в коде

В некоторых файлах Rust встречаются якорные комментарии:
```rust,ignore
// ANCHOR: test
let foo = 1;
...
// ANCHOR_END: test
```
Якорные комментарии можно игнорировать: они нужны только для того, чтобы вставлять фрагменты кода в эту книгу. См. [документацию `mdBook`](https://rust-lang.github.io/mdBook/format/mdbook.html#including-portions-of-a-file).

## Необходимое оборудование

- [Rust ESP Board](https://github.com/esp-rs/esp-rust-board): доступна на Mouser и AliExpress. [Полный список продавцов](https://github.com/esp-rs/esp-rust-board#where-to-buy).
- Кабель USB-C, подходящий для подключения платы к компьютеру разработчика.
- Точка доступа Wi-Fi с выходом в Интернет.

> Дополнительный отладчик или программатор не нужен.

## Симуляция проектов

Некоторые проекты можно симулировать, хотя бы частично, с помощью [Wokwi][wokwi]. Вот список проектов с поддержкой симуляции Wokwi:
- `intro/hello-world`
- `intro/http-client`
- `advanced/i2c-sensor-reading`
- `advanced/i2c-driver`
- `advanced/button-interrupt`

Перед запуском любого проекта для симуляции нужно [установить расширение][wokwi-installation]. Чтобы симулировать один проект:

1. Нажмите F1, выберите `Wokwi: Select Config File` и укажите `wokwi.toml` нужного проекта.
   1. Отредактируйте соответствующий `wokwi.toml`, чтобы симулировать упражнение или решение.
2. Соберите проект в режиме `debug`.
3. Снова нажмите F1 и выберите `Wokwi: Start Simulator`.

Также можно [отладить проект][wokwi-debug].

[wokwi]: https://wokwi.com/
[wokwi-installation]: https://docs.wokwi.com/vscode/getting-started#installation
[wokwi-debug]: https://docs.wokwi.com/vscode/debugging

## Проверка готовности к работе
<!-- TODO: Update this comments -->

⚠️ Если вы участвуете в курсе, который ведёт Ferrous Systems, настоятельно советуем подготовиться по инструкциям этой главы хотя бы за один рабочий день до начала, чтобы убедиться, что всё готово. Если возникнут проблемы или понадобится поддержка, [свяжитесь с нами](https://ferrous-systems.com/contact/).

⚠️ Если вы используете [ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html), некоторые выводы и адреса ведомых устройств будут отличаться: плата похожа, но не идентична. Это касается решений в [advanced/i2c-sensor-reading/](https://github.com/r007/EmbeddedRust-Rus/tree/main/std-training/advanced/i2c-sensor-reading/examples) и [advanced/i2c-driver/](https://github.com/r007/EmbeddedRust-Rus/tree/main/std-training/advanced/i2c-driver/src), где выводы и адреса для ESP32-C3-DevKitC-02 закомментированы.

## Сопутствующие материалы

- [Официальная книга esp-rs](https://esp-rs.github.io/book/introduction.html)
