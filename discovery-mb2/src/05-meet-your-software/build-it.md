# Сборка

Первый шаг — собрать наш «бинарный» крейт. Поскольку архитектура микроконтроллера отличается от архитектуры вашего компьютера, придётся выполнять кросс-компиляцию. В мире Rust кросс-компиляция делается просто: нужно передать дополнительный флаг `--target` в `rustc` или Cargo. Сложнее всего понять, какой аргумент передать этому флагу: *название* цели (target).

Как мы уже знаем, в микроконтроллере micro:bit v2 стоит процессор Cortex-M4F. `rustc` умеет кросс-компилировать под архитектуру Cortex-M и предоставляет несколько целей, которые покрывают разные семейства процессоров в этой архитектуре:

- `thumbv6m-none-eabi` — для процессоров Cortex-M0 и Cortex-M1
- `thumbv7m-none-eabi` — для процессора Cortex-M3
- `thumbv7em-none-eabi` — для процессоров Cortex-M4 и Cortex-M7
- `thumbv7em-none-eabihf` — для процессоров Cortex-M4**F** и Cortex-M7**F**
- `thumbv8m.main-none-eabi` — для процессоров Cortex-M33 и Cortex-M35P
- `thumbv8m.main-none-eabihf` — для процессоров Cortex-M33**F** и Cortex-M35P**F**

«Thumb» здесь означает версию системы команд Arm с более короткими инструкциями, которые уменьшают размер кода (это игра слов). Части `hf`/`F` означают аппаратное ускорение операций с плавающей точкой. Благодаря этому вычисления с дробными числами («числами с плавающей точкой») работают заметно быстрее.

Для micro:bit v2 нам понадобится цель `thumbv7em-none-eabihf`.

Перед кросс-компиляцией нужно скачать предварительно собранную версию стандартной библиотеки (точнее, её урезанную версию) для вашей цели. Это делается с помощью `rustup`:

``` console
$ rustup target add thumbv7em-none-eabihf
```

Этот шаг нужно выполнить только один раз: затем `rustup` будет обновлять эту цель (переустанавливая новый компонент стандартной библиотеки `rust-std`, который содержит используемую нами библиотеку `core`) при каждом обновлении тулчейна. Поэтому этот шаг можно пропустить, если вы уже добавили нужную цель при [проверке установки].

[проверке установки]: ../03-setup/verify.html#проверка-cargo-embed


Когда компонент `rust-std` установлен, можно кросс-компилировать программу с помощью Cargo. Убедитесь, что вы находитесь в каталоге `mdbook/src/05-meet-your-software` репозитория Git, и выполните сборку. Этот начальный код — пример, поэтому собираем его как пример.

``` console
$ cargo build --example init
   Compiling semver-parser v0.7.0
   Compiling proc-macro2 v1.0.86
   ...

    Finished dev [unoptimized + debuginfo] target(s) in 33.67s
```

> **ПРИМЕЧАНИЕ** Убедитесь, что собираете этот крейт *без* оптимизаций. Предоставленный файл `Cargo.toml` и команда сборки выше гарантируют, что оптимизации выключены, пока вы *не* передаёте `cargo` флаг `--release`.

Итак, мы получили исполняемый файл. Он не будет мигать светодиодами: это упрощённая версия, на основе которой мы позже построим остальное в этой главе. Для проверки убедимся, что полученный файл действительно является бинарником для Arm. (Команда ниже эквивалентна

    readelf -h ../../../target/thumbv7em-none-eabihf/debug/examples/init

в системах, где есть `readelf`.)

``` console
$ cargo readobj --example init -- --file-headers
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           Arm
  Version:                           0x1
  Entry point address:               0x117
  Start of program headers:          52 (bytes into file)
  Start of section headers:          793112 (bytes into file)
  Flags:                             0x5000400
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         4
  Size of section headers:           40 (bytes)
  Number of section headers:         21
  Section header string table index: 19
```

Если ваши числа не совпадают в точности, не волнуйтесь: многое из этого сильно зависит от текущего окружения сборки.

Далее мы прошьём программу в наш микроконтроллер.
