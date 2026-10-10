# Распиновка

У джойстика всего 5 выводов: питание, земля, выход оси X, выход оси Y и выход переключателя.

<img style="display: block; margin: auto;width:400px;margin-bottom: 10px;" alt="Распиновка джойстика" src="./images/joystick-pin-layout.jpg"/>

<table border="1" style="border-collapse: collapse; width: 100%;">
  <thead>
    <tr>
      <th style="width:14%">Вывод джойстика</th>
      <th>Описание</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><span class="slanted-text black">GND</span></td>
      <td>Вывод земли. Должен быть подключён к земле схемы.</td>
    </tr>
    <tr>
      <td><span class="slanted-text red">VCC</span></td>
      <td>Вывод питания (обычно 5 В или 3,3 В).</td>
    </tr>
    <tr>
      <td><span class="slanted-text green">VRX</span></td>
      <td>Аналоговый выход оси X: его напряжение меняется в зависимости от горизонтального положения джойстика и изменяется от 0 В до VCC при движении влево и вправо.</td>
    </tr>
    <tr>
      <td><span class="slanted-text blue">VRY</span></td>
      <td>Аналоговый выход оси Y: его напряжение меняется в зависимости от вертикального положения джойстика и изменяется от 0 В до VCC при движении вверх и вниз.</td>
    </tr>
    <tr>
      <td><span class="slanted-text purple">SW</span></td>
      <td>Вывод переключателя. Когда ручку джойстика нажимают, этот вывод обычно притягивается к низкому уровню (к GND).</td>
    </tr>
  </tbody>
</table>
