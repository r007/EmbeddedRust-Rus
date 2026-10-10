# Привет, плата!

Теперь всё готово для проверки согласованности!

✅ Подключите USB-C порт платы к компьютеру и перейдите в папку `hardware-check` репозитория практикума:

```console
cd intro/hardware-check
```

Чтобы проверить подключение к Wi-Fi, нужно указать имя сети (SSID) и пароль (PSK). Эти данные хранятся в отдельном файле `cfg.toml` (он добавлен в `.gitignore`), чтобы их случайно не раскрыли при публикации исходного кода или создании pull request. Пример такого файла есть в репозитории.

✅ Скопируйте `cfg.toml.example` в `cfg.toml` (в той же папке) и отредактируйте его, указав свои данные:

> ⚠️ [Диапазон 5 ГГц не поддерживается в ESP32-C3](https://www.espressif.com/en/news/ESP32-C3_Wi-Fi_Certified#:~:text=ESP32%2DC3%20is%20a%20safe,wide%20range%20of%20IoT%20applications), поэтому используйте Wi-Fi с активным диапазоном 2,4 ГГц.

```console
$ cp cfg.toml.example cfg.toml
$ $EDITOR cfg.toml
$ cat cfg.toml

[hardware-check]
wifi_ssid = "Your Wifi name"
wifi_psk = "Your Wifi password"
```


✅ Соберите, прошейте и откройте монитор проекта:

```console
$ cargo run

Serial port: /dev/SERIAL_DEVICE
Connecting...

Chip type:         ESP32-C3 (revision 3)
(...)
Compiling hardware-check v0.1.0
Finished release [optimized] target(s) in 1.78s

[00:00:45] ########################################     418/418     segment 0x10000

Flashing has completed!
(...)
rst:0x1 (POWERON),boot:0xc (SPI_FAST_FLASH_BOOT)
(...)
(...)
(...)
I (4427) wifi::wifi: Wifi connected!
```

>🔎 Если `cargo run` выполнился успешно, можно выйти сочетанием `ctrl+C`.

> 🔎 `cargo run` [настроен на использование `espflash`](https://github.com/r007/EmbeddedRust-Rus/blob/main/std-training/intro/hardware-check/.cargo/config.toml#L6) в качестве [пользовательского раннера](https://doc.rust-lang.org/cargo/reference/config.html#target). Тот же результат можно получить так:
> - с помощью `cargo-espflash`: `cargo espflash flash --release --monitor`;
> - собрав проект и прошив его через `espflash`: `cargo build --release && espflash target/riscv32imc-esp-espidf/release/hardware-check`.
> Для удобства эта настройка применена ко всем проектам курса.

При запуске светодиод платы должен загореться жёлтым. Затем, в зависимости от того, удалось ли подключиться к Wi-Fi, он либо загорится красным (ошибка), либо при успехе будет мигать, попеременно зелёным и синим. При ошибке Wi-Fi внизу также появится диагностическое сообщение, например:

```console
Error: could not connect to Wi-Fi network: ESP_ERR_TIMEOUT
```
> ⚠️ Ошибка `ESP_ERR_TIMEOUT` также возникнет, если имя сети или пароль указаны неверно, поэтому перепроверьте их.

## Дополнительная информация о сборке, прошивке и мониторинге

Если хотите попробовать собрать проект без прошивки, выполните:

 ```console
 cargo build
 ```
Также можно открыть монитор устройства без прошивки командой:

```console
espflash monitor
```

## Устранение неполадок

### Ошибки сборки

```console
error[E0463]: can't find crate for `core`
= note: the `riscv32imc-esp-espidf` target may not be installed
```

Вы пытаетесь собрать проект на `stable` Rust, а нужен `nightly`.
Это сообщение об ошибке немного вводит в заблуждение: этот таргет *нельзя* установить. Его нужно собрать из исходников с помощью `build-std`, а эта функция доступна только в nightly.

---

```console
error: cannot find macro `llvm_asm` in this scope
```

Вы используете несовместимую версию nightly. Настройте подходящую через `rust-toolchain.toml` или `cargo override`.

---

```console
CMake Error at .../Modules/CMakeDetermineSystem.cmake:129 (message):
```

Установка инструментария Espressif могла повредиться. Удалите её и запустите сборку заново, чтобы инициировать новую загрузку:

```console
rm -rf ~/.espressif
```
В Windows удалите папку `%USERPROFILE%\.espressif`.

---

 ```console
Serial port: /dev/tty.usbserial-110
Connecting...

Unable to connect, retrying with extra delay...
Unable to connect, retrying with default delay...
Unable to connect, retrying with extra delay...
Error: espflash::connection_failed

× Error while connecting to device
╰─▶ Failed to connect to the device
help: Ensure that the device is connected and the reset and boot pins are not being held down
```

Плата недоступна через USB-C кабель. Типичная ошибка подключения выглядит так:


Обходные пути:
1. Нажмите и удерживайте кнопку boot на плате, запустите команду прошивки и отпустите кнопку boot после начала прошивки.
2. Используйте USB-хаб.

[Источник](https://georgik.rocks/unable-to-flash-esp32-with-these-usb-c-cables/).
