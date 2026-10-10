{{#title Делитель напряжения: схема и расчёт для проектов на embedded Rust}}

# Делитель напряжения

Делитель напряжения — простая схема, которая понижает входное напряжение до меньшего выходного с помощью двух последовательно соединённых резисторов. Делитель может понадобиться, когда датчик или модуль выдаёт напряжение выше того, которое микроконтроллер безопасно выдерживает.

Резистор, подключённый к входному напряжению, называется \\( R_{1} \\), а резистор, подключённый к земле, — \\( R_{2} \\). Выходное напряжение \\( V_{out} \\) измеряется в точке между \\( R_{1} \\) и \\( R_{2} \\), и оно составляет долю от входного напряжения \\( V_{in} \\).

## Схема

<img style="display: block; margin: auto;" alt="Делитель напряжения" src="./images/voltage-divider.png"/>

Выходное напряжение (V<sub>out</sub>) рассчитывается по формуле:

\\[
V_{out} = V_{in} \times \frac{R_2}{R_1 + R_2}
\\]

### Пример расчёта \\( V_{out} \\)

Дано:
- \\( V_{in} = 3.3 V \\)
- \\( R_1 = 10 k\Omega \\)
- \\( R_2 = 10 k\Omega \\)

Подставляем значения:

\\[
V_{out} = 3.3 V \times \frac{10 k\Omega}{10 k\Omega + 10 k\Omega} = 3.3 V \times \frac{10}{20} = 3.3 V \times 0.5 = 1.65 V
\\]

Выходное напряжение \\( V_{out} \\) равно 1.65 В.

```rust,editable
fn main() {
    // Код можно редактировать
    // Можно менять значения и запускать код
    let vin: f64 = 3.3;
    let r1: f64 = 10000.0;
    let r2: f64 = 10000.0;

    let vout = vin * (r2 / (r1 + r2));

    println!("The output voltage Vout is: {:.2} V", vout);
}
```

## Применение

Делители напряжения используются, например, в потенциометрах: при повороте ручки сопротивление меняется, и вместе с ним меняется выходное напряжение. Их также применяют для измерения резистивных датчиков, таких как датчики света и термисторы. К ним подаётся известное напряжение, а микроконтроллер считывает напряжение в средней точке и по нему определяет значения датчика, например температуру.

## Симуляция делителя напряжения

<style>
canvas {
    border: 1px solid #ccc;
    margin-top: 20px;
    background:white;
}
</style>
<label for="vin">Входное напряжение (V<sub>in</sub>):</label>
<input type="number" id="vin" step="0.01" value="3.3" oninput="updateAndCalculate()"><br><br>

<label for="r1">Резистор R1 (Ω):</label>
<input type="number" id="r1" step="1" value="10000" oninput="updateAndCalculate()"><br><br>

<label for="r2">Резистор R2 (Ω):</label>
<input type="number" id="r2" step="1" value="10000" oninput="updateAndCalculate()"><br><br>

<p class="formula" id="formula">
    Формула: V<sub>out</sub> = V<sub>in</sub> × (R<sub>2</sub> / (R<sub>1</sub> + R<sub>2</sub>))
</p>
<p class="formula" id="filledFormula">
    Формула с подставленными значениями: V<sub>out</sub> = 3.3 × (10000 / (10000 + 10000))
</p>
<p id="result">Выходное напряжение (Vout): 1.65 V</p>

<canvas id="circuitCanvas" width="600" height="400"></canvas>

## Симулятор на сайте Falstad

Для построения схемы я использовал сайт [https://www.falstad.com/circuit/](https://www.falstad.com/circuit/). Это отличный инструмент для рисования схем. Вы можете скачать созданный мной файл [`voltage-divider.circuitjs.txt`](./voltage-divider.circuitjs.txt) и импортировать его, чтобы поэкспериментировать с этой схемой.

<script>
    function updateAndCalculate() {
        updateFormula();
        calculateVoltage();
        drawCircuit();
    }

    function updateFormula() {
        const vin = parseFloat(document.getElementById('vin').value) || 0;
        const r1 = parseFloat(document.getElementById('r1').value) || 0;
        const r2 = parseFloat(document.getElementById('r2').value) || 0;

        document.getElementById('filledFormula').textContent =
            `Формула с подставленными значениями: Vout = ${vin} × (${r2} / (${r1} + ${r2}))`;
    }

    function calculateVoltage() {
        const vin = parseFloat(document.getElementById('vin').value);
        const r1 = parseFloat(document.getElementById('r1').value);
        const r2 = parseFloat(document.getElementById('r2').value);

        if (isNaN(vin) || isNaN(r1) || isNaN(r2) || r1 <= 0 || r2 <= 0) {
            document.getElementById('result').textContent =
                "Введите корректные положительные числа во всех полях.";
            return;
        }

        const vout = vin * (r2 / (r1 + r2));
        document.getElementById('result').textContent =
            `Выходное напряжение (Vout): ${vout.toFixed(2)} V`;
    }

    function drawZigZagResistor(ctx, x, y, width, height) {
        const segments = 6;
        const step = height / segments;
        const segmentWidth = width / 2;

        ctx.beginPath();
        ctx.moveTo(x, y);

        for (let i = 0; i < segments; i++) {
            const offset = (i % 2 === 0) ? segmentWidth : -segmentWidth;
            ctx.lineTo(x + offset, y + step * (i + 1));
        }

        const finalOffset = (segments % 2 === 0) ? segmentWidth : -segmentWidth;
        ctx.lineTo(x + 1, y + height + 4);
        ctx.lineWidth = 2;
        ctx.stroke();
    }

    function drawGroundSymbol(ctx, x, y) {
        const lineSpacing = 5;
        const topLineWidth = 20;

        ctx.beginPath();
        ctx.moveTo(x - topLineWidth / 2, y);
        ctx.lineTo(x + topLineWidth / 2, y);
        ctx.stroke();

        const middleLineWidth = topLineWidth * 0.6;
        ctx.beginPath();
        ctx.moveTo(x - middleLineWidth / 2, y + lineSpacing);
        ctx.lineTo(x + middleLineWidth / 2, y + lineSpacing);
        ctx.stroke();

        const bottomLineWidth = middleLineWidth * 0.6;
        ctx.beginPath();
        ctx.moveTo(x - bottomLineWidth / 2, y + 2 * lineSpacing);
        ctx.lineTo(x + bottomLineWidth / 2, y + 2 * lineSpacing);
        ctx.stroke();
    }

    function drawCircuit() {
        const canvas = document.getElementById('circuitCanvas');
        const ctx = canvas.getContext('2d');
        const vin = parseFloat(document.getElementById('vin').value) || 0;
        const r1 = parseFloat(document.getElementById('r1').value) || 0;
        const r2 = parseFloat(document.getElementById('r2').value) || 0;
        const vout = vin * (r2 / (r1 + r2)) || 0;

        ctx.clearRect(0, 0, canvas.width, canvas.height);

        ctx.fillStyle = 'black';
        ctx.font = '16px Arial';
        ctx.fillText(`V_in: ${vin.toFixed(2)} V`, 35, 94);
        ctx.beginPath();
        ctx.moveTo(100, 100);
        ctx.lineTo(150, 100);
        ctx.stroke();

        ctx.moveTo(130, 100);
        ctx.lineTo(170, 100);
        ctx.stroke();

        ctx.beginPath();
        ctx.moveTo(170, 100);
        ctx.lineTo(170, 130);
        ctx.stroke();

        ctx.fillText(`R1: ${r1} Ω`, 200, 155);
        drawZigZagResistor(ctx, 170, 130, 23, 40);

        ctx.beginPath();
        ctx.moveTo(170, 175);
        ctx.lineTo(170, 230);
        ctx.stroke();

        ctx.fillText(`R2: ${r2} Ω`, 200, 256);
        drawZigZagResistor(ctx, 170, 230, 20, 40);

        ctx.beginPath();
        ctx.moveTo(170, 275);
        ctx.lineTo(170, 300);
        ctx.stroke();
        ctx.fillText('Земля', 160, 330);

        drawGroundSymbol(ctx, 170, 300);

        ctx.beginPath();
        ctx.moveTo(170, 200);
        ctx.lineTo(270, 200);
        ctx.stroke();
        ctx.fillText(`V_out: ${vout.toFixed(2)} V`, 280, 203);
    }

    updateAndCalculate();
</script>
