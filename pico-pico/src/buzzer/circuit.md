
# Подключение пищалки к Raspberry Pi Pico

Подключим GPIO 15 к положительному (сигнальному) выводу пищалки, а GND Pico — к земляному выводу пищалки. При необходимости можно использовать другой вывод GPIO.

<table style="margin-bottom:20px">
  <thead>
    <tr>
      <th>Вывод Pico</th>
      <th style="width: 250px; margin: 0 auto;">Провод</th>
      <th>Вывод пищалки</th>
      <th>Примечания</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>GPIO 15</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire red" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>Положительный вывод</td>
      <td>Получает сигналы ШИМ для создания звука.</td>
    </tr>
    <tr>
      <td>GND</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire black" style="width: 200px; margin: 0 auto;">
          <div class="male-left"></div>
          <div class="male-right"></div>
        </div>
      </td>
      <td>Земляной вывод</td>
      <td>Подключается к земле.</td>
    </tr>
  </tbody>
</table>

<img style="display: block; margin: auto;" alt="Схема подключения пищалки к Pico" src="./images/pico-buzzer-circuit.png"/>
