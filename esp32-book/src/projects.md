{{#title Подборка встраиваемых проектов на Rust для ESP32}}

# Подборка проектов на Rust для ESP32

Здесь собраны интересные проекты, которые я нашёл в интернете и которые связаны с ESP32 и Rust. Если у вас есть интересный проект, которым хотите поделиться, присылайте PR :)

Примечание: подборка включает проекты для всех семейств ESP32, поэтому не все они могут работать на том же SoC, который используется в книге.

## ESP32
- [ESP32 Rex](https://github.com/implferris/esp32-rex): игра про динозаврика для ESP32 с OLED-дисплеем на фреймворке Embassy.
- [ESP32 RFID Access Control](https://github.com/implferris/esp32-rfid-access): имитация умного дверного замка на Rust и ESP32: RFID, необязательный сервопривод и OLED-дисплей для имитации и управления доступом через дверь.
- [ESP32 Wi-Fi Tank](https://github.com/jamesmcm/esp32_wifi_tank): танк или ровер с управлением по Wi-Fi, построенный на плате управления ESP32 и Rust.
- [Solar Inverter](https://github.com/Orange-Murker/solar_inverter): солнечный инвертор, подключаемый к сети, с MPPT.
- [Paper train](https://github.com/vhdirk/papertrain): выводит задержки поездов NMBS на e-ink-дисплей, управляемый ESP32.
- [Snake Game](https://jamesmcm.github.io/blog/beginner-rust-esp32-lcdsnake/): игра «Змейка» для ESP32 с подключённым OLED-дисплеем, управляемая джойстиком.

## ESP32-C3
- [ha-vfd-dashboard](https://github.com/Orange-Murker/ha-vfd-dashboard/): панель Home Assistant на вакуумном люминесцентном дисплее.
- [ESP32-C3 Embassy](https://github.com/claudiomattera/esp32c3-embassy): собирает данные об окружающей среде (температура, влажность, давление) с датчика BME280 по I²C и выводит последнее значение на E-INK-дисплей WaveShare 1.54 дюйма (модель B, версия 2) по SPI.

## ESP32-S3
- [anemometer](https://github.com/taunusflieger/anemometer): анемометр на базе ESP32-S3 от Espressif, напечатанный на 3D-принтере.

## Общее
- [esp32-spooky-maze-game](https://github.com/georgik/esp32-spooky-maze-game): реализация игры-лабиринта для ESP32 на Rust в режиме bare metal.
