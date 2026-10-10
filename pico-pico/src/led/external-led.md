{{#title Мигание внешним светодиодом на Raspberry Pi Pico 2 на embedded Rust}}

# Мигание внешним светодиодом

Начиная с этого момента мы будем использовать с Pico больше внешних компонентов. Прежде чем перейти к ним, полезно освоиться с простыми схемами и научиться подключать компоненты к выводам Pico. В этой главе начнём с простого: будем мигать светодиодом, подключённым вне платы.

## Требования к оборудованию

- Светодиод
- Резистор
- Перемычки

## Обзор компонентов

1. Светодиод (LED, Light Emitting Diode) загорается, когда через него течёт ток. Длинная ножка (анод) подключается к плюсу, короткая (катод) — к земле. Мы подключим анод к GP13 (через резистор), а катод — к GND.

2. Резистор ограничивает ток в цепи, защищая такие компоненты, как светодиоды. Его номинал измеряется в омах (Ω). Мы будем использовать резистор 330 Ом, чтобы безопасно питать светодиод.

<table>
  <thead>
    <tr>
      <th>Вывод Pico</th>
      <th style="width: 250px; margin: 0 auto;">Провод</th>
      <th>Компонент</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>GPIO 13</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire yellow" style="width: 200px; margin: 0 auto;">
          <div class="female-left"></div>
          <div class="female-right"></div>
        </div>
      </td>
      <td>Резистор</td>
    </tr>
    <tr>
      <td>Резистор</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire orange" style="width: 200px; margin: 0 auto;">
          <div class="female-left"></div>
          <div class="female-right"></div>
        </div>
      </td>
      <td>Анод (длинная ножка) светодиода</td>
    </tr>
    <tr>
      <td>GND</td>
      <td style="text-align: center; vertical-align: middle; padding: 0;">
        <div class="wire black" style="width: 200px; margin: 0 auto;">
          <div class="female-left"></div>
          <div class="female-right"></div>
        </div>
      </td>
      <td>Катод (короткая ножка) светодиода</td>
    </tr>
  </tbody>
</table>

<img style="display: block; margin: auto;" alt="Pico 2" src="../images/pico-external-led.png"/>

Подключить Pico к светодиоду можно напрямую перемычками или собрать всё на макетной плате. Если вы не уверены в настройке оборудования, можно обратиться к [руководству Raspberry Pi](https://projects.raspberrypi.org/en/projects/introduction-to-the-pico/7).

<div class="image-with-caption" style="text-align:center; ">
    <img src="./images/pico-2-rp2350-with-external-led.png" alt="Подключение внешнего светодиода к Pico 2 (RP2350)" style="max-width:100%; height:auto; display:block; margin:0 auto;"/>
    <div class="caption" style="font-size:0.9em; color:#555; margin-top:6px;">Схема на макетной плате</div>
</div>

> [!TIP]
> На Pico подписи выводов нанесены на обратной стороне платы, и это может быть неудобно при подключении проводов. Я часто проверял схему распиновки, когда нужно было использовать вывод GPIO. Используйте логотип Raspberry Pi на лицевой стороне как ориентир и сверяйтесь со [схемой распиновки](../pico2-pinout.md), чтобы найти нужные выводы. Номера выводов 2 и 39 также напечатаны на лицевой стороне и могут служить дополнительными ориентирами.

## Мигание светодиодом — симуляция

<style>
.wrap{
  margin:0px auto;
  padding:18px;
  background:linear-gradient(180deg, rgba(255,255,255,0.02), rgba(255,255,255,0.01));
  border-radius:10px;
  box-shadow:0 8px 24px rgba(2,6,23,0.45);
  color:var(--mm-text, #e6eef8);
  font-family:ui-monospace,SFMono-Regular,Menlo,Monaco,monospace;
}

.top{display:flex;gap:18px;align-items:center;flex-wrap:wrap}

/* Визуальное отображение светодиода */
.led{
  width:56px;height:56px;border-radius:50%;background:#334155;display:flex;align-items:center;justify-content:center;
  box-shadow:0 0 0 6px rgba(16,185,129,0.02), inset 0 -6px 12px rgba(0,0,0,0.5);
  transition:background 180ms,box-shadow 180ms,transform 180ms;
}
.led.on{
  background:radial-gradient(circle at 35% 30%, #bbf7d0, #10b981);
  box-shadow:0 0 18px rgba(16,185,129,0.45), 0 6px 20px rgba(0,0,0,0.45);
  transform:scale(1.06);
}
.led-label{font-size:13px;color:#94a3b8;margin-top:8px;text-align:center}

/* Строки кода */
.code{
  margin-top:8px;
  background:#0b1220;
  padding:12px;
  border-radius:8px;
  font-size:14px;
  color:#cbd5e1;
  overflow:auto;
}
.line{padding:6px 8px;border-radius:6px;display:flex;align-items:center;gap:10px}
.line .num{width:28px;color:#94a3b8;text-align:right;padding-right:6px}
.line .text{white-space:pre}
.line.current{
  background:linear-gradient(90deg, rgba(255,0,0,0.35), rgba(255,150,150,0.25));
  box-shadow:inset 0 0 10px rgba(255,0,0,0.45);
}

/* Индикатор прогресса */
.progress-wrap{margin-top:10px}
.progress{height:12px;background:rgba(255,255,255,0.04);border-radius:8px;overflow:hidden}
.bar{height:100%;width:0%;background:linear-gradient(90deg, rgba(16,185,129,0.95), rgba(34,197,94,0.95));transition:width 0.06s linear}
.progress-info{display:flex;justify-content:space-between;margin-top:6px;color:#94a3b8;font-size:13px}

/* Элементы управления */
.controls{margin-top:12px;display:flex;gap:10px;align-items:center;flex-wrap:wrap}
button{background:#0b1826;border:1px solid rgba(255,255,255,0.03);color:inherit;padding:8px 10px;border-radius:8px;cursor:pointer}
button:active{transform:translateY(1px)}
input[type=number]{width:110px;padding:8px;border-radius:8px;border:1px solid rgba(255,255,255,0.04);background:#071023;color:inherit}
</style>

В этой симуляции задержка по умолчанию установлена 5000 миллисекунд, чтобы анимация была спокойнее и её было легче проследить. Можно уменьшить её, например, до 500 миллисекунд, чтобы светодиод мигал быстрее. Когда мы будем запускать реальный код на Pico, используем задержку 500 миллисекунд.

<div class="wrap">
  <div class="top">
    <div style="display:flex;flex-direction:column;align-items:center">
      <div id="led" class="led" aria-hidden="true"></div>
      <div class="led-label"><strong id="led-state">LOW</strong></div>
    </div>
    <div style="flex:1">
      <div class="code" id="code">
        <div class="line" data-index="0"><div class="num">1</div><div class="text">let mut led = Output::new(p.PIN_13, Level::Low);</div></div>
        <div class="line" data-index="1"><div class="num">2</div><div class="text">loop {</div></div>
        <div class="line" data-index="2"><div class="num">3</div><div class="text">    led.set_high(); // Включить светодиод</div></div>
        <div class="line" data-index="3"><div class="num">4</div><div class="text">    Timer::after_millis(<span class="ms-val">5000</span>).await;</div></div>
        <div class="line" data-index="4"><div class="num">5</div><div class="text">    led.set_low(); // Выключить светодиод</div></div>
        <div class="line" data-index="5"><div class="num">6</div><div class="text">    Timer::after_millis(<span class="ms-val">5000</span>).await;</div></div>
        <div class="line" data-index="6"><div class="num">7</div><div class="text">}</div></div>
      <div class="progress-wrap">
        <div class="progress" aria-hidden="true"><div id="bar" class="bar"></div></div>
        <div class="progress-info"><div id="progress-label">Простой</div><div id="ms-left">0 ms</div></div>
      </div>
      <div class="controls">
        <label>Интервал (мс): <input id="interval" type="number" value="5000" min="50" step="50"></label>
        <button id="restart">Перезапуск</button>
        <button id="pause">Пауза</button>
        <button id="resume" style="display:none">Продолжить</button>
      </div>
    </div>
  </div>
</div>

<script>
document.addEventListener("DOMContentLoaded", () => {

  const lines = Array.from(document.querySelectorAll('.line'));
  const ledEl = document.getElementById('led');
  const ledState = document.getElementById('led-state');
  const bar = document.getElementById('bar');
  const progressLabel = document.getElementById('progress-label');
  const msLeft = document.getElementById('ms-left');
  const intervalInput = document.getElementById('interval');
  const restartBtn = document.getElementById('restart');
  const pauseBtn = document.getElementById('pause');
  const resumeBtn = document.getElementById('resume');
  const msValSpans = Array.from(document.querySelectorAll('.ms-val'));

  let intervalMs = Number(intervalInput.value) || 5000;
  let running = true;
  let paused = false;

  function setLed(on){
    if(on){
      ledEl.classList.add('on');
      ledState.textContent = 'HIGH';
    } else {
      ledEl.classList.remove('on');
      ledState.textContent = 'LOW';
    }
  }

  function highlight(index){
    lines.forEach(l => l.classList.toggle('current', Number(l.dataset.index) === index));
  }

  function sleep(ms){
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  // обновляем отображаемое значение миллисекунд в примерах кода
  function updateCodeMs(v){
    msValSpans.forEach(el => el.textContent = v);
  }

  // анимируем таймер и обновляем индикатор прогресса и подписи
  function runTimer(ms, onUpdate){
    return new Promise(resolve => {
      let start = performance.now();
      let lastNow = start;
      function step(now){
        if(paused){
          // сдвигаем начало вперёд, чтобы время не накапливалось во время паузы
          start += (now - lastNow);
          lastNow = now;
          requestAnimationFrame(step);
          return;
        }
        const elapsed = now - start;
        const p = Math.min(1, elapsed / ms);
        onUpdate(p, Math.max(0, ms - Math.floor(elapsed)));
        if(p >= 1){
          resolve();
        } else {
          lastNow = now;
          requestAnimationFrame(step);
        }
      }
      requestAnimationFrame(step);
    });
  }

  async function mainLoop(){
    while(running){
      intervalMs = Number(intervalInput.value) || 5000;
      updateCodeMs(intervalMs);

      // строка: led.set_high();
      highlight(2);
      await sleep(300);           // небольшая пауза, чтобы подсветка была видна
      setLed(true);
      progressLabel.textContent = 'Ожидание после set_high()';

      // строка таймера
      highlight(3);
      bar.style.width = '0%';
      await runTimer(intervalMs, (p, left) => {
        bar.style.width = (p * 100) + '%';
        msLeft.textContent = left + ' ms';
      });

      // строка: led.set_low();
      highlight(4);
      await sleep(300);           // небольшая пауза, чтобы подсветка была видна
      setLed(false);
      progressLabel.textContent = 'Ожидание после set_low()';

      // строка таймера
      highlight(5);
      bar.style.width = '0%';
      await runTimer(intervalMs, (p, left) => {
        bar.style.width = (p * 100) + '%';
        msLeft.textContent = left + ' ms';
      });

      // конец цикла — на мгновение подсвечиваем закрывающую скобку
      highlight(6);
      progressLabel.textContent = 'Новый цикл...';
      await sleep(120);

      bar.style.width = '0%';
      msLeft.textContent = '0 ms';
      progressLabel.textContent = 'Простой';

      // учитываем паузу
      while(paused){
        await sleep(100);
      }
    }
  }

  // элементы управления
  restartBtn.addEventListener('click', () => {
    paused = false;
    running = false;
    setTimeout(() => { running = true; mainLoop(); }, 50);
  });
  pauseBtn.addEventListener('click', () => {
    paused = true;
    pauseBtn.style.display = 'none';
    resumeBtn.style.display = 'inline-block';
    progressLabel.textContent = 'Пауза';
  });
  resumeBtn.addEventListener('click', () => {
    paused = false;
    resumeBtn.style.display = 'none';
    pauseBtn.style.display = 'inline-block';
    progressLabel.textContent = 'Продолжаем...';
  });

  // обновляем интервал при вводе и отражаем его в коде
  intervalInput.addEventListener('input', () => {
    intervalMs = Number(intervalInput.value) || 5000;
    updateCodeMs(intervalMs);
  });

  // инициализация
  updateCodeMs(intervalMs);
  highlight(0);
  setLed(false);
  mainLoop();
});
</script>
