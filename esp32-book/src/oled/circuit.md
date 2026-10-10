{{#title Подключение OLED-дисплея SSD1306 к ESP32}}

# Схема подключения

Для обмена данными между ESP32 и OLED-дисплеем мы будем использовать I2C. Для I2C нужно настроить два вывода GPIO: SDA (Serial Data, данные) и SCL (Serial Clock, тактирование). На ESP32 для I2C можно использовать любые выводы GPIO. Мы назначим GPIO 18 как SCL, а GPIO 23 как SDA. Вывод VCC OLED подключим к выводу 3,3 В ESP32, а вывод земли (GND) — к земле ESP32.

<table style="margin-bottom:20px">
  <thead>
    <tr>
      <th>Вывод ESP32</th>
      <th style="width: 250px; margin: 0 auto;">Провод</th>
      <th>Вывод OLED</th>
      <th>Примечания</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>3.3V</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire red" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>VCC</td>
      <td>Питает OLED-дисплей.</td>
    </tr>
    <tr>
      <td>GND</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire black" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>Земля</td>
      <td>Подключается к выводу земли OLED.</td>
    </tr>
    <tr>
      <td>GPIO 18</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire blue" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>SCL</td>
      <td>Подключает сигнал тактирования (SCL) для связи I2C.</td>
    </tr>
    <tr>
      <td>GPIO 23</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire yellow" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>SDA </td>
      <td>Подключает сигнал данных (SDA) для связи I2C.</td>
    </tr>
  </tbody>
</table>

<img style="display: block; margin: auto;" title="схема ESP32 с OLED SSD1306" src="./images/connecting esp32 with oled ssd1306 circuit.png"/>
