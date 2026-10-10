# Синглтоны

> В программной инженерии паттерн «синглтон» — это шаблон проектирования, который ограничивает создание класса одним объектом.
>
> *Википедия: [Singleton Pattern]*

[Singleton Pattern]: https://en.wikipedia.org/wiki/Singleton_pattern


## Почему нельзя просто использовать глобальные переменные?

Мы могли бы сделать всё публичной статической переменной, например так:

```rust,ignore
static mut THE_SERIAL_PORT: SerialPort = SerialPort;

fn main() {
    let _ = unsafe {
        THE_SERIAL_PORT.read_speed();
    };
}
```

Но у этого есть несколько проблем. Это изменяемая глобальная переменная, а в Rust работа с такими переменными всегда небезопасна. К тому же эти переменные видны во всей программе, поэтому borrow checker не может помочь отслеживать ссылки и владение ими.

## Как это сделать в Rust?

Вместо того чтобы просто делать периферию глобальной переменной, мы можем создать структуру, в данном случае называемую `PERIPHERALS`, которая содержит `Option<T>` для каждой периферии.

```rust,ignore
struct Peripherals {
    serial: Option<SerialPort>,
}
impl Peripherals {
    fn take_serial(&mut self) -> SerialPort {
        let p = replace(&mut self.serial, None);
        p.unwrap()
    }
}
static mut PERIPHERALS: Peripherals = Peripherals {
    serial: Some(SerialPort),
};
```

Эта структура позволяет получить единственный экземпляр нашей периферии. Если мы попытаемся вызвать `take_serial()` больше одного раза, наш код выполнит панику!

```rust,ignore
fn main() {
    let serial_1 = unsafe { PERIPHERALS.take_serial() };
    // Это вызывает панику!
    // let serial_2 = unsafe { PERIPHERALS.take_serial() };
}
```

Хотя работа с этой структурой — `unsafe`, после того как у нас есть содержащийся в ней `SerialPort`, нам больше не нужен ни `unsafe`, ни сама структура `PERIPHERALS`.

У этого есть небольшие накладные расходы во время выполнения: нам нужно обернуть `SerialPort` в option и один раз вызвать `take_serial()`. Однако эта небольшая стоимость в начале позволяет использовать borrow checker во всей остальной программе.

## Поддержка в существующих библиотеках

Хотя выше мы создали собственную структуру `Peripherals`, для вашего кода это делать не обязательно. В крейте `cortex_m` есть макрос `singleton!()`, который сделает это за вас.

```rust,ignore
use cortex_m::singleton;

fn main() {
    // Допустимо, если `main` выполняется только один раз
    let x: &'static mut bool =
        singleton!(: bool = false).unwrap();
}
```

[документация cortex_m](https://docs.rs/cortex-m/latest/cortex_m/macro.singleton.html)

Кроме того, если вы используете [`cortex-m-rtic`](https://github.com/rtic-rs/cortex-m-rtic), весь процесс определения и получения этой периферии абстрагирован за вас, и вместо этого вы получаете структуру `Peripherals`, которая содержит версию без `Option<T>` для всех элементов, которые вы определили.

```rust,ignore
// cortex-m-rtic v0.5.x
#[rtic::app(device = lm3s6965, peripherals = true)]
const APP: () = {
    #[init]
    fn init(cx: init::Context) {
        static mut X: u32 = 0;
         
        // Периферия Cortex-M
        let core: cortex_m::Peripherals = cx.core;
        
        // Периферия, специфичная для устройства
        let device: lm3s6965::Peripherals = cx.device;
    }
}
```

## Но зачем?

Но как эти синглтоны заметно меняют то, как работает наш код на Rust?

```rust,ignore
impl SerialPort {
    const SER_PORT_SPEED_REG: *mut u32 = 0x4000_1000 as _;

    fn read_speed(
        &self // <------ Это действительно очень важно
    ) -> u32 {
        unsafe {
            ptr::read_volatile(Self::SER_PORT_SPEED_REG)
        }
    }
}
```

Здесь важны два фактора:

* Поскольку мы используем синглтон, есть только один способ или одно место, чтобы получить структуру `SerialPort`;
* Чтобы вызвать метод `read_speed()`, нам нужно иметь владение структурой `SerialPort` или ссылку на неё.

Вместе эти два фактора означают, что доступ к аппаратуре возможен только тогда, когда мы надлежащим образом удовлетворили borrow checker, то есть ни в какой момент у нас не может быть нескольких изменяемых ссылок на одну и ту же аппаратуру!

```rust,ignore
fn main() {
    // Нет ссылки на `self`! Не сработает.
    // SerialPort::read_speed();

    let serial_1 = unsafe { PERIPHERALS.take_serial() };

    // Можно читать только то, к чему у вас есть доступ
    let _ = serial_1.read_speed();
}
```

## Относитесь к аппаратуре как к данным

Кроме того, поскольку одни ссылки изменяемые, а другие — неизменяемые, становится возможным видеть, может ли функция или метод потенциально изменить состояние аппаратуры. Например,

Это разрешено менять настройки аппаратуры:

```rust,ignore
fn setup_spi_port(
    spi: &mut SpiPort,
    cs_pin: &mut GpioPin
) -> Result<()> {
    // ...
}
```

А это — нет:

```rust,ignore
fn read_button(gpio: &GpioPin) -> bool {
    // ...
}
```

Это позволяет проверять на этапе **компиляции**, должен ли код вносить изменения в аппаратуру или нет, а не во время выполнения. Стоит заметить, что это обычно работает только в пределах одного приложения, но для bare-metal систем наше ПО компилируется в одно приложение, поэтому это обычно не является ограничением.
