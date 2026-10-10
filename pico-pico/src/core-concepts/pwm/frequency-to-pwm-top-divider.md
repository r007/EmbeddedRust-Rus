# Калькулятор TOP и делителя для целевой частоты

В MicroPython можно задать частоту ШИМ напрямую, без ручного расчёта значений TOP и делителя. Внутри MicroPython вычисляет эти значения по целевой частоте и системной частоте тактирования.

Я хотел проверить, есть ли что-то подобное в Rust. Обсуждая это в Matrix-чате rp-rs, 9names перенёс на Rust [соответствующий код на C](https://github.com/micropython/micropython/blob/634125820744efa33679fb95a6e441dadaa4f6a7/ports/rp2/machine_pwm.c#L212C13-L212C36) из MicroPython, который вычисляет значения TOP и делителя. Этот код принимает на вход целевую частоту и частоту источника тактирования и возвращает соответствующие значения TOP и делителя. Реализацию можно найти [здесь](https://github.com/9names/rp2040_rust_playground/tree/main/rp2040-pwm-freq).

Этот Rust-код можно использовать напрямую в своём проекте. Я скомпилировал тот же код в WASM и собрал вокруг него небольшую форму, чтобы вы могли попробовать его здесь.

По умолчанию частота источника тактирования равна системной частоте RP2350 — 150 МГц, а целевая частота — 50 Гц. При необходимости можно изменить оба значения.

<style>
  .pwm-card {
    max-width: 520px;
    padding: 24px;
    border-radius: 12px;
    border: 1px solid #d5c4a1;
    background: #fbf1c7;
    color: #3c3836;
    font-family: system-ui, -apple-system, BlinkMacSystemFont, sans-serif;
  }

  @media (prefers-color-scheme: dark) {
    .pwm-card {
      background: #282828;
      border-color: #504945;
      color: #ebdbb2;
    }
  }

  .pwm-card p {
    margin: 0 0 20px;
    font-size: 14px;
    color: #7c6f64;
  }

  @media (prefers-color-scheme: dark) {
    .pwm-card p {
      color: #a89984;
    }
  }

  /* Поле ввода */
  .pwm-card .pwm-field {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-bottom: 16px;
  }

  .pwm-card .pwm-field label {
    font-size: 14px;
    color: #665c54;
  }

  @media (prefers-color-scheme: dark) {
    .pwm-card .pwm-field label {
      color: #bdae93;
    }
  }

  .pwm-card .pwm-field input {
    font-size: 16px;
    padding: 10px 12px;
    border-radius: 8px;
    border: 1px solid #d5c4a1;
    background: #f9f5d7;
    color: #3c3836;
  }

  @media (prefers-color-scheme: dark) {
    .pwm-card .pwm-field input {
      background: #1d2021;
      border-color: #504945;
      color: #ebdbb2;
    }
  }

  .pwm-card .pwm-field input:focus {
    outline: none;
    border-color: #b57614;
  }

  /* Кнопка */
  .pwm-card .pwm-button {
    width: 100%;
    margin-top: 8px;
    padding: 12px;
    font-size: 15px;
    font-weight: 600;
    border-radius: 10px;
    border: none;
    background: #d79921;
    color: #282828;
    cursor: pointer;
  }

  .pwm-card .pwm-button:hover {
    background: #b57614;
  }

  /* Ошибка */
  .pwm-card .pwm-error {
    margin-top: 16px;
    padding: 12px;
    border-radius: 8px;
    background: #fee2e2;
    color: #7f1d1d;
    font-size: 14px;
  }

  @media (prefers-color-scheme: dark) {
    .pwm-card .pwm-error {
      background: #3c1f1f;
      color: #fca5a5;
    }
  }

  /* Результат */
  .pwm-card .pwm-output {
    margin-top: 24px;
    border-top: 1px solid #d5c4a1;
    padding-top: 16px;
  }

  @media (prefers-color-scheme: dark) {
    .pwm-card .pwm-output {
      border-top-color: #504945;
    }
  }

  .pwm-card .pwm-output h3 {
    margin: 0 0 12px;
    font-size: 16px;
    color: #7c6f64;
  }

  @media (prefers-color-scheme: dark) {
    .pwm-card .pwm-output h3 {
      color: #d5c4a1;
    }
  }

  /* Сетка */
  .pwm-card .pwm-grid {
    display: grid;
    grid-template-columns: 1fr auto;
    row-gap: 10px;
    column-gap: 16px;
    font-size: 15px;
  }

  .pwm-card .pwm-key {
    color: #665c54;
  }

  @media (prefers-color-scheme: dark) {
    .pwm-card .pwm-key {
      color: #bdae93;
    }
  }

  .pwm-card .pwm-value {
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-weight: 600;
    color: #af3a03;
  }

  @media (prefers-color-scheme: dark) {
    .pwm-card .pwm-value {
      color: #fe8019;
    }
  }
</style>

<div class="pwm-card">
  <p>Расчёт TOP и делителя</p>
  <div class="pwm-field">
    <label>Частота источника тактирования (Гц)</label>
    <input id="src" value="150000000">
  </div><div class="pwm-field">
    <label>Целевая частота (Гц)</label>
    <input id="freq" value="50">
  </div>
  <button id="btn" class="pwm-button">Рассчитать</button>
  <div id="error" class="pwm-error" hidden></div>
  <div id="out" class="pwm-output" hidden>
    <h3>Результат</h3>
    <div class="pwm-grid">
      <div class="pwm-key">TOP</div><div class="pwm-value" id="o-top"></div>
      <div class="pwm-key">Делитель (целая часть)</div><div class="pwm-value" id="o-div-int"></div>
      <div class="pwm-key">Делитель (дробная часть)</div><div class="pwm-value" id="o-div-frac"></div>
      <div class="pwm-key">Делитель (итоговый)</div><div class="pwm-value" id="o-div-joined"></div>
      <div class="pwm-key">Фактическая частота (с округлением до 5 знаков)</div><div class="pwm-value" id="o-actual"></div>
    </div>
  </div>
</div>

> [!Note]
> Делитель показан в виде целой и дробной частей.
>
> Дробная часть — это **не** десятичная дробь. Она представляет собой [дробь с фиксированной точкой из 4 бит](https://blog.implrust.com/posts/2025/12/fixed-point-crate-in-rust/).
>
> Эффективный делитель вычисляется так:
>
> `DIV = DIV_INT + (DIV_FRAC / 16)`
>
> Например, `DIV_INT = 45` и `DIV_FRAC = 13` означают, что делитель равен `45 + 0.8125`, а не `45.13`.

## Код

Если вы используете rp-hal, целую и дробную части задают отдельно:

```rust
pwm.set_top(65483);
pwm.set_div_int(45);
pwm.set_div_frac(13);
```

Если вы используете `embassy-rp`, обе части объединяются в одном поле делителя внутри структуры `Config`. Это не значение с плавающей запятой: внутри используется число с фиксированной точкой, которое хранит целую и дробную части вместе. Если вы не знакомы с числами с фиксированной точкой, я написал отдельную статью, где они подробно разобраны, — её можно прочитать [здесь](https://blog.implrust.com/posts/2025/12/fixed-point-crate-in-rust/).

Если нужен только целый делитель, можно просто передать значение типа `u8`:

```rust
let mut servo_config: PwmConfig = Default::default();
servo_config.top = 46_874;
servo_config.divider = 64.into();
```

Если нужна и дробная часть, необходимо добавить крейт `fixed` в зависимости и задать делитель с помощью типа с фиксированной точкой:

```rust
let mut servo_config: PwmConfig = Default::default();
servo_config.top = 65483;
servo_config.divider = FixedU16::<U4>::from_num(45.8125);
// или
// servo_config.divider = fixed::types::U12F4::from_num(45.8125);
```

<script type="module">
  import init, { calculate_pwm } from "./assets/pwm_freq_top.js";
  await init();

  const out = document.getElementById("out");
  const errorBox = document.getElementById("error");

  const oTop = document.getElementById("o-top");
  const oDivInt = document.getElementById("o-div-int");
  const oDivFrac = document.getElementById("o-div-frac");
  const oActual = document.getElementById("o-actual");
  const oDivJoined = document.getElementById("o-div-joined");

  document.getElementById("btn").onclick = () => {
    const src = Number(document.getElementById("src").value);
    const freq = Number(document.getElementById("freq").value);

    errorBox.hidden = true;
    out.hidden = true;

    try {
        const r = calculate_pwm(src, freq);

        oTop.textContent = r.top;
        oDivInt.textContent = r.div_int;
        oDivFrac.textContent = r.div_frac;
        oActual.textContent = r.actual_freq.toFixed(5) + " Hz";

        const joinedDiv = r.div_int + r.div_frac / 16;
        oDivJoined.textContent = joinedDiv.toFixed(4);

        out.hidden = false;
    } catch (e) {
        // console.log(e);
        let message = "Неизвестная ошибка";

        if (String(e).includes("FreqTooLarge")) {
            message =
            "Целевая частота слишком высока для аппаратного ШИМ. Попробуйте более низкую частоту.";
        } else if (String(e).includes("FreqTooSmall")) {
            message =
            "Целевая частота слишком низка для аппаратного ШИМ. Попробуйте более высокую частоту.";
        }

        errorBox.textContent = message;
        errorBox.hidden = false;
    }
  };
</script>
