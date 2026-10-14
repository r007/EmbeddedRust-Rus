# Программное обеспечение

Следуйте шагам ниже для стандартной установки инструментов платформы ESP32-C3.

🔎 Если вам нужна нестандартная установка (например, сборка компонентов из исходников или добавление поддержки целевых платформ `Xtensa`), инструкции есть в главе [Rust on ESP targets](https://esp-rs.github.io/book/installation/index.html) книги *Rust on ESP*.

## Тулчейн Rust

✅ Если Rust ещё не установлен на вашем компьютере, получите его на <https://rustup.rs/>.

Кроме того, для ESP32-C3 сейчас нужна [*nightly*-версия](https://rust-lang.github.io/rustup/concepts/channels.html#working-with-nightly-rust) тулчейна Rust. Для этого курса мы будем использовать версию `nightly-2024-06-30`.

✅ Установите *nightly*-версию Rust и добавьте поддержку целевой архитектуры командой:

```console
rustup toolchain install nightly-2024-06-30 --component rust-src
```

🔎 Rust умеет кросс-компилироваться под любую поддерживаемую цель (см. `rustup target list`). По умолчанию установлена только нативная архитектура вашей системы.
Для сборки под архитектуру `Xtensa` (она *не* входит в этот материал) по состоянию на январь 2022 года требуется форк компилятора Rust.

## Инструменты Espressif

Понадобится несколько инструментов:
- [`cargo-espflash`](https://github.com/esp-rs/espflash/tree/main/cargo-espflash) — загрузка прошивки в микроконтроллер и открытие serial-монитора с интеграцией в Cargo;
- [`espflash`](https://github.com/esp-rs/espflash/tree/main/espflash) — загрузка прошивки в микроконтроллер и открытие serial-монитора;
- [`ldproxy`](https://github.com/esp-rs/embuild/tree/master/ldproxy) — зависимость тулчейна сборки Espressif.

✅ Установите их командой:

```console
cargo install cargo-espflash espflash ldproxy
```

⚠️ Команды `espflash` и `cargo-espflash`, приведённые в книге, предполагают версию 2 или новее.

## Зависимости тулчейна

### Debian/Ubuntu

```console
sudo apt install llvm-dev libclang-dev clang libudev-dev libuv1-dev pkgconf python3-venv python-is-python3
```
### `macOS`

При использовании менеджера пакетов Homebrew, который мы рекомендуем:
```console
brew install llvm libuv
```

### Устранение неполадок

- Python 3 — обязательная зависимость. Он предустановлен в стандартной macOS и обычно в популярных дистрибутивах Linux для настольных компьютеров. Известно, что установленный **Python 2** с дополнением `virtualenv`, которое на него указывает, может вызывать проблемы со сборкой.

- Ошибка `failed to run custom build command for libudev-sys vX.X.X` или `esp-idf-sys vX.X.X`:

    На момент написания этого можно решить так:
    1. Выполните эту команду:

    `apt-get update \
    && apt-get install -y vim nano git curl gcc ninja-build cmake libudev-dev python3 python3-pip libusb-1.0-0 libssl-dev \
    pkg-config libtinfo5`

    2. Перезапустите терминал.

    3. Если не помогло, выполните `cargo clean`, удалите папку `~/.espressif` (в Windows — `%USERPROFILE%\.espressif`) и пересоберите проект.

    4. В Ubuntu может понадобиться сменить ядро на `5.19`. Версию ядра можно узнать командой `uname -r`.


## Docker

Альтернативный вариант — использовать Docker. В репозитории есть `Dockerfile` с инструкциями по установке тулчейна Rust и всех необходимых пакетов. **Эта виртуализированная среда предназначена для сборки бинарных файлов под целевую платформу Espressif. Прошивать бинарные файлы из контейнеров нельзя**, поэтому есть два варианта:
- Выполнять команды прошивки, например `cargo-espflash`, в основной системе. В этом случае рекомендуется держать открытыми два терминала:
    - в контейнере: собрать проект;
    - в основной системе: использовать подкоманду `cargo-espflash`, чтобы прошить программу в устройство.
- Использовать крейт [`web-flash`](https://github.com/esp-rs/esp-web-flash-server), чтобы прошить полученные из контейнера бинарные файлы. Контейнер уже содержит `web-flash`. Вот как прошить результат сборки [проекта `hardware-check`](./02_4_hello_board.md):
   ```console
   web-flash --chip esp32c3 target/riscv32imc-esp-espidf/debug/hardware-check
   ```

✅ Установите [`Docker`](https://docs.docker.com/get-docker/) для вашей операционной системы.

✅ Получите образ Docker. Есть два способа:
- Собрать образ Docker из `Dockerfile`:
    ```console
    docker image build --tag rust-std-training --file .devcontainer/Dockerfile .
    ```
    Сборка образа занимает некоторое время (20–30 минут) в зависимости от ОС и оборудования.
- Скачать его с [Docker Hub](https://hub.docker.com/r/espressif/rust-std-training):
    ```console
    docker pull espressif/rust-std-training
    ```
✅ Запустите новый контейнер Docker:

- Для локального образа Docker:
  ```console
  docker run --mount type=bind,source="$(pwd)",target=/workspace,consistency=cached -it rust-std-training /bin/bash
  ```
- С Docker Hub:
  ```console
  docker run --mount type=bind,source="$(pwd)",target=/workspace,consistency=cached -it espressif/rust-std-training:latest /bin/bash
  ```

Это запускает интерактивную оболочку в контейнере Docker. Также локальный репозиторий монтируется в папку `/workspace` внутри контейнера. Изменения в проекте в основной системе отражаются в контейнере и наоборот.

## Дополнительное программное обеспечение

### VS Code

Один из редакторов с хорошей поддержкой Rust — [VS Code](https://code.visualstudio.com/), он доступен для большинства платформ. При работе в VS Code мы рекомендуем следующие расширения, которые помогут в разработке:

* [`Rust Analyzer`](https://rust-analyzer.github.io/) — автодополнение и навигация по коду;
* `Even Better TOML` — редактирование конфигурационных файлов в формате TOML.

Есть и несколько полезных расширений для продвинутого использования:

* [`lldb`](https://github.com/vadimcn/vscode-lldb) — нативный отладчик на основе LLDB;
* [`crates`](https://github.com/serayuzgur/crates) — управление зависимостями Rust.

### VS Code и Dev Containers

Расширение для VS Code, которое может помочь в разработке внутри контейнера Docker, — [`Remote Containers`](https://github.com/Microsoft/vscode-remote-release).
Оно использует тот же `Dockerfile`, что и [настройка Docker](#docker), но собирает образ и подключается к нему прямо из VS Code. После установки расширения VS Code распознаёт конфигурацию в папке `.devcontainer`. Чтобы подключить VS Code к контейнеру, используйте команду `Remote Containers - Reopen in Container`.
