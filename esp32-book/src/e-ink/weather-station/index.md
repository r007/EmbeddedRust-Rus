{{#title Погодная панель на Rust с ESP32 и e-Paper дисплеем}}

# Пишем код на Rust для погодной панели на e-Paper с ESP32

Давайте сделаем с e-ink дисплеем что-нибудь поинтереснее, чем вывод статичного текста или изображений. Мы создадим простую метеостанцию, которая получает актуальные данные о погоде из интернета по HTTP API и обновляет дисплей свежей информацией. Это что-то вроде программы «Hello, World» для e-paper.

## Предварительные требования

Мы будем использовать API сайта «openweathermap.org», чтобы получать данные о погоде. Сайт выдаёт бесплатный API-ключ с лимитом 1000 запросов в день, этого должно хватить. Зайдите на сайт, зарегистрируйте бесплатный аккаунт, и вы получите собственный API-ключ.


### Создание проекта с помощью esp-generate

Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 weather-station
```

Откроется экран с вопросом о выборе опций.

- Сначала выберите опцию «Enable unstable HAL features.»
- Выберите опцию «Enable allocations via the esp-alloc crate.»
- Затем можно включить «Enable Wi-Fi via esp-radio crate.»
- Выберите опцию «Adds embassy framework support».



Сохраните настройки, нажав на клавиатуре «s».


## Зависимости

Обновите embassy-net и добавьте feature «dns»:

```toml
embassy-net = { version = "0.7.1", features = [
  "defmt",
  "dhcpv4",
  "medium-ethernet",
  "tcp",
  "udp",
  "dns",
] }
```

**Дополнительные зависимости:**

Вкратце расскажу о новых зависимостях. Крейты `embedded-graphics` и `tinybmp` мы уже использовали много раз, в том числе в прошлой главе, так что вы должны быть с ними хорошо знакомы. Мы будем использовать их, чтобы рисовать фигуры и загружать BMP-файлы.

```toml
embedded-graphics = "0.8.1"
tinybmp = "0.6.0"
profont = "0.7.0"
chrono = { version = "0.4.40", default-features = false, features = ["serde"] }
serde = { version = "1.0.219", default-features = false, features = ["derive"] }
serde-json-core = "0.6.0"
serde_repr = "0.1.20"

epd-waveshare = { features = [
  "graphics",
], git = "https://github.com/ImplFerris/epd-waveshare" }

embedded-hal-bus = { version = "0.3" }

heapless = { version = "0.9.2", features = ["serde"] }

reqwless = { version = "0.13.0", default-features = false, features = [
  "embedded-tls",
  "alloc",
] }
```


- [profont](https://docs.rs/profont/latest/profont/): крейт предоставляет моноширинный программистский шрифт ProFont для embedded-graphics. Мы используем его, потому что для погодной панели нужен шрифт немного большего размера.
- [chrono](https://docs.rs/chrono/latest/chrono/): для работы с датой и временем. Мы будем использовать его, чтобы десериализовать текущую дату и время, полученные из API.
- [serde](https://docs.rs/serde/latest/serde/): фреймворк для сериализации и десериализации структур данных Rust.
- [serde_json_core](https://docs.rs/serde-json-core/latest/serde_json_core/): для десериализации JSON в структуру Rust мы используем этот крейт. Он создан специально для окружений no_std, а обычно вместо него применяют serde_json.
- [serde_repr](https://docs.rs/serde_repr/latest/serde_repr/): крейт позволяет сериализовать и десериализовать перечисления по их числовым значениям, что упрощает сопоставление числовых кодов погодных условий из API (например, 200 или 201) с вариантами перечисления Rust.
- [reqwless](https://docs.rs/reqwless/latest/reqwless/index.html): мы уже использовали этот крейт — это HTTP-клиент, который работает в окружении no_std. С его помощью будем отправлять запросы к API и получать ответы.
- [epd-waveshare](https://docs.rs/epd-waveshare/latest/epd_waveshare/): простой драйвер для e-Ink дисплеев Waveshare по SPI. Оригинальный крейт не работал должным образом с 1,54-дюймовым вариантом, поэтому мне пришлось сделать форк и пропатчить код. Это скорее обходное решение, чем полноценное исправление, поэтому я пока не отправлял PR в оригинальный репозиторий. Пока будем использовать форк.


## Структура проекта
Это общая структура проекта, и мы пройдём её по шагам.

```
├── build.rs
├── src
│   ├── bin
│   │   └── main.rs
│   ├── ca_cert.pem
│   ├── dashboard.rs
│   ├── icons
│   │   ├── air.bmp
│   │   ├── ...bmp
│   │   └── ...bmp
│   ├── icons.rs
│   ├── lib.rs
│   ├── weather.rs
│   └── wifi.rs
├── ...

```

Так как глава получится большой, рекомендую сверяться с готовым проектом [здесь](https://github.com/ImplFerris/esp32-epaper-weather/). Там могут быть импорты, которые я не буду описывать в руководстве, и другие второстепенные детали.

## Модуль lib 

В `lib.rs` мы объявляем подмодули вместе с вспомогательным макросом, который использовали в разделе о Wi-Fi. Этот макрос резервирует память на этапе компиляции для значения `static`, но позволяет инициализировать его во время выполнения.

```rust
#![no_std]
pub mod dashboard;
pub mod icons;
pub mod weather;
pub mod wifi;

#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}
```
