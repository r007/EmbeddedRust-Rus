{{#title Как работает LDR в схеме делителя напряжения}}

# Как работает LDR?

Мы уже дали введение в то, что такое LDR. Повторю ещё раз: LDR меняет своё сопротивление в зависимости от количества света, падающего на него. Чем ярче свет, тем меньше сопротивление, и чем темнее, тем оно больше.

Дракула: представьте, что LDR — это Дракула. На солнечном свету он слабеет (точно так же, как сопротивление становится меньше). А в темноте он становится сильнее (точно так же, как сопротивление растёт).

Мы не будем подробно разбирать, из каких полупроводниковых материалов делают LDR и почему он ведёт себя именно так. Рекомендую прочитать эту [статью](https://www.elprocus.com/ldr-light-dependent-resistor-circuit-and-working/) и изучить тему подробнее, если интересно.

### Пример выходного напряжения при полной освещённости
Сопротивление LDR низкое при полной освещённости, из-за чего выходное напряжение (\\( V_{out} \\)) становится заметно ниже.

<img style="display: block; margin: auto;" alt="voltage-divider-ldr1" src="./images/voltage-divider-ldr1.png"/>


### Пример выходного напряжения при слабом свете
При меньшем количестве света сопротивление LDR растёт, и выходное напряжение увеличивается.

<img style="display: block; margin: auto;" alt="voltage-divider-ldr2" src="./images/voltage-divider-ldr2.png"/>

### Пример выходного напряжения в полной темноте
В темноте сопротивление LDR высокое, из-за чего выходное напряжение (\\( V_{out} \\)) выше.

<img style="display: block; margin: auto;" alt="voltage-divider-ldr3" src="./images/voltage-divider-ldr3.png"/>


## Симуляция LDR в делителе напряжения
Вы можете менять значение яркости и наблюдать, как меняется сопротивление R2 (то есть LDR). Также можно смотреть, как меняется напряжение \\( V_{out} \\) при увеличении или уменьшении яркости.

<style>
canvas {
    border: 1px solid #ccc;
    margin-top: 20px;
    background:white;
}
</style>
<label for="vin">Входное напряжение (Vin):</label>
<input type="number" id="vin" step="0.01" value="3.3" oninput="updateAndCalculate()"><br><br>
<label for="r1">Резистор R1 (Ω):</label>
<input type="number" id="r1" step="1" value="10000" oninput="updateAndCalculate()"><br><br>
<label for="brightness">Яркость:</label>
<input type="range" id="brightness" min="0" max="100" value="50" oninput="updateAndCalculate()">
<span id="brightnessValue">50</span>%<br><br>
<p class="formula" id="formula">
    Формула: \( V_\text{out} = V_\text{in} \times \frac{R_2}{R_1 + R_2} \)
</p>
<p class="formula" id="filledFormula">
    Подставленная формула: V<sub>out</sub> = 3.3 × 999 / (10000 + 999)
</p>
<p id="result">Выходное напряжение (V<sub>out</sub>): 0.25 V</p>
<canvas id="circuitCanvas" width="600" height="400"></canvas>


### Circuitjs 
Приведённые выше схемы я создал на сайте Falstad. Вы можете импортировать файл схемы, который я сделал, [`voltage-divider-ldr.circuitjs.txt`](./voltage-divider-ldr.circuitjs.txt), на [сайт Falstad](https://www.falstad.com/circuit/) и поэкспериментировать.
 

<script>
        function updateAndCalculate() {
            updateFormula();
            calculateVoltage();
            drawCircuit();
        }
        function updateFormula() {
            const vin = parseFloat(document.getElementById('vin').value) || 0;
            const r1 = parseFloat(document.getElementById('r1').value) || 0;
            const brightness = parseFloat(document.getElementById('brightness').value);
            const rMin = 999;
            const rMax = 100000;
            const r2 = rMin + (rMax - rMin) * (100 - brightness) / 100;
            document.getElementById('filledFormula').innerHTML = 
                `Подставленная формула: V<sub>out</sub> = ${vin.toFixed(2)} × ${r2.toFixed(2)} / (${r1} + ${r2.toFixed(2)})`;
        }
        function calculateVoltage() {
            const vin = parseFloat(document.getElementById('vin').value);
            const r1 = parseFloat(document.getElementById('r1').value);
            const brightness = parseFloat(document.getElementById('brightness').value);
            const rMin = 999;
            const rMax = 100000;
            const r2 = rMin + (rMax - rMin) * (100 - brightness) / 100;
            document.getElementById('brightnessValue').textContent = brightness;
            if (isNaN(vin) || isNaN(r1) || r1 <= 0 || r2 <= 0) {
                document.getElementById('result').textContent = 
                    "Пожалуйста, введите корректные положительные числа во все поля.";
                return;
            }
            const vout = vin * (r2 / (r1 + r2));
            document.getElementById('result').innerHTML = 
                `Выходное напряжение (V<sub>out</sub>): ${vout.toFixed(2)} V`;
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
        function drawArrow(ctx, x, y, length, angle) {
            ctx.beginPath();
            ctx.moveTo(x, y);
            ctx.lineTo(x + length * Math.cos(angle), y + length * Math.sin(angle));
            ctx.lineWidth = 1;
            ctx.stroke();
            const arrowSize = 11;
            const angleLeft = angle + Math.PI / 6;
            const angleRight = angle - Math.PI / 6;
            ctx.beginPath();
            ctx.moveTo(x + length * Math.cos(angle), y + length * Math.sin(angle));
            ctx.lineTo(x + (length - arrowSize) * Math.cos(angleLeft), y + (length - arrowSize) * Math.sin(angleLeft));
            ctx.moveTo(x + length * Math.cos(angle), y + length * Math.sin(angle));
            ctx.lineTo(x + (length - arrowSize) * Math.cos(angleRight), y + (length - arrowSize) * Math.sin(angleRight));
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
        function formatResistance(resistance) {
            return resistance >= 1000
                ? `${(resistance / 1000).toFixed(1)} kΩ`
                : `${resistance.toFixed(0)} Ω`;
        }
        function drawCircuit() {
            const canvas = document.getElementById('circuitCanvas');
            const ctx = canvas.getContext('2d');
            const vin = parseFloat(document.getElementById('vin').value) || 0;
            const r1 = parseFloat(document.getElementById('r1').value) || 0;
            const brightness = parseFloat(document.getElementById('brightness').value);

            const rMin = 999;
            const rMax = 100000;

            const r2 = rMin + (rMax - rMin) * (100 - brightness) / 100;

            const vout = (r1 + r2) !== 0 ? (vin * (r2 / (r1 + r2))) : 0;

            ctx.clearRect(0, 0, canvas.width, canvas.height);

            ctx.fillStyle = 'black';
            ctx.font = '14px Arial';

            ctx.fillText(`V_in: ${vin.toFixed(2)} V`, 38, 96);
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

            ctx.fillText(`R1: ${formatResistance(r1)}`, 200, 155);
            drawZigZagResistor(ctx, 170, 130, 23, 40);

            ctx.beginPath();
            ctx.moveTo(170, 175);
            ctx.lineTo(170, 230);
            ctx.stroke();

            ctx.fillText(`R2: ${formatResistance(r2)}`, 200, 256);
            drawZigZagResistor(ctx, 170, 230, 20, 40);

            const arrowX = 170;
            const arrowY = 220;
            const angle = Math.PI / 4;

            drawArrow(ctx, arrowX - 30, arrowY, 25, angle);
            drawArrow(ctx, arrowX - 33, arrowY + 22, 25, angle);

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
