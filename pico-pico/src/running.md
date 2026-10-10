{{#title Как собрать и запустить embedded Rust на Raspberry Pi Pico 2 | impl Rust для RP2350}}

# Запуск программы

Прежде чем разбирать дальнейшие примеры, рассмотрим общие шаги сборки и запуска любой программы на Raspberry Pi Pico 2. Pico 2 содержит и процессоры ARM Cortex-M33, и RISC-V Hazard3, и мы приведём инструкции для обеих архитектур.

Примечание: эти команды нужно выполнять из папки вашего проекта. Они приведены здесь как общий шаг, чтобы не повторять его в каждом разделе. Если проект ещё не создан, начните с раздела «Быстрый старт» или «Мигание светодиодом».

## Сборка и запуск для ARM

Используйте эту команду, чтобы собирать и запускать программы на Raspberry Pi Pico 2 в режиме ARM, с процессорами Cortex-M33.

```sh
# собрать программу
cargo build --target=thumbv8m.main-none-eabihf
```

Чтобы прошить приложение на Pico 2, нажмите и удерживайте кнопку BOOTSEL. Удерживая её, подключите Pico 2 к компьютеру micro USB-кабелем. Кнопку можно отпустить, когда USB уже подключён.

<img style="display: block; margin: auto;" alt="кнопка BOOTSEL" src="./images/bootsel.png"/>

```sh
# Запустить программу
cargo run --target=thumbv8m.main-none-eabihf
```

> [!NOTE]
> В примерах кода в файле `.cargo/config.toml` задан runner:
`runner = "picotool load -u -v -x -t elf"`. Это значит, что при выполнении `cargo run` на самом деле вызывается `picotool` с подкомандой `load`, которая и прошивает программу.

## Сборка и запуск для RISC-V

Используйте эту команду, чтобы собирать и запускать программы на Raspberry Pi Pico 2 в режиме RISC-V, с процессорами Hazard3.

> [!IMPORTANT]
> Эта книга сосредоточена на ARM. Некоторые примеры могут потребовать изменений, чтобы заработать в режиме RISC-V. Для простоты при чтении книги рекомендуется следовать процессу для ARM.

```sh
# собрать программу
cargo build --target=riscv32imac-unknown-none-elf
```

Выполните те же действия с BOOTSEL, что описаны выше.

```sh
# Запустить программу
cargo run --target=riscv32imac-unknown-none-elf
```

## С отладочным зондом

При использовании отладочного зонда можно прошить программу на Pico 2 напрямую:

```sh
# cargo flash --chip RP2350
# cargo flash --chip RP2350 --release
cargo flash --release
```

Если хотите одновременно прошить программу и видеть её вывод в реальном времени, используйте:

```sh
# cargo embed --chip RP2350
# cargo embed --chip RP2350 --release
cargo embed --release
```

[cargo-embed](https://probe.rs/docs/tools/cargo-embed/) — более продвинутая версия cargo-flash. Он умеет прошивать программу, а также открывать RTT-терминал и GDB-сервер.
