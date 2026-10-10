# Устранение неполадок

## Проблемы с `cargo-embed`

Большинство проблем с `cargo-embed` связано с тем, что правила `udev` в Linux установлены неправильно, так что убедитесь, что вы сделали всё правильно.

Если застряли, можно открыть issue в [трекере `discovery`] или зайти в [чат Rust Embedded в Matrix] или [чат probe-rs в Matrix] и попросить помощи.

[трекере `discovery`]: https://github.com/r007/EmbeddedRust-Rus/issues
[чат Rust Embedded в Matrix]: https://matrix.to/#/#rust-embedded:matrix.org
[чат probe-rs в Matrix]: https://matrix.to/#/#probe-rs:matrix.org

## Проблемы с Cargo

### "can't find crate for `core`"

*Симптомы:*

```
   Compiling volatile-register v0.1.2
   Compiling rlibc v1.0.0
   Compiling r0 v0.1.0
error[E0463]: can't find crate for `core`

error: aborting due to previous error

error[E0463]: can't find crate for `core`

error: aborting due to previous error

error[E0463]: can't find crate for `core`

error: aborting due to previous error

Build failed, waiting for other jobs to finish...
Build failed, waiting for other jobs to finish...
error: Could not compile `r0`.

To learn more, run the command again with --verbose.
```

*Причина:*

Вы забыли установить нужную цель (target) для вашего микроконтроллера `thumbv7em-none-eabihf`.

*Решение:*

Установите нужную цель.

``` console
$ rustup target add thumbv7em-none-eabihf
```

### Не удаётся прошить устройство: `No loadable segments were found in the ELF file`

*Симптомы:*
```console
> cargo embed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
      Config default
      Target /home/user/embedded/target/thumbv7em-none-eabihf/debug/examples/init
 WARN probe_rs::flashing::loader: No loadable segments were found in the ELF file.
       Error No loadable segments were found in the ELF file.
```

*Причина:*

Cargo должен знать, как собрать программу и как её слинковать под требования целевого устройства. Поэтому нужно задать правильные параметры в файле `.cargo/config.toml`.

*Решение:*

Добавьте файл `.cargo/config.toml` с правильными параметрами:
```toml
{{#include ../../05-meet-your-software/.cargo/config.toml}}
```

Подробности см. в разделе [Настройка для встраиваемой разработки](../../05-meet-your-software/embedded-setup.md).
