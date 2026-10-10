{{#title Подключение джойстика к ESP32 через выводы АЦП}}

# Подключение джойстика к ESP32

Подключим джойстик к ESP32. Нужно соединить выводы VRX и VRY с выводами АЦП ESP32. Джойстик будем питать от 3,3 В вместо 5 В, потому что выводы GPIO ESP32 допускают только 3,3 В. Подключение к 5 В может повредить выводы ESP32. К счастью, джойстик может работать и от 3,3 В.

<table>
  <thead>
    <tr>
      <th>Вывод ESP32</th>
      <th style="width: 250px; margin: 0 auto;">Провод</th>
      <th>Вывод джойстика</th>
    </tr>
  </thead>
  <tbody>
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
      <td>3.3V</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire red" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>VCC</td>
    </tr>
    <tr>
      <td>GPIO 13</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire green" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>VRX</td>
    </tr>
    <tr>
      <td>GPIO 14</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire blue" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>VRY</td>
    </tr>
    <tr>
      <td>GPIO 32</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire yellow" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>SW</td>
    </tr>
  </tbody>
</table>

<img style="display: block; margin: auto;margin-top:30px;" alt="подключение джойстика к esp32" src="./images/connecting joystick with esp32.png"/>
