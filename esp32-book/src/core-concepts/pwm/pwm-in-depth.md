{{#title Таймеры ШИМ и коэффициент заполнения во встраиваемых системах}}

# ШИМ в деталях

<style> 
canvas {
    border: 1px solid #000;
    margin-top: 20px;
}
.controls {
    margin: 20px;
}
.control {
    margin: 10px 0;
}
#pwmCanvas {
    background-color: #fefefe;
}
#timerCanvas {
    background-color: #fefefe;
}
#simulation-div{
        background-color: #fefefe;
            text-align: center;

}   
</style>

## Работа таймера
Таймер играет ключевую роль в генераторе ШИМ. Он считает от нуля до заданного максимального значения (хранящегося в регистре), затем сбрасывается и начинает цикл заново. Этот подсчёт определяет длительность одного полного цикла, которая называется периодом.


## Значение сравнения (Compare Value)
Аппаратная часть таймера сравнивает его текущее значение с значением сравнения (хранящимся в регистре). Пока счётчик меньше значения сравнения, сигнал остаётся высоким; когда счётчик превышает значение сравнения, сигнал переходит в низкий уровень.


## Разрешение ШИМ (PWM Resolution)
В ШИМ (широтно-импульсной модуляции) разрешение — это то, насколько точно можно управлять коэффициентом заполнения. Оно определяется количеством битов, которые используются в регистре сравнения ШИМ.

Таймер считает от 0 до максимального значения, которое зависит от разрешения. Чем выше разрешение, тем точнее можно настраивать коэффициент заполнения.

Для системы с **n** битами разрешения таймер может считать от 0 до \\(2^n - 1\\), что даёт \\(2^n\\) возможных уровней коэффициента заполнения.

Например:
- 8-битное разрешение позволяет таймеру считать от 0 до 255, то есть даёт 256 возможных уровней коэффициента заполнения.
- 10-битное разрешение позволяет таймеру считать от 0 до 1023, то есть даёт 1024 возможных уровня коэффициента заполнения.

Более высокое разрешение обеспечивает более точное управление коэффициентом заполнения, но при этом таймеру нужно считать больше значений в пределах того же периода, что может снизить частоту или потребовать больше вычислительной мощности. По сути, разрешение определяет, сколько различных значений коэффициента заполнения можно задать: чем больше битов, тем тоньше настройка.


## Симуляция

В этой симуляции вы можете менять разрешение ШИМ в битах и коэффициент заполнения. Изменение разрешения в битах увеличивает максимальное значение счётчика, но остаётся в пределах периода (на коэффициент заполнения оно не влияет). Изменение коэффициента заполнения соответственно меняет состояния «включено» и «выключено», но тоже остаётся в пределах периода.
 
  <div class="controls">
    <div class="control">
      <label for="resolution">Разрешение ШИМ (биты): </label>
      <input type="range" id="resolution" min="4" max="20" value="8">
      <input type="number" id="resolutionNumber" min="4" max="20" value="8">
    </div>
    <div class="control">
      <label for="dutyCycle">Коэффициент заполнения (%): </label>
      <input type="range" id="dutyCycle" min="0" max="100" value="75">
      <input type="number" id="dutyCycleNumber" min="0" max="100" value="75">
    </div>
  </div>
 <div id='simulation-div'>
    <canvas id="timerCanvas" width="800" height="200"></canvas>
    <canvas id="pwmCanvas" width="800" height="150"></canvas>
  </div>

## Связь между коэффициентом заполнения, частотой и разрешением

Эта диаграмма иллюстрирует связь между коэффициентом заполнения, частотой, периодом, шириной импульса и разрешением. На первый взгляд она может показаться сложной, но если разобрать её по частям, всё становится яснее.

В этом примере разрешение таймера равно 4 битам, то есть таймер считает от 0 до 15. Когда таймер достигает максимального значения, срабатывает прерывание переполнения (синяя стрелка), и счётчик сбрасывается в 0. Время, которое таймеру нужно, чтобы досчитать от 0 до максимума, называется «периодом».

<img style="display: block; margin: auto;" alt="ШИМ" src="../images/pwm-duty-cycle-timer.jpg"/>

Коэффициент заполнения задан равным 50%, то есть сигнал остаётся в высоком состоянии половину периода. На каждом шаге счёта таймер сравнивает текущее значение счётчика со значением сравнения для коэффициента заполнения. Когда значение счётчика превышает это значение сравнения (жёлтая стрелка), сигнал переходит из высокого состояния в низкое. Это вызывает прерывание сравнения, которое сигнализирует о смене состояния.

Время, в течение которого сигнал находится в высоком состоянии, называется шириной импульса.

  <script>
    const timerCanvas = document.getElementById('timerCanvas');
    const timerCtx = timerCanvas.getContext('2d');

    const pwmCanvas = document.getElementById('pwmCanvas');
    const pwmCtx = pwmCanvas.getContext('2d');

    const resolutionInput = document.getElementById('resolution');
    const resolutionNumber = document.getElementById('resolutionNumber');
    const dutyCycleInput = document.getElementById('dutyCycle');
    const dutyCycleNumber = document.getElementById('dutyCycleNumber');

    const numCountLabels = 1;

    function drawTimerAndPWM(resolution, dutyCycle) {
      const maxTicks = Math.pow(2, resolution) - 1;
      const highTicks = Math.round(maxTicks * (dutyCycle / 100));
      const periodWidth = pwmCanvas.width / 10;

      timerCtx.clearRect(0, 0, timerCanvas.width, timerCanvas.height);
      pwmCtx.clearRect(0, 0, pwmCanvas.width, pwmCanvas.height);

      timerCtx.beginPath();
      const stepsToDraw = Math.min(800, maxTicks);
      const stepIncrement = Math.ceil(maxTicks / stepsToDraw);
      for (let period = 0; period < 10; period++) {
        const startX = period * periodWidth;
        for (let tick = 0; tick <= maxTicks; tick += stepIncrement) {
          const x1 = startX + (tick / maxTicks) * periodWidth;
          const y1 = timerCanvas.height - (tick / maxTicks) * (timerCanvas.height - 100);
          const y2 = timerCanvas.height - ((tick + stepIncrement) / maxTicks) * (timerCanvas.height - 100);
          timerCtx.lineTo(x1, y1);
          if (tick + stepIncrement <= maxTicks) {
            timerCtx.lineTo(x1, y2);
          }
        }
      }
      timerCtx.strokeStyle = 'blue';
      timerCtx.lineWidth = 2;
      timerCtx.stroke();

      timerCtx.font = '12px Arial';
      timerCtx.fillStyle = 'black';
      let labelValues = [];
      if (numCountLabels == 1) {
        labelValues = [0, maxTicks];
      } else if (numCountLabels == 2) {
        labelValues = [0, Math.round(maxTicks / 2), maxTicks];
      } else {
        labelValues = [0, Math.round(maxTicks / 3), Math.round(2 * maxTicks / 3), maxTicks];
      }

      for (let i = 0; i < labelValues.length; i++) {
        const y = timerCanvas.height - (labelValues[i] / maxTicks) * (timerCanvas.height - 100);
        timerCtx.fillText(Math.round(labelValues[i]), 5, y);
      }

      const dutyCycleY = timerCanvas.height - (highTicks / maxTicks) * (timerCanvas.height - 100);
      timerCtx.beginPath();
      timerCtx.moveTo(0, dutyCycleY);
      timerCtx.lineTo(timerCanvas.width, dutyCycleY);
      timerCtx.strokeStyle = 'red';
      timerCtx.lineWidth = 1;
      timerCtx.stroke();

      const maxValueY = timerCanvas.height - (maxTicks / maxTicks) * (timerCanvas.height - 100);
      timerCtx.beginPath();
      timerCtx.moveTo(0, maxValueY);
      timerCtx.lineTo(timerCanvas.width, maxValueY);
      timerCtx.strokeStyle = 'gray';
      timerCtx.lineWidth = 1;
      timerCtx.setLineDash([5, 5]);
      timerCtx.stroke();
      timerCtx.setLineDash([]);

      pwmCtx.beginPath();
      for (let period = 0; period < 10; period++) {
        const startX = period * periodWidth;
        pwmCtx.moveTo(startX, pwmCanvas.height / 2);
        pwmCtx.lineTo(startX + (dutyCycle / 100) * periodWidth, pwmCanvas.height / 2);
        pwmCtx.lineTo(startX + (dutyCycle / 100) * periodWidth, pwmCanvas.height - 20);
        pwmCtx.lineTo(startX + periodWidth, pwmCanvas.height - 20);
        pwmCtx.lineTo(startX + periodWidth, pwmCanvas.height / 2);
      }
      pwmCtx.strokeStyle = 'green';
      pwmCtx.lineWidth = 2;
      pwmCtx.stroke();

      timerCtx.font = '16px Arial';
      timerCtx.fillStyle = 'black';
      timerCtx.fillText(`Разрешение: ${resolution} бит`, 10, 20);
      timerCtx.fillText(`Максимальное значение счётчика: ${maxTicks}`, 10, 40);

      pwmCtx.font = '16px Arial';
      pwmCtx.fillStyle = 'black';
      pwmCtx.fillText(`Коэффициент заполнения: ${dutyCycle}%`, 10, 20);
    }

    function updateSimulation() {
      const resolution = parseInt(resolutionInput.value);
      const dutyCycle = parseInt(dutyCycleInput.value);
      drawTimerAndPWM(resolution, dutyCycle);
    }

    resolutionInput.addEventListener('input', () => {
      resolutionNumber.value = resolutionInput.value;
      updateSimulation();
    });

    resolutionNumber.addEventListener('input', () => {
      resolutionInput.value = resolutionNumber.value;
      updateSimulation();
    });

    dutyCycleInput.addEventListener('input', () => {
      dutyCycleNumber.value = dutyCycleInput.value;
      updateSimulation();
    });

    dutyCycleNumber.addEventListener('input', () => {
      dutyCycleInput.value = dutyCycleNumber.value;
      updateSimulation();
    });

    updateSimulation();
  </script>
