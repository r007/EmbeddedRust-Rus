# Оборудование

К этому моменту вы должны уже немного освоить инструменты и процесс разработки. В этом разделе мы перейдём на настоящее оборудование; процесс в основном останется прежним. Давайте начнём.

## Знакомство с вашим оборудованием

Прежде чем начать, нужно выяснить некоторые характеристики целевого устройства, так как они понадобятся для настройки проекта:

- Ядро ARM, например Cortex-M3.

- Есть ли в ядре ARM FPU? Ядра Cortex-M4**F** и Cortex-M7**F** его содержат.

- Сколько флеш-памяти и ОЗУ есть в целевом устройстве? Например, 256 КиБ флеш-памяти и 32 КиБ ОЗУ.

- Где в адресном пространстве отображены флеш-память и ОЗУ? Например, ОЗУ обычно расположено по адресу `0x2000_0000`.

Эту информацию можно найти в техническом описании (data sheet) или справочном руководстве (reference manual) вашего устройства.

В этом разделе мы будем использовать наше эталонное оборудование — STM32F3DISCOVERY. Эта плата содержит микроконтроллер STM32F303VCT6. Он имеет:

- ядро Cortex-M4F, которое включает FPU одинарной точности;

- 256 КиБ флеш-памяти по адресу `0x0800_0000`;

- 40 КиБ ОЗУ по адресу `0x2000_0000`. (Есть ещё одна область ОЗУ, но для простоты мы её учитывать не будем.)

## Настройка

Начнём с нуля, с нового экземпляра шаблона. См. [предыдущий раздел про QEMU] — там есть напоминание о том, как сделать это без `cargo-generate`.

[предыдущий раздел про QEMU]: qemu.md

``` text
$ cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
 Project Name: app
 Creating project called `app`...
 Done! New project created /tmp/app

$ cd app
```

Первый шаг — задать целевую платформу по умолчанию в `.cargo/config.toml`.

``` console
tail -n5 .cargo/config.toml
```

``` toml
# Выберите ОДНУ из этих целевых платформ
# target = "thumbv6m-none-eabi"    # Cortex-M0 и Cortex-M0+
# target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4 и Cortex-M7 (без FPU)
target = "thumbv7em-none-eabihf" # Cortex-M4F и Cortex-M7F (с FPU)
```

Мы будем использовать `thumbv7em-none-eabihf`, так как эта платформа подходит для ядра Cortex-M4F.
> **ПРИМЕЧАНИЕ**: как вы помните из предыдущей главы, каждую целевую платформу нужно устанавливать, а эта — новая. Поэтому не забудьте выполнить `rustup target add thumbv7em-none-eabihf`.

Второй шаг — внести информацию об областях памяти в файл `memory.x`.

``` text
$ cat memory.x
/* Скрипт компоновщика для STM32F303VCT6 */
MEMORY
{
  /* ЗАМЕЧАНИЕ: 1 K = 1 KiBi = 1024 байта */
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}
```
> **ПРИМЕЧАНИЕ**: если по какой-то причине вы изменили файл `memory.x` после первой сборки для конкретной целевой платформы, выполните `cargo clean` перед `cargo build`, так как `cargo build` может не отслеживать изменения в `memory.x`.

Начнём снова с примера hello, но сначала нужно сделать небольшое изменение.

В `examples/hello.rs` убедитесь, что вызов `debug::exit()` закомментирован или удалён. Он нужен только для запуска в QEMU.

```rust,ignore
#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    // выход из QEMU
    // ПРИМЕЧАНИЕ: не запускайте это на реальном оборудовании; это может повредить состояние OpenOCD
    // debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
```

Теперь можно кросс-компилировать программы командой `cargo build`, а бинарные файлы исследовать с помощью `cargo-binutils`, как вы делали раньше. Крейт `cortex-m-rt` берёт на себя всю магию, необходимую, чтобы ваш чип заработал: практически все процессоры Cortex-M загружаются одинаковым образом.

``` console
cargo build --example hello
```

## Отладка

Отладка будет выглядеть немного иначе. Более того, первые шаги могут отличаться в зависимости от целевого устройства. В этом разделе мы покажем шаги, необходимые для отладки программы, запущенной на STM32F3DISCOVERY. Это справочный материал; за информацией об отладке конкретного устройства обращайтесь к [Debugonomicon](https://github.com/rust-embedded/debugonomicon).

Как и раньше, мы будем использовать удалённую отладку, а клиентом будет процесс GDB. На этот раз сервером будет OpenOCD.

Как и в разделе [проверка установки][verify], подключите отладочную плату к ноутбуку или ПК и проверьте, что разъём ST-LINK установлен.

[verify]: ../intro/install/verify.md

В терминале запустите `openocd`, чтобы подключиться к ST-LINK на отладочной плате. Запускайте эту команду из корня шаблона: `openocd` найдёт файл `openocd.cfg`, в котором указано, какой файл интерфейса и какой файл целевого устройства использовать.

``` console
cat openocd.cfg
```

``` text
# Пример конфигурации OpenOCD для отладочной платы STM32F3DISCOVERY

# В зависимости от ревизии вашей платы выберите ОДИН из этих интерфейсов.
# В любой момент времени должна быть раскомментирована только одна строка source.

# Ревизия C (более новая)
source [find interface/stlink.cfg]

# Ревизии A и B (более старые)
# source [find interface/stlink-v2.cfg]

source [find target/stm32f3x.cfg]
```

> **ПРИМЕЧАНИЕ** Если на шаге [проверки установки][verify] вы выяснили, что у вас более старая ревизия отладочной платы, то на этом этапе нужно изменить файл `openocd.cfg`, чтобы использовать `interface/stlink-v2.cfg`.

``` text
$ openocd
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
adapter speed: 1000 kHz
adapter_nsrst_delay: 100
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
none separate
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : clock speed 950 kHz
Info : STLINK v2 JTAG v27 API v2 SWIM v15 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 2.913879
Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

В другом терминале запустите GDB, тоже из корня шаблона.

``` text
gdb-multiarch -q target/thumbv7em-none-eabihf/debug/examples/hello
```

**ПРИМЕЧАНИЕ**: как и раньше, вместо `gdb-multiarch` может понадобиться другая версия gdb — в зависимости от той, что вы установили в главе об установке. Это также может быть `arm-none-eabi-gdb` или просто `gdb`.

Далее подключите GDB к OpenOCD, который ждёт TCP-соединения на порту 3333.

``` console
(gdb) target remote :3333
Remote debugging using :3333
0x00000000 in ?? ()
```

Теперь прошейте (загрузите) программу в микроконтроллер командой `load`.

``` console
(gdb) load
Loading section .vector_table, size 0x400 lma 0x8000000
Loading section .text, size 0x1518 lma 0x8000400
Loading section .rodata, size 0x414 lma 0x8001918
Start address 0x08000400, load size 7468
Transfer rate: 13 KB/sec, 2489 bytes/write.
```

Программа загружена. Она использует semihosting, поэтому перед любым вызовом semihosting нужно сообщить OpenOCD, что semihosting следует включить. Команды для OpenOCD можно отправлять с помощью команды `monitor`.

``` console
(gdb) monitor arm semihosting enable
semihosting is enabled
```

> Все команды OpenOCD можно посмотреть, вызвав команду `monitor help`.

Как и раньше, можно перейти сразу к `main` с помощью точки останова и команды `continue`.

``` console
(gdb) break main
Breakpoint 1 at 0x8000490: file examples/hello.rs, line 11.
Note: automatically using hardware breakpoints for read-only addresses.

(gdb) continue
Continuing.

Breakpoint 1, hello::__cortex_m_rt_main_trampoline () at examples/hello.rs:11
11      #[entry]
```

> **ПРИМЕЧАНИЕ** Если после команды `continue` GDB блокирует терминал вместо того, чтобы остановиться на точке останова, стоит перепроверить, правильно ли в файле `memory.x` заданы области памяти для вашего устройства (и начало, *и* длина).

Войдите в функцию main командой `step`.

``` console
(gdb) step
halted: PC: 0x08000496
hello::__cortex_m_rt_main () at examples/hello.rs:13
13          hprintln!("Hello, world!").unwrap();
```

После продвижения программы командой `next` вы должны увидеть «Hello, world!», выведенное в консоли OpenOCD, среди прочего.

``` console
$ openocd
(..)
Info : halted: PC: 0x08000502
Hello, world!
Info : halted: PC: 0x080004ac
Info : halted: PC: 0x080004ae
Info : halted: PC: 0x080004b0
Info : halted: PC: 0x080004b4
Info : halted: PC: 0x080004b8
Info : halted: PC: 0x080004bc
```
Сообщение выводится только один раз, потому что программа входит в бесконечный цикл, определённый в строке 19: `loop {}`.

Теперь можно выйти из GDB командой `quit`.

``` console
(gdb) quit
A debugging session is active.

        Inferior 1 [Remote target] will be detached.

Quit anyway? (y or n)
```

Отладка теперь требует ещё нескольких шагов, поэтому мы упаковали их в один скрипт GDB с именем `openocd.gdb`. Файл был создан на шаге `cargo generate` и должен работать без изменений. Давайте взглянем:

``` console
cat openocd.gdb
```

``` text
target extended-remote :3333

# выводить демангленные (demangled) символы
set print asm-demangle on

# обнаруживать необработанные исключения, hard fault'ы и паники
break DefaultHandler
break HardFault
break rust_begin_unwind

monitor arm semihosting enable

load

# запустить процесс, но сразу остановить процессор
stepi
```

Теперь запуск `<gdb> -x openocd.gdb target/thumbv7em-none-eabihf/debug/examples/hello` сразу подключит GDB к OpenOCD, включит semihosting, загрузит программу и запустит процесс.

Кроме того, можно превратить `<gdb> -x openocd.gdb` в пользовательский runner, чтобы `cargo run` собирал программу *и* запускал сеанс GDB. Этот runner уже включён в `.cargo/config.toml`, но закомментирован.

``` console
head -n10 .cargo/config.toml
```

``` toml
[target.thumbv7m-none-eabi]
# раскомментируйте, чтобы `cargo run` запускал программы на QEMU
# runner = "qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel"

[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# раскомментируйте ОДИН из этих трёх вариантов, чтобы `cargo run` запускал сеанс GDB
# выбор зависит от вашей системы
runner = "arm-none-eabi-gdb -x openocd.gdb"
# runner = "gdb-multiarch -x openocd.gdb"
# runner = "gdb -x openocd.gdb"
```

``` text
$ cargo run --example hello
(..)
Loading section .vector_table, size 0x400 lma 0x8000000
Loading section .text, size 0x1e70 lma 0x8000400
Loading section .rodata, size 0x61c lma 0x8002270
Start address 0x800144e, load size 10380
Transfer rate: 17 KB/sec, 3460 bytes/write.
(gdb)
```
