{{#title Как настроить тулчейны Rust для разработки на ESP32}}

# Тулчейн

Как только вы добавите target в файл `config.toml` и попытаетесь собрать проект командой `cargo build`, сборка перестанет работать. Вы увидите примерно такую ошибку:

```sh
'esp32' is not a recognized processor for this target (ignoring processor)
'esp32' is not a recognized processor for this target (ignoring processor)
error[E0463]: can't find crate for `std`
  |
  = note: the `xtensa-esp32-none-elf` target may not be installed
  = help: consider downloading the target with `rustup target add xtensa-esp32-none-elf`

For more information about this error, try `rustc --explain E0463`.
error: could not compile `std_to_no_std` (bin "std_to_no_std") due to 1 previous error
```

Сообщение говорит, что esp32 не распознан как процессор для этой цели, и жалуется, что target, возможно, не установлен. Но постойте: разве мы не установили всё с помощью espup? Установили.

Так что же произошло? По умолчанию Rust использует глобальный тулчейн вашей системы. Он ничего не знает о ESP32. Утилита espup установила отдельный тулчейн с именем «esp» (его тоже можно назвать иначе). Нам нужно лишь указать Rust использовать его.

В корне проекта создайте файл `rust-toolchain.toml` и добавьте в него:

```toml
[toolchain]
channel = "esp"
```

Вместо тулчейна «esp» по умолчанию можно использовать собственный. В этой книге я использую собственный тулчейн «book-1.0.0». Главное — убедитесь, что ваш тулчейн установлен и вы указываете его имя точно; иначе появится ошибка, что тулчейн не установлен.

## Build-std

Попробуйте собрать проект снова. Сборка всё равно не пройдёт, но на этот раз ошибка выглядит немного иначе: теперь говорится, что не удаётся найти крейт для std.

```sh
error[E0463]: can't find crate for `std`
  |
  = note: the `xtensa-esp32-none-elf` target may not be installed
  = help: consider downloading the target with `rustup target add xtensa-esp32-none-elf`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`

For more information about this error, try `rustc --explain E0463`.
error: could not compile `std_to_no_std` (bin "std_to_no_std") due to 1 previous error
```

Дело в том, что target ESP32 — это [Tier 3 target](https://doc.rust-lang.org/beta/rustc/target-tier-policy.html#tier-3-target-policy). Это значит, что Rust не поставляет для него готовые стандартные библиотеки. Нам придётся собрать библиотеку core из исходников.

Чтобы это сделать, обновите файл `.cargo/config.toml`:
```toml
[unstable]
build-std = ["core"]
```

Попробуйте собрать проект ещё раз. Он всё равно не соберётся. Но не переживайте, мы уже близко. В следующем разделе исправим следующую группу ошибок.
