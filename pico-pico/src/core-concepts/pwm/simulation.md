{{#title Симуляция ШИМ: коэффициент заполнения и частота в embedded Rust}}

# Симуляция

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
    height: 50px; /* Более длинный вывод для анода */
    margin-right: 15px;
    background-color: #666;
  }

  .cathode {
    height: 40px; /* Более короткий вывод для катода */
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

Ниже — интерактивная симуляция. Используйте ползунки, чтобы менять коэффициент заполнения и частоту, и наблюдайте, как меняются ширина импульса и яркость светодиода. Верхняя часть прямоугольного сигнала соответствует состоянию «включено» (HIGH), нижняя — «выключено» (LOW). Ширина высокого участка меняется вместе со коэффициентом заполнения.

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

Если в симуляции плавно менять коэффициент заполнения от низкого к высокому и обратно, вы заметите, что светодиод как будто меняет яркость.

<script>
  const pwmCanvas = document.getElementById('pwmCanvas');
  const pwmCtx = pwmCanvas.getContext('2d');

  const dutyCycleSlider = document.getElementById('dutyCycle');
  const dutyCycleValue = document.getElementById('dutyCycleValue');
  const frequencySlider = document.getElementById('frequency');
  const frequencyValue = document.getElementById('frequencyValue');
  const ledBody = document.getElementById('ledBody');

  let dutyCycle = 50; // Начальная коэффициент заполнения в процентах
  let frequency = 10; // Начальная частота в Гц

  function drawPWM() {
    pwmCtx.clearRect(0, 0, pwmCanvas.width, pwmCanvas.height);

    const period = 1000 / frequency; // Период в мс
    const onTime = period * (dutyCycle / 100); // Время включения в мс
    const offTime = period - onTime; // Время выключения в мс

    const totalWidth = pwmCanvas.width;
    const cycles = frequency; // Количество периодов для отображения
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

  // Первичная отрисовка
  drawPWM();
  updateLED();
</script>
