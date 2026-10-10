# Рекомендации для интерфейсов GPIO

<a id="c-zst-pin"></a>
## Типы выводов по умолчанию имеют нулевой размер (C-ZST-PIN)

Интерфейсы GPIO, предоставляемые HAL, должны предоставлять выделенные типы нулевого размера для каждого вывода на каждом интерфейсе или порту. Это даёт абстракцию GPIO с нулевой стоимостью, когда все назначения выводов известны статически.

Каждый интерфейс или порт GPIO должен реализовывать метод `split`, который возвращает структуру со всеми выводами.

Пример:

```rust
pub struct PA0;
pub struct PA1;
// ...

pub struct PortA;

impl PortA {
    pub fn split(self) -> PortAPins {
        PortAPins {
            pa0: PA0,
            pa1: PA1,
            // ...
        }
    }
}

pub struct PortAPins {
    pub pa0: PA0,
    pub pa1: PA1,
    // ...
}
```

<a id="c-erased-pin"></a>
## Типы выводов предоставляют методы стирания вывода и порта (C-ERASED-PIN)

Выводы должны предоставлять методы стирания типа, которые переносят их свойства из времени компиляции во время выполнения и дают больше гибкости в приложениях.

Пример:

```rust
/// Порт A, вывод 0.
pub struct PA0;

impl PA0 {
    pub fn erase_pin(self) -> PA {
        PA { pin: 0 }
    }
}

/// Вывод порта A.
pub struct PA {
    /// Номер вывода.
    pin: u8,
}

impl PA {
    pub fn erase_port(self) -> Pin {
        Pin {
            port: Port::A,
            pin: self.pin,
        }
    }
}

pub struct Pin {
    port: Port,
    pin: u8,
    // (эти поля можно упаковать, чтобы уменьшить объём памяти)
}

enum Port {
    A,
    B,
    C,
    D,
}
```

<a id="c-pin-state"></a>
## Состояние вывода должно кодироваться как параметры типа (C-PIN-STATE)

Выводы могут быть настроены как входы или выходы с разными характеристиками в зависимости от микросхемы или семейства. Это состояние должно кодироваться в системе типов, чтобы предотвратить использование выводов в неправильных состояниях.

Дополнительное, специфичное для микросхемы состояние (например, нагрузочная способность выхода) также можно кодировать таким образом, используя дополнительные параметры типа.

Методы изменения состояния вывода должны предоставляться в виде методов `into_input` и `into_output`.

Кроме того, должны предоставляться методы `with_{input,output}_state`, которые временно переконфигурируют вывод в другое состояние, не перемещая его.

Для каждого типа вывода должны предоставляться следующие методы (то есть и стёртые, и нестёртые типы выводов должны предоставлять один и тот же API):

* `pub fn into_input<N: InputState>(self, input: N) -> Pin<N>`
* `pub fn into_output<N: OutputState>(self, output: N) -> Pin<N>`
* ```ignore
  pub fn with_input_state<N: InputState, R>(
      &mut self,
      input: N,
      f: impl FnOnce(&mut PA1<N>) -> R,
  ) -> R
  ```
* ```ignore
  pub fn with_output_state<N: OutputState, R>(
      &mut self,
      output: N,
      f: impl FnOnce(&mut PA1<N>) -> R,
  ) -> R
  ```


Состояние вывода должно ограничиваться запечатанными трейтами (sealed traits). Пользователям HAL не должно требоваться добавлять собственные состояния. Трейты могут предоставлять специфичные для HAL методы, необходимые для реализации API состояний вывода.

Пример:

```rust
# use std::marker::PhantomData;
mod sealed {
    pub trait Sealed {}
}

pub trait PinState: sealed::Sealed {}
pub trait OutputState: sealed::Sealed {}
pub trait InputState: sealed::Sealed {
    // ...
}

pub struct Output<S: OutputState> {
    _p: PhantomData<S>,
}

impl<S: OutputState> PinState for Output<S> {}
impl<S: OutputState> sealed::Sealed for Output<S> {}

pub struct PushPull;
pub struct OpenDrain;

impl OutputState for PushPull {}
impl OutputState for OpenDrain {}
impl sealed::Sealed for PushPull {}
impl sealed::Sealed for OpenDrain {}

pub struct Input<S: InputState> {
    _p: PhantomData<S>,
}

impl<S: InputState> PinState for Input<S> {}
impl<S: InputState> sealed::Sealed for Input<S> {}

pub struct Floating;
pub struct PullUp;
pub struct PullDown;

impl InputState for Floating {}
impl InputState for PullUp {}
impl InputState for PullDown {}
impl sealed::Sealed for Floating {}
impl sealed::Sealed for PullUp {}
impl sealed::Sealed for PullDown {}

pub struct PA1<S: PinState> {
    _p: PhantomData<S>,
}

impl<S: PinState> PA1<S> {
    pub fn into_input<N: InputState>(self, input: N) -> PA1<Input<N>> {
        todo!()
    }

    pub fn into_output<N: OutputState>(self, output: N) -> PA1<Output<N>> {
        todo!()
    }

    pub fn with_input_state<N: InputState, R>(
        &mut self,
        input: N,
        f: impl FnOnce(&mut PA1<N>) -> R,
    ) -> R {
        todo!()
    }

    pub fn with_output_state<N: OutputState, R>(
        &mut self,
        output: N,
        f: impl FnOnce(&mut PA1<N>) -> R,
    ) -> R {
        todo!()
    }
}

// То же самое для `PA` и `Pin`, а также других типов выводов.
```
