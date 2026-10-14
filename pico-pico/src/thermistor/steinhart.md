
# Уравнение Стейнхарта — Харта

Уравнение Стейнхарта — Харта — более точный метод перевода сопротивления термистора в температуру по сравнению с простым уравнением B. Оно использует три коэффициента, чтобы моделировать нелинейное поведение термистора в более широком диапазоне температур с большей точностью.

> [!Tip]
> В нашей программе мы не будем использовать эту формулу. Для простоты будем использовать уравнение B.

## Формула Стейнхарта — Харта

\\[
\frac{1}{T} = A + B \ln R + C (\ln R)^3
\\]

В этом уравнении T — искомая температура в кельвинах. R — измеренное сопротивление в омах. A, B и C — коэффициенты Стейнхарта — Харта, которые индивидуальны для каждого термистора. ln — натуральный логарифм.

### Калибровка

Чтобы определить точные значения A, B и C, поместите термистор в три известных температурных условия, обычно в ледяную воду (холод), при комнатной температуре и в горячую или кипящую воду.

Для каждого условия измерьте сопротивление термистора, используя значение АЦП, и запишите реальную температуру с помощью надёжного термометра. Перед дальнейшими расчётами переведите все температуры в кельвины.

Используя эти три пары «сопротивление — температура» вместе, коэффициенты A, B и C вычисляются путём решения системы уравнений. Коэффициенты не привязаны по отдельности к холодной, комнатной или горячей точке. Вместо этого все три калибровочные точки вместе задают кривую, которая описывает нелинейное поведение термистора в измеренном диапазоне.

Получив коэффициенты, те же значения A, B и C можно использовать для вычисления температуры при любом значении сопротивления в этом диапазоне.

### Вычисление коэффициентов Стейнхарта — Харта

Имея три пары данных «сопротивление — температура», можно вычислить коэффициенты Стейнхарта — Харта A, B и C.

$$
\begin{bmatrix}
    1 & \ln R_1 & \ln^3 R_1 \\\\
    1 & \ln R_2 & \ln^3 R_2 \\\\
    1 & \ln R_3 & \ln^3 R_3
\end{bmatrix}\begin{bmatrix}
    A \\\\
    B \\\\
    C
\end{bmatrix} = \begin{bmatrix}
    \frac{1}{T_1} \\\\
    \frac{1}{T_2} \\\\
    \frac{1}{T_3}
\end{bmatrix}
$$

Где:

- \\( R_1, R_2, R_3 \\) — значения сопротивления при температурах \\( T_1, T_2, T_3 \\).

**Посчитаем коэффициенты**

Вычисляем натуральные логарифмы сопротивлений:
$$
L_1 = \ln R_1, \quad L_2 = \ln R_2, \quad L_3 = \ln R_3
$$

Промежуточные вычисления:
$$
Y_1 = \frac{1}{T_1}, \quad Y_2 = \frac{1}{T_2}, \quad Y_3 = \frac{1}{T_3}
$$

$$
\gamma_2 = \frac{Y_2 - Y_1}{L_2 - L_1}, \quad \gamma_3 = \frac{Y_3 - Y_1}{L_3 - L_1}
$$

И, наконец:
$$
C = \left( \frac{ \gamma_3 - \gamma_2 }{ L_3 - L_2} \right) \left(L_1 + L_2 + L_3\right)^{-1} \\
$$
$$
B = \gamma_2 - C \left(L_1^2 + L_1 L_2 + L_2^2\right) \\
$$
$$
A = Y_1 - \left(B + L_1^2 C\right) L_1
$$

<span style="color: green;">Хорошая новость, всем!</span> Коэффициенты вычислять вручную не нужно. Достаточно указать значения сопротивления и температуры для холодной, комнатной и горячей среды, а затем с помощью формы ниже определить A, B и C.

### Расчёт значения АЦП и сопротивления

<span style="color: orange;">Примечание:</span> если у вас уже есть температура и соответствующее сопротивление, можно сразу ввести эти значения во вторую таблицу.

Если у вас есть значение АЦП и вы хотите вычислить сопротивление, используйте эту таблицу, чтобы найти соответствующее сопротивление при разных температурах. Когда вы вводите значение АЦП для каждой температуры, рассчитанное сопротивление автоматически обновляется во второй таблице.

Для этого расчёта понадобится базовое сопротивление термистора, которое необходимо для определения сопротивления при заданной температуре по значению АЦП.

Учтите, что количество бит АЦП может потребоваться изменить, если вы используете другой микроконтроллер. В нашем случае, для Raspberry Pi Pico, разрешение АЦП составляет 12 бит.

<form id="adcForm">
  <label for="baseResistance">Базовое сопротивление (Ω): </label>
  <input type="number" id="baseResistance" name="baseResistance" step="any" value="10000" oninput="updateResistance()">
    <br/>
    <label for="adcBits">Разрядность АЦП: </label>
  <input type="number" id="adcBits" name="adcBits" step="any" value="12" oninput="updateResistance()">
  <br/>
  <br/>

  <table>
    <thead>
      <tr>
         <th>Среда</th>
        <th>Значение АЦП</th>
      </tr>
    </thead>
    <tbody>
      <!-- Строка для холодной воды -->
      <tr>
        <td>Холодная вода</td>
        <td><input type="number" id="adcColdCount" name="adcColdCount" step="any" oninput="updateResistance()"></td>
      </tr>
      <!-- Строка для комнатной температуры -->
      <tr>
        <td>Комнатная температура</td>
        <td><input type="number" id="adcRoomCount" name="adcRoomCount" step="any" oninput="updateResistance()"></td>
      </tr>
      <!-- Строка для кипящей воды -->
      <tr>
      <td>Кипящая вода</td>
        <td><input type="number" id="adcBoilCount" name="adcBoilCount" step="any" oninput="updateResistance()"></td>
      </tr>
    </tbody>
  </table>
</form>

### Поиск коэффициентов

Измените температуру, введя значение в градусах Фаренгейта или Цельсия: форма автоматически переведёт его в другой формат. Укажите сопротивление для каждой температуры, а затем нажмите кнопку «Рассчитать коэффициенты».

<form id="steinhartForm" onsubmit="calcCoeffBtnClicked(event)">
<table>
<thead>
<tr>
<th>Среда</th>
<th>Сопротивление (Ом)</th>
<th>Температура (°F)</th>
<th>Температура (°C)</th>
<th>Температура (K)</th>
</tr>
</thead>
<tbody>
<tr>
<td>Холодная вода</td>
<td><input type="number" id="resistanceCold" name="resistanceCold" step="any" oninput="validateInput()"></td>
<td><input type="number" id="coldTempF" name="coldTempF" step="any" oninput="calcTempFromFarenhit('coldTempC', 'coldTempF', 'coldTempK', 'resistanceCold')"></td>
<td><input type="number" id="coldTempC" name="coldTempC" step="any" oninput="calcTempFromCel('coldTempC', 'coldTempF', 'coldTempK', 'resistanceCold')"></td>

<td><input type="number" id="coldTempK" name="coldTempK" step="any" readonly></td>
</tr>
<tr>
<td>Комнатная температура</td>
<td><input type="number" id="resistanceRoom" name="resistanceRoom" step="any" oninput="validateInput()"></td>
<td><input type="number" id="roomTempF" name="roomTempF" step="any" oninput="calcTempFromFarenhit('roomTempC', 'roomTempF', 'roomTempK', 'resistanceRoom')"></td>
<td><input type="number" id="roomTempC" name="roomTempC" step="any" value="25"  oninput="calcTempFromCel('roomTempC', 'roomTempF', 'roomTempK', 'resistanceRoom')"></td>
<td><input type="number" id="roomTempK" name="roomTempK" step="any" readonly></td>
</tr>
<tr>
<td>Кипящая вода</td>
<td><input type="number" id="resistanceBoiling" name="resistanceBoiling"  step="any" oninput="validateInput()"></td>

<td><input type="number" id="boilTempF" name="boilTempF" step="any"  oninput="calcTempFromFarenhit('boilTempC', 'boilTempF', 'boilTempK', 'resistanceBoiling')"></td>
<td><input type="number" id="boilTempC" name="boilTempC" step="any" oninput="calcTempFromCel('boilTempC', 'boilTempF', 'boilTempK', 'resistanceBoiling')"></td>
<td><input type="number" id="boilTempK" name="boilTempK" step="any" readonly></td>
</tr>
</tbody>
</table>

<h3>Результаты</h3>
<p>
    A: <input type="text" id="resultA" readonly />
    <span id="actualA"></span>
</p>
<p>
    B: <input type="text" id="resultB" readonly />
    <span id="actualB"></span>
</p>
<p>
    C: <input type="text" id="resultC" readonly />
    <span id="actualC"></span>
</p>

<button type="submit" id="submitBtn" >Рассчитать коэффициенты</button>
</form>

<h3>Расчёт температуры по сопротивлению</h3>
<p>Теперь с этими коэффициентами можно вычислить температуру для любого сопротивления:</p>

<label for="r">R (Ом): </label>
<input type="number" name="r" value="10000" id="inputResistance" >

<button type="button" id="calculateBtn" onclick="calculateTemperatureFromResistance()" >Рассчитать температуру</button>

<label for="tc">Результат (°C): </label>
<input type="text" name="tc" id="resultCelsius" readonly>

<label for="tf">Результат (°F): </label>
<input type="text" name="tf" id="resultFahrenheit" readonly>

<!-- Блок сообщения об ошибке -->
<p id="errorMessage" style="color: red; display: none;">Ошибка: сначала рассчитайте коэффициенты (A, B, C).</p>

<script>
window.onload = function() {
  // Значения по умолчанию для холодной воды
  document.getElementById("resistanceCold").value = 25000;
  document.getElementById("coldTempC").value = 5;
  calcTempFromCel('coldTempC', 'coldTempF', 'coldTempK', 'resistanceCold');

  // Значения по умолчанию для комнатной температуры
  document.getElementById("resistanceRoom").value = 10000;
  document.getElementById("roomTempC").value = 25;
  calcTempFromCel('roomTempC', 'roomTempF', 'roomTempK', 'resistanceRoom');

  // Значения по умолчанию для кипящей воды
  document.getElementById("resistanceBoiling").value = 4000;
  document.getElementById("boilTempC").value = 45;
  calcTempFromCel('boilTempC', 'boilTempF', 'boilTempK', 'resistanceBoiling');

  calculateCoefficients();
};

// Функция для вычисления сопротивления по базовому сопротивлению и значению АЦП
function calculateResistance(baseResistance, adcCount, adcBits) {
  const maxADCValue = Math.pow(2, adcBits) - 1;  // Максимальное значение АЦП для заданной разрядности (например, 12 бит = 4095)

  const resistance = baseResistance * ((maxADCValue / adcCount)-1);

  return resistance;
}

function updateResistance() {
  const baseResistance = parseFloat(document.getElementById("baseResistance").value);
  const adcBits = parseInt(document.getElementById("adcBits").value);

  const adcColdCount = parseFloat(document.getElementById("adcColdCount").value);
  const adcRoomCount = parseFloat(document.getElementById("adcRoomCount").value);
  const adcBoilCount = parseFloat(document.getElementById("adcBoilCount").value);

  // Вычисляем сопротивление для каждой среды по значениям АЦП
  if (!isNaN(baseResistance) && !isNaN(adcBits)) {
    const resistanceCold = calculateResistance(baseResistance, adcColdCount, adcBits);
    document.getElementById("resistanceCold").value = resistanceCold.toFixed(2);

    const resistanceRoom = calculateResistance(baseResistance, adcRoomCount, adcBits);
    document.getElementById("resistanceRoom").value = resistanceRoom.toFixed(2);

    const resistanceBoiling = calculateResistance(baseResistance, adcBoilCount, adcBits);
    document.getElementById("resistanceBoiling").value = resistanceBoiling.toFixed(2);
  }
}

function calcTempFromCel(celsiusId, fahrenheitId, kelvinId, resistanceId) {
    const tempC = parseFloat(document.getElementById(celsiusId).value);

    if (!isNaN(tempC)) {
        const tempF = (tempC * 9/5) + 32;
        const tempK = tempC + 273.15;
        document.getElementById(fahrenheitId).value = tempF.toFixed(2);
        document.getElementById(kelvinId).value = tempK.toFixed(2);
    } else{
        document.getElementById(fahrenheitId).value = "";
        document.getElementById(kelvinId).value = "";
    }
}

function calcTempFromFarenhit(celsiusId, fahrenheitId, kelvinId, resistanceId) {
    const tempF = parseFloat(document.getElementById(fahrenheitId).value);
    if (!isNaN(tempF)) {
        const tempC = (tempF - 32) * 5 / 9;
        const tempK = tempC + 273.15;
        document.getElementById(celsiusId).value = tempC.toFixed(2);
        document.getElementById(kelvinId).value = tempK.toFixed(2);
    } else{
        document.getElementById(celsiusId).value = "";
        document.getElementById(kelvinId).value = "";
    }
}

function validateInput() {
    const resistanceCold = document.getElementById("resistanceCold").value;
    const resistanceRoom = document.getElementById("resistanceRoom").value;
    const resistanceBoiling = document.getElementById("resistanceBoiling").value;
    const coldTempC = document.getElementById("coldTempC").value;
    const roomTempC = document.getElementById("roomTempC").value;
    const boilTempC = document.getElementById("boilTempC").value;
    const submitBtn = document.getElementById("submitBtn");
}

function calcCoeffBtnClicked(event){
    event.preventDefault();
    calculateCoefficients();
}

function calculateCoefficients() {
    // const coldTempC = parseFloat(document.getElementById("coldTempC").value);
    // const roomTempC = parseFloat(document.getElementById("roomTempC").value);
    // const boilTempC = parseFloat(document.getElementById("boilTempC").value);

    // const coldTempK = coldTempC + 273.15;
    // const roomTempK = roomTempC + 273.15;
    // const boilTempK = boilTempC + 273.15;
    const T1 = parseFloat(document.getElementById("coldTempK").value);
    const T2 = parseFloat(document.getElementById("roomTempK").value);
    const T3 = parseFloat(document.getElementById("boilTempK").value);

    const resistanceCold = parseFloat(document.getElementById("resistanceCold").value);
    const resistanceRoom = parseFloat(document.getElementById("resistanceRoom").value);
    const resistanceBoiling = parseFloat(document.getElementById("resistanceBoiling").value);

    const L1 = Math.log(resistanceCold); //натуральный логарифм
    const L2 = Math.log(resistanceRoom);
    const L3 = Math.log(resistanceBoiling);

    const Y1 = 1 / T1;
    const Y2 = 1 / T2;
    const Y3 = 1 / T3;

    const gamma2 = (Y2 - Y1) / (L2 - L1); //γ2
    const gamma3 = (Y3 - Y1) / (L3 - L1); //γ3

    // Вычисляем коэффициенты A, B и C
    const C = ((gamma3 - gamma2) / (L3 - L2)) * (L1 + L2 + L3) ** -1;
    const B = gamma2 - C * (Math.pow(L1, 2) + L1 * L2 + Math.pow(L2, 2));
    const A = Y1 - (B + Math.pow(L1, 2) * C) * L1;

    document.getElementById("resultA").value = A.toExponential(8);
    document.getElementById("resultB").value = B.toExponential(8);
    document.getElementById("resultC").value = C.toExponential(8);

    document.getElementById("actualA").textContent = `(${A.toFixed(16)})`;
    document.getElementById("actualB").textContent = `(${B.toFixed(16)})`;
    document.getElementById("actualC").textContent = `(${C.toFixed(16)})`;
}

function calculateTemperatureFromResistance() {

    const A = parseFloat(document.getElementById('resultA').value);
    const B = parseFloat(document.getElementById('resultB').value);
    const C = parseFloat(document.getElementById('resultC').value);

    if (isNaN(A) || isNaN(B) || isNaN(C)) {
        document.getElementById('errorMessage').style.display = 'block';
        document.getElementById('resultFahrenheit').value = '';
        document.getElementById('resultCelsius').value = '';
        return;
    } else{
            document.getElementById('errorMessage').style.display = 'none';
    }

    let resistance = parseFloat(document.getElementById('inputResistance').value);
    if (isNaN(resistance)) {
        alert("Введите корректное сопротивление.");
        return;
    }

    // Вычисляем температуру в кельвинах по уравнению Стейнхарта — Харта:
    // 1/T = A + B*ln(R) + C*(ln(R))^3
    let inverseTemperature = A + B * Math.log(resistance) + C * Math.pow(Math.log(resistance), 3);
    let temperatureKelvin = 1 / inverseTemperature;

    // Переводим в градусы Цельсия и Фаренгейта
    let temperatureCelsius = temperatureKelvin - 273.15;
    let temperatureFahrenheit = (temperatureCelsius * 9/5) + 32;

    document.getElementById('resultFahrenheit').value = temperatureFahrenheit.toFixed(2);
    document.getElementById('resultCelsius').value = temperatureCelsius.toFixed(2);

}
</script>

### Функция на Rust

```rust
fn steinhart_temp_calc(
    resistance: f64, // Сопротивление в омах
    a: f64,          // Коэффициент A
    b: f64,          // Коэффициент B
    c: f64,          // Коэффициент C
) -> Result<(f64, f64), String> {
    if resistance <= 0.0 {
        return Err("Resistance must be a positive number.".to_string());
    }

    // Вычисляем температуру в кельвинах по уравнению Стейнхарта — Харта:
    // 1/T = A + B*ln(R) + C*(ln(R))^3
    let ln_r = resistance.ln();
    let inverse_temperature = a + b * ln_r + c * ln_r.powi(3);

    if inverse_temperature == 0.0 {
        return Err("Invalid coefficients or resistance leading to division by zero.".to_string());
    }

    let temperature_kelvin = 1.0 / inverse_temperature;

    let temperature_celsius = temperature_kelvin - 273.15;
    let temperature_fahrenheit = (temperature_celsius * 9.0 / 5.0) + 32.0;

    Ok((temperature_celsius, temperature_fahrenheit))
}

fn main() {
    // Примеры входных данных
    let a = 2.10850817e-3;
    let b = 7.97920473e-5;
    let c = 6.53507631e-7;
    let resistance = 10000.0;

    match steinhart_temp_calc(resistance, a, b, c) {
        Ok((celsius, fahrenheit)) => {
            println!("Temperature in Celsius: {:.2}", celsius);
            println!("Temperature in Fahrenheit: {:.2}", fahrenheit);
        }
        Err(e) => println!("Error: {}", e),
    }
}
```

## Ссылки

- [Калькулятор термисторов](https://www.thinksrs.com/downloads/programs/therm%20calc/ntccalibrator/ntccalculator.html) (на английском языке)
- [Коэффициенты Стейнхарта — Харта для расчёта температуры двигателя](https://www.servo.jp/member/admin/document_upload/AN144-Thermistor-Steinhart-Hart-Coefficients.pdf) (на английском языке)
- [Калибровка коэффициентов Стейнхарта — Харта для термисторов](https://www.thinksrs.com/downloads/PDFs/ApplicationNotes/LDC%20Note%204%20NTC%20Calculatorold.pdf) (на английском языке)
- [Кулинарный термометр с коррекцией Стейнхарта — Харта](https://www.instructables.com/ESP32-NTP-Temperature-Probe-Cooking-Thermometer-Wi/) (на английском языке)
