{{#title Настройка среды для embedded-разработки на Rust для Raspberry Pi Pico 2 | impl Rust для RP2350}}

# Настройка

## Picotool

picotool — инструмент для работы с бинарными файлами RP2040/RP2350 и для взаимодействия с устройствами RP2040/RP2350, когда они находятся в режиме BOOTSEL.

[Репозиторий Picotool](https://github.com/raspberrypi/picotool)

> [!TIP]
> Можно также скачать готовые бинарные файлы инструментов SDK <a href="https://github.com/raspberrypi/pico-sdk-tools">отсюда</a> — это проще, чем выполнять описанные ниже шаги.

Вот краткая сводка шагов, которые я выполнял:

```sh
# Установить зависимости
sudo apt install build-essential pkg-config libusb-1.0-0-dev cmake

mkdir embedded && cd embedded

# Клонировать Pico SDK
git clone https://github.com/raspberrypi/pico-sdk
cd pico-sdk
git submodule update --init lib/mbedtls
cd ../

# Задать переменную окружения для Pico SDK
PICO_SDK_PATH=/MY_PATH/embedded/pico-sdk

# Клонировать репозиторий Picotool
git clone https://github.com/raspberrypi/picotool
```

Сборка и установка Picotool

```sh
cd picotool
mkdir build && cd build
# cmake ../
cmake -DPICO_SDK_PATH=/MY_PATH/embedded/pico-sdk/ ../
make -j8
sudo make install
```

В Linux можно добавить правила udev, чтобы запускать picotool без sudo:

```sh
cd ../
# Внутри склонированной директории picotool
sudo cp udev/60-picotool.rules /etc/udev/rules.d/
```

## Цели компиляции Rust

Чтобы собирать и загружать Rust-код для чипа RP2350, нужно добавить соответствующие цели:

```sh
rustup target add thumbv8m.main-none-eabihf
rustup target add riscv32imac-unknown-none-elf
```

## probe-rs — инструмент для прошивки и отладки

probe-rs — современный инструментарий на Rust для прошивки и отладки embedded-устройств. Он поддерживает цели ARM и RISC-V и работает напрямую с аппаратными отладочными зондами. Если вы используете отладочный зонд с Pico 2, для прошивки и отладки вы будете опираться именно на probe-rs.

Установите probe-rs с помощью официального скрипта-установщика:

```bash
curl -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

Актуальные инструкции по установке лучше смотреть в [официальной документации probe-rs](https://probe.rs/).

По умолчанию отладочные зонды в Linux доступны только с правами root. Чтобы не вводить sudo для каждой команды, установите подходящие правила udev, которые позволят обычным пользователям работать с зондом. Следуйте [приведённым инструкциям](https://probe.rs/docs/getting-started/probe-setup/).

**Краткая сводка:**

1. [Скачайте файл правил udev](https://probe.rs/files/69-probe-rs.rules) из репозитория probe-rs
2. Скопируйте его в `/etc/udev/rules.d/`
3. Перезагрузите правила udev командой `sudo udevadm control --reload`
4. Отключите и снова подключите отладочный зонд

После этой настройки probe-rs можно использовать без прав root.
