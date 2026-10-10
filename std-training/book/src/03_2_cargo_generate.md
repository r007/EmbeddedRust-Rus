# Создание новых проектов

Теперь мы используем [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) (универсальный мастер создания проектов), чтобы настроить наше первое приложение.

Подробнее о создании проектов — в главе [Writing Your Own Application](https://esp-rs.github.io/book/writing-your-own-application/index.html) книги The Rust on ESP Book.

> Большинство других упражнений этого практикума уже содержат готовую заготовку проекта и не требуют `cargo-generate`.
>
✅ Установите `cargo-generate`:

```console
cargo install cargo-generate
```

✅ Перейдите в папку `intro` и запустите `cargo generate` с [шаблоном `esp-idf`](https://github.com/esp-rs/esp-idf-template):

```console
cd intro
cargo generate esp-rs/esp-idf-template cargo
```

[Мастер запросит данные о вашем новом проекте](https://github.com/esp-rs/esp-idf-template#generate-the-project). Если предлагается выбор из нескольких вариантов, переключайтесь стрелками вверх и вниз и выбирайте клавишей Enter.

Первое сообщение, которое вы увидите:
`⚠️Unable to load config file: /home/$USER/.cargo/cargo-generate.toml`. Эта ошибка возникает, потому что у вас нет файла избранных настроек, но он вам не нужен, и это предупреждение можно игнорировать.

🔎 Вы можете создать [файл избранных настроек](https://cargo-generate.github.io/cargo-generate/favorites.html), который будет размещён в `$CARGO_HOME/cargo-generate`, и переопределить его через `-c, --config <config-file>`.

> Если ошиблись, нажмите `Ctrl+C` и начните заново.

✅ Настройте проект:

(Эти пункты могут появиться в другом порядке)

* Project Name: `hello-world`
* MCU: `esp32c3`
* Configure advanced template options?: `false`

🔎 `.cargo/config.toml` содержит локальные настройки ([список всех настроек](https://doc.rust-lang.org/cargo/reference/config.html)) для вашего пакета.
`Cargo.toml` содержит зависимости, а `Cargo.lock` [фиксирует все ваши зависимости](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html).

Необязательно, но рекомендуется: чтобы сэкономить место на диске и время загрузки, задайте [общий каталог инструментария](https://github.com/esp-rs/esp-idf-sys/blob/master/BUILD-OPTIONS.md#esp_idf_tools_install_dir-esp_idf_tools_install_dir). Иначе каждый новый проект или рабочее пространство получит собственную копию инструментария на вашем компьютере:


✅ Откройте `hello-world/.cargo/config.toml` и добавьте следующую строку в конец секции `[env]`. Остальное не меняйте.

```toml
[env]
# ...
ESP_IDF_TOOLS_INSTALL_DIR = { value = "global" } # добавьте эту строку
```

✅ Откройте `hello-world/rust-toolchain.toml` и приведите файл к такому виду:

```toml
[toolchain]
channel = "nightly-2025-01-01" # измените эту строку
```

✅ Запустите проект следующей командой, находясь в папке `hello-world`:

```console
cd hello-world
cargo run
```

✅ Последние строки вывода должны выглядеть так:

```console
(...)
I (268) cpu_start: Starting scheduler.
Hello, world!
```

## Дополнительные задания
- Если функция `main` завершается, для повторного запуска нужно перезагрузить микроконтроллер. Что произойдёт, если вместо этого в конце поставить бесконечный цикл? Проверьте свою догадку, прошив программу с циклом.
- Можете придумать, как избежать того, что вы сейчас видите? (нажмите, чтобы увидеть подсказку:[^hint])

## Устранение неполадок
- Если `cargo run` зависает на `Connecting...`, возможно, ещё работает другой процесс монитора (например, оставшийся после начальной проверки `hardware-check`). Попробуйте найти его и завершить. Если не помогло, отключите и снова подключите USB-кабель платы.
- `⛔ Git Error: authentication required`: скорее всего, ваша конфигурация git подменяет HTTPS-адреса GitHub на SSH. Проверьте глобальный `~/.git/config` на наличие секций `insteadOf` и отключите их.

[^hint]: отдайте управление операционной системе, вызывая `sleep` в цикле вместо активного ожидания (busy waiting). (используйте `use std::thread::sleep`)
