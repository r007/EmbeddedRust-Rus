{{#title Распиновка LCD1602 с I2C для ESP32: назначение выводов}}

# Раскладка выводов

У I2C-варианта ЖК-модуля всего 4 вывода.

<img style="display: block; margin: auto;" alt="lcd1602" src="./images/lcd-i2c-pins.jpg"/>

<br/>

<table border="1" style="border-collapse: collapse; width: 100%; text-align: center;">
  <thead>
    <tr>
      <th>Вывод</th>
      <th>Обозначение</th>
      <th>Описание</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>1</td>
      <td>VCC</td>
      <td>Питание (обычно 5 В)</td>
    </tr>
    <tr>
      <td>2</td>
      <td>GND</td>
      <td>Земля</td>
    </tr>
    <tr>
      <td>3</td>
      <td>SDA</td>
      <td>Линия данных для связи I2C</td>
    </tr>
    <tr>
      <td>4</td>
      <td>SCL</td>
      <td>Линия тактирования для связи I2C</td>
    </tr>
  </tbody>
</table>
