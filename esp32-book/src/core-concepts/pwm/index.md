{{#title ШИМ (PWM) для начинающих: ESP32 и embedded Rust}}

# Широтно-импульсная модуляция (PWM)

<style>
  
  .slider-container {
    margin: 20px 0;
  }

  label {
    margin-right: 10px;
  }

  .led-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    position: relative;
  }

  .led-body {
    width: 30px;
    height: 40px;
    background: radial-gradient(circle at center, #ff5555, #cc0000);
    border-radius: 50% 50% 0 0;
    border: 2px solid #990000;
    position: relative;
    box-shadow: 0 0 10px rgba(255, 85, 85, 0.8);
  }

  .led-body::after {
    content: '';
    position: absolute;
    top: 5px;
    left: 7px;
    width: 16px;
    height: 16px;
    background: rgba(255, 255, 255, 0.4);
    border-radius: 50%;
  }

  .led-pin {
    width: 2px;
    height: 40px;
    background-color: #333;
    position: relative;
  }

  .anode {
    height: 50px; /* Более длинная ножка — анод */
    margin-right: 15px;
    background-color: #666;
  }

  .cathode {
    height: 40px; /* Более короткая ножка — катод */
    margin-left: 15px;
    background-color: #333;
    position: absolute;
    margin-top: 45px;
  }

  canvas {
    border: 1px solid #ccc;
    display: block;
    margin: 10px auto;
  }
  #pwmCanvas {
    background-color: #fefefe;
  } 
</style>

В этом разделе мы разберёмся, что такое ШИМ (PWM) и зачем он нужен.

## Цифровые и аналоговые сигналы
В цифровой схеме сигналы бывают либо высокими (например, 5 В или 3,3 В), либо низкими (0 В), и промежуточных значений нет. Благодаря этим двум различным состояниям цифровые сигналы идеально подходят для компьютеров и цифровых устройств: их легко хранить, считывать и передавать без потери точности.   

Аналоговые сигналы, напротив, могут плавно изменяться в пределах диапазона, принимая любое значение между низким и высоким напряжением. Такое плавное изменение важно для задач, требующих тонкой настройки, например для регулировки громкости звука или яркости света.

Устройствам вроде сервоприводов и светодиодов (для эффекта плавного изменения яркости) часто нужно постепенное и точное управление напряжением, которое аналоговые сигналы обеспечивают благодаря непрерывному диапазону.

Микроконтроллеры используют ШИМ, чтобы преодолеть этот разрыв.

## Что такое ШИМ?
ШИМ (PWM — Pulse Width Modulation, широтно-импульсная модуляция) создаёт аналогоподобный сигнал, быстро переключая цифровой сигнал между включённым и выключенным состоянием. Среднее выходное напряжение можно регулировать, меняя длительность высокого уровня импульса, то есть коэффициент заполнения (duty cycle). Так можно имитировать непрерывный аналоговый уровень. 
 
 <img style="display: block; margin: auto;" alt="ШИМ светодиода" src="../images/led-pwm.jpg" />

## Коэффициент заполнения

**Коэффициент заполнения** (duty cycle) — это процент времени, в течение которого сигнал находится во включённом состоянии за один полный цикл.

Например:
- Коэффициент заполнения 100% означает, что сигнал всегда включён.
- Коэффициент заполнения 50% означает, что сигнал половину времени включён, а половину выключен.
- Коэффициент заполнения 0% означает, что сигнал всегда выключен.




Ниже показана интерактивная симуляция. С помощью ползунков измените коэффициент заполнения и частоту и посмотрите, как меняются ширина импульса и яркость светодиода. Верхняя часть квадратного сигнала означает, что сигнал в высоком состоянии (включён), нижняя часть — что он в низком состоянии (выключен).

<canvas id="pwmCanvas" width="800" height="200"></canvas>
<div class="led-container">
  <div class="led-body" id="ledBody"></div>
  <div class="led-pin anode"></div>
  <div class="led-pin cathode"></div>
</div>

<div class="slider-container">
  <label for="dutyCycle">Коэффициент заполнения (%): </label>
  <input type="range" id="dutyCycle" min="0" max="100" value="50">
  <span id="dutyCycleValue">50</span>%
</div>
<div class="slider-container">
  <label for="frequency">Частота (Гц): </label>
  <input type="range" id="frequency" min="1" max="50" value="10">
  <!-- <span id="frequencyValue">x</span> Hz -->
</div>

Если менять коэффициент заполнения в симуляции от «низкого» к «высокому» и обратно, вы заметите, что светодиод словно плавно меняет яркость. Это и есть эффект диммирования.

## Период и частота
Период — это общее время, за которое завершается один цикл «включено — выключено». 

Частота ШИМ-сигнала — это количество циклов, которые он совершает за одну секунду, измеряется в герцах (Гц). Частота обратно пропорциональна периоду. Поэтому более высокая частота означает более короткий период и более быстрое переключение между высоким и низким уровнями.

\\[
\text{Frequency (Hz)} = \\frac{1}{\text{Period (s)}}
\\]

Если период равен 1 секунде, то частота будет 1 Гц.

\\[
1 \text{Hz} = \\frac{1 \text{ cycle}}{1 \text{ second}} = \\frac{1}{1 \text{ s}}
\\]

Например, если период равен 20 мс (0,02 с), частота будет 50 Гц.

\\[
\text{Frequency} = \\frac{1}{20 \text{ ms}} = \\frac{1}{0.02 \text{ s}} = 50 \text{ Hz}
\\]


**Расчёт количества циклов по частоте**

Формула для расчёта количества циклов:  
\\[
\text{Cycle Count} = \text{Frequency (Hz)} \\times \text{Total Time (seconds)}
\\]

Если ШИМ-сигнал имеет частоту 50 Гц, это значит, что он совершает 50 циклов за одну секунду.

В следующей главе мы подробно разберём ШИМ и таймер.

<script>
  const pwmCanvas = document.getElementById('pwmCanvas');
  const pwmCtx = pwmCanvas.getContext('2d');
  
  const dutyCycleSlider = document.getElementById('dutyCycle');
  const dutyCycleValue = document.getElementById('dutyCycleValue');
  const frequencySlider = document.getElementById('frequency');
  const frequencyValue = document.getElementById('frequencyValue');
  const ledBody = document.getElementById('ledBody');

  let dutyCycle = 50; // Начальный коэффициент заполнения в процентах
  let frequency = 10; // Начальная частота в Гц

  function drawPWM() {
    pwmCtx.clearRect(0, 0, pwmCanvas.width, pwmCanvas.height);

    const period = 1000 / frequency; // Период в мс
    const onTime = period * (dutyCycle / 100); // Время включения в мс
    const offTime = period - onTime; // Время выключения в мс

    const totalWidth = pwmCanvas.width;
    const cycles = frequency; // Количество отображаемых циклов
    const cycleWidth = totalWidth / cycles;

    pwmCtx.strokeStyle = 'black';
    pwmCtx.lineWidth = 2;
    pwmCtx.beginPath();

    let x = 0;

    if (dutyCycle === 100) {
      pwmCtx.moveTo(0, 50);
      pwmCtx.lineTo(pwmCanvas.width, 50);
    } else if (dutyCycle === 0) {
      pwmCtx.moveTo(0, 150);
      pwmCtx.lineTo(pwmCanvas.width, 150);
    } else {
      for (let i = 0; i < cycles; i++) {
        const highWidth = (onTime / period) * cycleWidth;
        const lowWidth = (offTime / period) * cycleWidth;

        pwmCtx.moveTo(x, 50);
        pwmCtx.lineTo(x + highWidth, 50);
        pwmCtx.lineTo(x + highWidth, 150);
        pwmCtx.lineTo(x + highWidth + lowWidth, 150);
        pwmCtx.lineTo(x + highWidth + lowWidth, 50);

        x += cycleWidth;
      }
    }
    pwmCtx.stroke();
  }

  function updateLED() {
    const brightness = dutyCycle / 100;
    
    ledBody.style.background = `radial-gradient(circle at center, rgba(255, 85, 85, ${brightness}), #cc0000)`;
  }

  function update() {
    dutyCycle = parseInt(dutyCycleSlider.value, 10);
    frequency = parseInt(frequencySlider.value, 10);

    dutyCycleValue.textContent = dutyCycle;
    // frequencyValue.textContent = frequency;

    drawPWM();
    updateLED();
  }

  dutyCycleSlider.addEventListener('input', update);
  frequencySlider.addEventListener('input', update);

  // Первоначальная отрисовка
  drawPWM();
  updateLED();
</script>
