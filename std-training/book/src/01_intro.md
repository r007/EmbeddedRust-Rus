<p style="text-align:center;"><img src="./assets/esp-logo-black.svg" width="50%"></p>

# Введение

## Содержание материала

Это учебный материал Ferrous Systems «Embedded Rust на Espressif». Он разделён на два практикума: вводный и продвинутый. Вводный знакомит с основами разработки встраиваемых систем и с тем, как заставить плату взаимодействовать с внешним миром: реагировать на команды и отправлять данные с датчиков.

Продвинутый курс развивает эти темы: обработка прерываний, низкоуровневый доступ к периферии и написание собственных драйверов.

Вы можете присоединиться к [сообществу esp-rs](https://matrix.to/#/#esp-rs:matrix.org) в Matrix и задавать там любые технические вопросы. Сообщество открыто для всех.

> ⚠️ Эта книга использует экосистему `std` (`esp-idf-hal`, `esp-idf-sys`, `esp-idf-svc`). Её поддерживает сообщество, и эти крейты официально не поддерживаются Espressif.
>
> Espressif официально поддерживает подход `no_std` через [`esp-hal`][esp-hal]. Эквивалентный материал для официально поддерживаемого подхода см. в [курсе по `no_std`][no-std-training].

[esp-hal]: https://github.com/esp-rs/esp-hal
[no-std-training]: https://docs.espressif.com/projects/rust/no_std-training/

## Переводы

Эта книга переведена добровольцами. Если вы хотите, чтобы ваш перевод был указан здесь, откройте PR с его добавлением.

- [简体中文](https://narukara.github.io/std-training-zh-cn/) ([репозиторий](https://github.com/Narukara/std-training-zh-cn))

## Плата

[Rust ESP Board](https://github.com/esp-rs/esp-rust-board) обязательна[^note] для работы с этой книгой: эмуляторы вроде QEMU не поддерживаются.


Дизайн и изображения платы, распиновка и схемы также доступны в этом репозитории.

Если вы записались на один из курсов, плату вам предоставит Espressif.

Мы в первую очередь работаем с платформой [ESP32-C3](https://www.espressif.com/en/products/socs/esp32-c3) — микроконтроллером на архитектуре [`RISC-V`](https://riscv.org/) с широкими возможностями для IoT: встроенными Wi-Fi и Bluetooth 5 (LE), а также большими объёмами RAM и flash-памяти для сложных приложений. Значительная часть курса применима и к `Xtensa`, другой архитектуре Espressif, в частности к [ESP32-S3](https://www.espressif.com/en/products/socs/esp32-s3). Общие принципы низкоуровневого доступа тоже сохраняются, но работа с конкретным железом во многом отличается. При необходимости обращайтесь к техническим справочным руководствам ([C3](https://www.espressif.com/sites/default/files/documentation/esp32-c3_technical_reference_manual_en.pdf), [S3](https://www.espressif.com/sites/default/files/documentation/esp32-s3_technical_reference_manual_en.pdf)) или к [другим техническим документам](https://www.espressif.com/en/support/documents/technical-documents).

## Знание Rust

- Базовый Rust, как в [The Rust Book](https://doc.rust-lang.org/book/), главы 1–6. Глава 4 «Владение» не обязательна для полного понимания.
- [The Rust on ESP Book](https://esp-rs.github.io/book/) не обязательна, но очень рекомендуется: она поможет разобраться в экосистеме Rust на ESP и во многих концепциях, которые будут разбираться на курсе.

[^note]: Вводную часть можно пройти и на [ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html), но мы этого не рекомендуем. Курс заметно проще проходить на том же оборудовании.
