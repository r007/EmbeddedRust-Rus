{{#title Прошивка Rust-программы в Raspberry Pi Pico 2 с помощью Picotool | impl Rust для RP2350}}

# Прошивка Rust-программы в Raspberry Pi Pico 2

После сборки программы у нас появится ELF-файл, готовый к прошивке.

Для отладочной сборки (`cargo build`) файл находится здесь:

```sh
./target/thumbv8m.main-none-eabihf/debug/pico-from-scratch
```

Для релизной сборки (`cargo build --release`) — здесь:

```sh
./target/thumbv8m.main-none-eabihf/release/pico-from-scratch
```

Чтобы загрузить программу на Pico, мы воспользуемся инструментом [Picotool](https://github.com/raspberrypi/picotool). Вот команда для прошивки нашей программы:

```rust
picotool load -u -v -x -t elf ./target/thumbv8m.main-none-eabihf/debug/pico-from-scratch
```

Вот что делает каждый флаг:

- `-u` — режим обновления (записываются только изменившиеся данные)
- `-v` — проверить, что всё записано корректно
- `-x` — запустить программу сразу после загрузки
- `-t elf` — сообщает picotool, что мы используем ELF-файл

## Команда cargo run

Каждый раз вводить эту длинную команду утомительно. Упростим её, обновив файл `.cargo/config.toml`. Мы можем настроить Cargo так, чтобы при запуске `cargo run` он автоматически использовал picotool:

```toml
[target.thumbv8m.main-none-eabihf]
runner = "picotool load -u -v -x -t elf"
```

Теперь достаточно ввести:

```sh
cargo run --release

# или

cargo run
```

и программа будет прошита и запущена на Pico.

Пока что ничего на самом деле не прошьётся. Нам не хватает одного важного шага.
