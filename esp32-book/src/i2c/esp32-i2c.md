{{#title Как использовать I2C на ESP32 во встраиваемом Rust}}

# Шины I2C ESP32

ESP32 содержит два интерфейса шины I2C, каждый из которых можно настроить как контроллер или как целевое устройство. По даташиту ESP32 один интерфейс относится к основной системе, а другой — к низкопотребляющей подсистеме.

Оба интерфейса поддерживают стандартный режим (100 Кбит/с) и быстрый режим (400 Кбит/с), а также могут работать на скоростях до 5 МГц, хотя реальная производительность зависит от мощности подтягивающих резисторов на линиях SDA и SCL. ESP32 поддерживает 7-битную и 10-битную адресацию, включая режим двух адресов (dual address mode).

Благодаря гибкой матрице GPIO сигналы SDA и SCL можно назначить почти на любые выводы GPIO.

## Использование I2C в esp-hal

Чтобы использовать I2C с крейтом esp-hal, мы настраиваем интерфейс и назначаем выводы вручную. Вот пример настройки I2C с GPIO18 как SCL и GPIO23 как SDA:

```rust
 let i2c_bus = esp_hal::i2c::master::I2c::new(
    peripherals.I2C0,
    esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
)
.unwrap()
.with_scl(peripherals.GPIO18)
.with_sda(peripherals.GPIO23);
```

Когда шина готова, её можно передать драйверу, например драйверу для OLED-дисплея:

```rust
let interface = I2CDisplayInterface::new(i2c_bus);
// инициализируем дисплей
let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
    .into_buffered_graphics_mode();
```
