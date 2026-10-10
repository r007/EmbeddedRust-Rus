# Пошаговое руководство по решению

1. Инициализируйте периферию светодиода и включите его с произвольным значением, просто чтобы убедиться, что он работает.
   ```rust
{{#include ../../advanced/button-interrupt/examples/solution_led.rs:led}}

    led.set_pixel(RGB8::new(20, 0, 20)).unwrap(); // Удалите эту строку после того, как попробуете один раз
   ```
2. Включайте светодиод только при нажатии кнопки. Пока это можно сделать, добавив следующую строку после сообщения о нажатии кнопки:
   ```rust
   led.set_pixel(arbitrary_color)?;
   ```
3. Создайте случайные RGB-значения, вызвав `esp_random()`.
   * Эта функция `unsafe`.
   * Она возвращает `u32`, поэтому её нужно привести к `u8`.

    ```rust
    unsafe {
    //...
    1 => {
        let r = esp_random() as u8;
        let g = esp_random() as u8;
        let b = esp_random() as u8;

        let color = RGB8::new(r, g, b);
        led.set_pixel(color)?;

        },
    _ => {},
   ```

4. **Необязательно**: если вы собираетесь переиспользовать этот код в другом месте, имеет смысл вынести его в отдельную функцию. Так мы подробно разберём, какие части кода должны находиться в блоках `unsafe`.

```rust
// ...
{{#include ../../advanced/button-interrupt/examples/solution_led.rs:loop}}

// ...
{{#include ../../advanced/button-interrupt/examples/solution_led.rs:random_light}}

```

