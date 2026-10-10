# Semihosting

Semihosting — это механизм, который позволяет встраиваемым устройствам выполнять ввод-вывод на хосте; в основном он используется для вывода сообщений в консоль хоста. Для semihosting нужна отладочная сессия и практически ничего больше (никаких дополнительных проводов!), поэтому им очень удобно пользоваться. Недостаток в том, что он очень медленный: каждая операция записи может занимать несколько миллисекунд в зависимости от аппаратного отладчика (например, ST-Link), который вы используете.

Крейт [`cortex-m-semihosting`] предоставляет API для операций semihosting на устройствах Cortex-M. Программа ниже — это версия «Hello, world!», использующая semihosting:

[`cortex-m-semihosting`]: https://crates.io/crates/cortex-m-semihosting

```rust,ignore
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    loop {}
}
```

Если вы запустите эту программу на реальном оборудовании, сообщение «Hello, world!» появится в журналах OpenOCD.

``` text
$ openocd
(..)
Hello, world!
(..)
```

Сначала из GDB нужно включить semihosting в OpenOCD:
``` console
(gdb) monitor arm semihosting enable
semihosting is enabled
```

QEMU понимает операции semihosting, поэтому программа выше будет работать и с `qemu-system-arm` без запуска отладочной сессии. Обратите внимание, что для включения поддержки semihosting нужно передать QEMU флаг `-semihosting-config`; эти флаги уже включены в файл `.cargo/config.toml` шаблона.

``` text
$ # эта программа заблокирует терминал
$ cargo run
     Running `qemu-system-arm (..)
Hello, world!
```

Есть также операция semihosting `exit`, которую можно использовать, чтобы завершить процесс QEMU. Важно: **не** используйте `debug::exit` на реальном оборудовании; эта функция может повредить вашу сессию OpenOCD, и вы не сможете отлаживать другие программы, пока не перезапустите её.

```rust,ignore
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::debug;

#[entry]
fn main() -> ! {
    let roses = "blue";

    if roses == "red" {
        debug::exit(debug::EXIT_SUCCESS);
    } else {
        debug::exit(debug::EXIT_FAILURE);
    }

    loop {}
}
```

``` text
$ cargo run
     Running `qemu-system-arm (..)

$ echo $?
1
```

Ещё один совет: можно настроить поведение при панике так, чтобы оно вызывало `exit(EXIT_FAILURE)`. Это позволит писать `no_std`-тесты, которые проходят выполнение (run-pass), и запускать их на QEMU.

Для удобства крейт `panic-semihosting` имеет функцию «exit», которая при включении вызывает `exit(EXIT_FAILURE)` после вывода сообщения о панике в stderr хоста.

```rust,ignore
#![no_main]
#![no_std]

use panic_semihosting as _; // features = ["exit"]

use cortex_m_rt::entry;
use cortex_m_semihosting::debug;

#[entry]
fn main() -> ! {
    let roses = "blue";

    assert_eq!(roses, "red");

    loop {}
}
```

``` text
$ cargo run
     Running `qemu-system-arm (..)
panicked at 'assertion failed: `(left == right)`
  left: `"blue"`,
 right: `"red"`', examples/hello.rs:15:5

$ echo $?
1
```

**ПРИМЕЧАНИЕ**: чтобы включить эту функцию в `panic-semihosting`, отредактируйте раздел dependencies в вашем `Cargo.toml`, где указан `panic-semihosting`, следующим образом:

``` toml
panic-semihosting = { version = "VERSION", features = ["exit"] }
```

где `VERSION` — нужная версия. Подробнее о функциях зависимостей см. раздел [`specifying dependencies`] в книге Cargo.

[`specifying dependencies`]:
https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
