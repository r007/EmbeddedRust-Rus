{{#title Помощь и устранение неполадок для embedded Rust на Raspberry Pi Pico 2 | impl Rust для RP2350}}

# Помощь и устранение неполадок

Если при выполнении упражнений вы столкнётесь с ошибками, багами или другими проблемами, вот несколько способов их найти и устранить.

## 1. Сравните с рабочим кодом

Посмотрите полные примеры кода или склонируйте эталонный проект для сравнения. Внимательно проверьте свой код и версии зависимостей в `Cargo.toml`. Ищите синтаксические и логические ошибки. Если нужная функция (feature) не включена или features не совпадают, включите нужные features, как показано в упражнении.

Если вы нашли несоответствие версий, можно либо адаптировать код под более новую версию (разобраться самостоятельно — отличный способ лучше понять материал), либо обновить зависимости до версий, которые использовались в руководстве.

## 2. Поищите или создайте issue на GitHub

Откройте страницу issues на GitHub, чтобы проверить, не сталкивался ли кто-то с той же проблемой:
[https://github.com/r007/EmbeddedRust-Rus/issues?q=is%3Aissue](https://github.com/r007/EmbeddedRust-Rus/issues?q=is%3Aissue)

Если нет, создайте новый issue и ясно опишите проблему.

## 3. Спросите сообщество

Сообщество Rust Embedded активно в Matrix-чате. Matrix — открытая децентрализованная сеть для безопасного общения.

Вот несколько полезных каналов Matrix по темам книги:

- **Рабочая группа по embedded-устройствам**
  [`#rust-embedded:matrix.org`](https://matrix.to/#/#rust-embedded:matrix.org)
  Общие обсуждения использования Rust в embedded-разработке.

- **Разработка для семейства RP**
  [`#rp-rs:matrix.org`](https://matrix.to/#/#rp-rs:matrix.org)
  Разработка на Rust и обсуждения чипов Raspberry Pi серии RP.

- **Отладка с probe-rs**
  [`#probe-rs:matrix.org`](https://matrix.to/#/#probe-rs:matrix.org)
  Поддержка и обсуждение инструментария отладки [probe-rs](https://probe.rs).

- **Embedded Graphics**
  [`#rust-embedded-graphics:matrix.org`](https://matrix.to/#/#rust-embedded-graphics:matrix.org)
  Для работы с [`embedded-graphics`](https://docs.rs/embedded-graphics) — библиотекой для рисования в embedded-системах.

Вы можете создать аккаунт Matrix и присоединиться к этим каналам, чтобы получить помощь от опытных разработчиков.

Другие чаты сообщества можно найти в разделе [Community Chat Rooms в Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust?tab=readme-ov-file#community-chat-rooms).

## 4. Discord

Существует неофициальное сообщество Embedded Rust в Discord, где можно задавать вопросы, обсуждать темы, делиться опытом и показывать свои проекты. Оно особенно полезно новичкам и для общих обсуждений.

Имейте в виду, что мейнтейнеры большинства HAL и embedded-экосистемы более активны в Matrix. Тем не менее этот Discord-сервер тоже может стать хорошим местом для учёбы и общения.

Присоединиться: [https://discord.gg/NHenanPUuG](https://discord.gg/NHenanPUuG)
