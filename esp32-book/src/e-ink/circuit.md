{{#title Подключение дисплея Waveshare e-Ink к ESP32 по SPI}}

# Схема подключения

## Распиновка e-Paper дисплея

<img style="display: block; margin: auto;" src="./images/e-paper-1.54 inch-display.png" alt="распиновка e-ink"/>

- **VCC**: питание (3,3 В или 5 В) для дисплея.  
- **GND**: земля  
- **DIN (MOSI)**: линия данных SPI для отправки информации от микроконтроллера на дисплей.  
- **CLK (SCK)**: синхронизирует передачу данных по SPI.  
- **CS (Chip Select, активный низкий уровень)**: включает или отключает обмен по SPI. При низком уровне дисплей активен.  
- **DC (Data/Command Select)**: определяет, что передаётся: данные пикселей (высокий уровень) или команда (низкий уровень).  
- **RST (Reset, активный низкий уровень)**: сбрасывает дисплей при подаче низкого уровня, а затем высокого. Необходим для инициализации.  
- **BUSY**: показывает, занят ли дисплей обработкой (высокий уровень) или готов принимать новые команды (низкий уровень).  


## Подключение e-Paper дисплея к ESP32

<img style="display: block; margin: auto;" src="./images/esp32-circuit-e-ink.png" alt="распиновка e-ink"/>
<br/>
<table style="margin-bottom:20px">
  <thead>
    <tr>
      <th>Вывод e-Paper</th>
      <th style="width: 250px; margin: 0 auto;">Провод</th>
      <th>Вывод ESP32</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>VCC</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire red" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>3.3V</td>
    </tr>
    <tr>
      <td>GND</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire black" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>GND</td>
    </tr>
    <tr>
      <td>DIN (MOSI)</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire blue" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>GPIO23</td>
    </tr>
    <tr>
      <td>CLK (SCK)</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire yellow" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>GPIO18</td>
    </tr>
    <tr>
      <td>CS</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire orange" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>GPIO33</td>
    </tr>
    <tr>
      <td>DC</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire green" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>GPIO17 (TX2 на DevKit)</td>
    </tr>
    <tr>
      <td>RST</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire white" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>GPIO16 (Rx2 на DevKit)</td>
    </tr>
    <tr>
      <td>BUSY</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire purple" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>GPIO22</td>
    </tr>
  </tbody>
</table>

