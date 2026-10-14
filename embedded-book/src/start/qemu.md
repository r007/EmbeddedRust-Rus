# QEMU

Мы начнём писать программу для [LM3S6965] — микроконтроллера Cortex-M3. Мы выбрали его в качестве первой цели, потому что его [можно эмулировать](https://wiki.qemu.org/Documentation/Platforms/ARM#Supported_in_qemu-system-arm) с помощью QEMU. Поэтому в этом разделе вам не придётся возиться с железом, и мы сможем сосредоточиться на инструментах и процессе разработки.

[LM3S6965]: http://www.ti.com/product/LM3S6965

**ВАЖНО**
В этом руководстве мы будем называть проект «app». Всякий раз, когда вы видите слово «app», замените его на имя, которое вы выбрали для своего проекта. Либо назовите проект «app», и подстановки не понадобятся.

## Создание нестандартной программы на Rust

Мы используем шаблон проекта [`cortex-m-quickstart`], чтобы создать из него новый проект. Созданный проект будет содержать минимальное приложение — хорошую отправную точку для нового встраиваемого приложения на Rust. Кроме того, проект будет содержать каталог `examples` с несколькими отдельными приложениями, которые демонстрируют ключевые возможности встраиваемого Rust.

[`cortex-m-quickstart`]: https://github.com/rust-embedded/cortex-m-quickstart

### Использование `cargo-generate`

Сначала установите cargo-generate:
```console
cargo install cargo-generate
```
Затем создайте новый проект:
```console
cargo generate --git https://github.com/knurling-rs/app-template
```

```text
 Project Name: app
 Creating project called `app`...
 Done! New project created /tmp/app
```

```console
cd app
```

### Использование `git`

Клонируйте репозиторий:

```console
git clone https://github.com/rust-embedded/cortex-m-quickstart app
cd app
```

Затем заполните плейсхолдеры в файле `Cargo.toml`:

```toml
[package]
authors = ["{{authors}}"] # "{{authors}}" -> "Иван Иванов"
edition = "2018"
name = "{{project-name}}" # "{{project-name}}" -> "app"
version = "0.1.0"

# ..

[[bin]]
name = "{{project-name}}" # "{{project-name}}" -> "app"
test = false
bench = false
```

### Без использования инструментов

Скачайте последний снимок шаблона `cortex-m-quickstart` и распакуйте его.

```console
curl -LO https://github.com/rust-embedded/cortex-m-quickstart/archive/master.zip
unzip master.zip
mv cortex-m-quickstart-master app
cd app
```

Или вы можете открыть страницу [`cortex-m-quickstart`], нажать зелёную кнопку «Clone or download», а затем «Download ZIP».

Затем заполните плейсхолдеры в файле `Cargo.toml`, как во второй части варианта «Использование `git`».

## Обзор программы

Для удобства здесь приведены самые важные части исходного кода из `src/main.rs`:

```rust,ignore
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {
        // здесь будет ваш код
    }
}
```

Эта программа немного отличается от стандартной программы на Rust, поэтому давайте присмотримся к ней внимательнее.

`#![no_std]` указывает, что программа *не* будет связываться со стандартным крейтом `std`. Вместо этого она будет связываться с его подмножеством — крейтом `core`.

`#![no_main]` указывает, что программа не будет использовать стандартный интерфейс `main`, который применяют большинство программ на Rust. Главная причина использовать `no_main` (тут без каламбура) в том, что интерфейс `main` в контексте `no_std` требует nightly-версии компилятора.

`use panic_halt as _;`. Этот крейт предоставляет `panic_handler`, который определяет поведение программы при панике. Мы подробнее рассмотрим это в главе [Паника](panicking.md) этой книги.

[`#[entry]`][entry] — атрибут, который предоставляет крейт [`cortex-m-rt`] и который используется, чтобы пометить точку входа программы. Поскольку мы не используем стандартный интерфейс `main`, нам нужен другой способ указать точку входа, и таким способом является `#[entry]`.

[entry]: https://docs.rs/cortex-m-rt-macros/latest/cortex_m_rt_macros/attr.entry.html
[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt

`fn main() -> !`. Наша программа будет *единственным* процессом, работающим на целевом оборудовании, поэтому мы не хотим, чтобы она завершалась! Мы используем [расходящуюся функцию](https://doc.rust-lang.org/rust-by-example/fn/diverging.html) (часть `-> !` в сигнатуре функции), чтобы на этапе компиляции гарантировать это.

## Кросс-компиляция

Прежде всего нам понадобится раскладка памяти для целевого микроконтроллера — в нашем случае LM3S6965. Без неё сборка не сможет скомпоновать образ. Создайте в корне проекта файл `memory.x` и вставьте в него следующее содержимое:

```text
MEMORY
{
  /* ЗАМЕЧАНИЕ: 1 K = 1 KiBi = 1024 байта */
  /* TODO: подстройте эти области памяти под раскладку памяти вашего устройства */
  /* Эти значения соответствуют LM3S6965 — одному из немногих устройств, которые может эмулировать QEMU */
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

/* Здесь будет выделен стек вызовов. */
/* Стек имеет полный нисходящий (full descending) тип. */
/* Эту переменную можно использовать, чтобы разместить стек вызовов и статические
   переменные в разных областях памяти. Ниже показано значение по умолчанию */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

/* Этим символом можно задать расположение секции .text */
/* Если его не указывать, секция .text будет размещена сразу после секции .vector_table */
/* Это требуется только на микроконтроллерах, которые хранят некоторую конфигурацию
   сразу после таблицы векторов */
/* _stext = ORIGIN(FLASH) + 0x400; */

/* Пример размещения неинициализированных переменных в произвольных областях RAM. */
/* Предполагается, что вы определили область RAM2 выше, а в исходниках на Rust
   добавили атрибут `#[link_section = ".ram2bss"]` к данным,
   которые хотите там разместить. */
/* Обратите внимание, что эта секция не будет обнулена рантаймом! */
/* SECTIONS {
     .ram2bss (NOLOAD) : ALIGN(4) {
       *(.ram2bss);
       . = ALIGN(4);
     } > RAM2
   } INSERT AFTER .bss;
*/
```

Следующий шаг — *кросс*-компиляция программы для архитектуры Cortex-M3. Это так же просто, как выполнить `cargo build --target $TRIPLE`, если вы знаете, какой должна быть целевая платформа (`$TRIPLE`). К счастью, `.cargo/config.toml` в шаблоне уже содержит ответ:

```console
tail -n6 .cargo/config.toml
```

```toml
[build]
# Выберите ОДНУ из этих целевых платформ
# target = "thumbv6m-none-eabi"    # Cortex-M0 и Cortex-M0+
target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4 и Cortex-M7 (без FPU)
# target = "thumbv7em-none-eabihf" # Cortex-M4F и Cortex-M7F (с FPU)
```

Для кросс-компиляции под архитектуру Cortex-M3 нужно использовать `thumbv7m-none-eabi`. Эта целевая платформа не устанавливается автоматически вместе с тулчейном Rust, поэтому сейчас самое время добавить его в тулчейн, если вы ещё этого не сделали:

``` console
rustup target add thumbv7m-none-eabi
```

Поскольку целевая платформа `thumbv7m-none-eabi` указана по умолчанию в вашем `.cargo/config.toml`, две команды ниже делают одно и то же:

```console
cargo build --target thumbv7m-none-eabi
cargo build
```

## Инспекция

Теперь у нас есть ELF-бинарный файл, не нативный для вашей машины, в `target/thumbv7m-none-eabi/debug/app`. Мы можем исследовать его с помощью `cargo-binutils`.

С помощью `cargo-readobj` мы можем вывести заголовки ELF, чтобы убедиться, что это бинарный файл для ARM.

``` console
cargo readobj --bin app -- --file-headers
```

Обратите внимание:
* `--bin app` — сокращение для инспекции бинарного файла `target/$TRIPLE/debug/app`;
* `--bin app` также (пере)компилирует бинарный файл, если это необходимо

``` text
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0x0
  Type:                              EXEC (Executable file)
  Machine:                           ARM
  Version:                           0x1
  Entry point address:               0x405
  Start of program headers:          52 (bytes into file)
  Start of section headers:          153204 (bytes into file)
  Flags:                             0x5000200
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         2
  Size of section headers:           40 (bytes)
  Number of section headers:         19
  Section header string table index: 18
```

`cargo-size` может вывести размеры секций компоновщика бинарного файла.

```console
cargo size --bin app --release -- -A
```
здесь мы используем `--release`, чтобы посмотреть оптимизированную версию

``` text
app  :
section             size        addr
.vector_table       1024         0x0
.text                 92       0x400
.rodata                0       0x45c
.data                  0  0x20000000
.bss                   0  0x20000000
.debug_str          2958         0x0
.debug_loc            19         0x0
.debug_abbrev        567         0x0
.debug_info         4929         0x0
.debug_ranges         40         0x0
.debug_macinfo         1         0x0
.debug_pubnames     2035         0x0
.debug_pubtypes     1892         0x0
.ARM.attributes       46         0x0
.debug_frame         100         0x0
.debug_line          867         0x0
Total              14570
```

> Напоминание о секциях компоновщика ELF
>
> - `.text` содержит инструкции программы
> - `.rodata` содержит константы, например строки
> - `.data` содержит статически выделенные переменные, начальные значения которых *не* равны нулю
> - `.bss` также содержит статически выделенные переменные, но их начальные значения равны нулю
> - `.vector_table` — *нестандартная* секция, которую мы используем для хранения таблицы векторов (прерываний)
> - `.ARM.attributes` и секции `.debug_*` содержат метаданные и *не* загружаются на целевое устройство при прошивке бинарного файла.

**ВАЖНО**: файлы ELF содержат метаданные, например отладочную информацию, поэтому их *размер на диске* *не* отражает точно объём памяти, который программа займёт на устройстве после прошивки. *Всегда* используйте `cargo-size`, чтобы проверить, насколько большой бинарный файл на самом деле.

С помощью `cargo-objdump` можно выполнить дизассемблирование бинарного файла.

```console
cargo objdump --bin app --release -- --disassemble --no-show-raw-insn --print-imm-hex
```

> **ПРИМЕЧАНИЕ** если приведённая выше команда выдаёт `Unknown command line argument`, см. следующий баг-репорт: https://github.com/rust-embedded/book/issues/269

> **ПРИМЕЧАНИЕ** этот вывод может отличаться в вашей системе. Новые версии rustc, LLVM и библиотек могут генерировать другой ассемблерный код. Мы сократили некоторые инструкции, чтобы фрагмент оставался компактным.

```text
app:  file format ELF32-arm-little

Disassembly of section .text:
main:
     400: bl  #0x256
     404: b #-0x4 <main+0x4>

Reset:
     406: bl  #0x24e
     40a: movw  r0, #0x0
     < .. truncated any more instructions .. >

DefaultHandler_:
     656: b #-0x4 <DefaultHandler_>

UsageFault:
     657: strb  r7, [r4, #0x3]

DefaultPreInit:
     658: bx  lr

__pre_init:
     659: strb  r7, [r0, #0x1]

__nop:
     65a: bx  lr

HardFaultTrampoline:
     65c: mrs r0, msp
     660: b #-0x2 <HardFault_>

HardFault_:
     662: b #-0x4 <HardFault_>

HardFault:
     663: <unknown>
```

## Запуск

Теперь посмотрим, как запустить встраиваемую программу на QEMU! На этот раз мы используем пример `hello`, который уже что-то делает. По умолчанию этот пример использует `[defmt]` и RTT, чтобы выводить текст.

[defmt]: https://defmt.ferrous-systems.com/

> **ПРИМЕЧАНИЕ** `defmt` — сторонняя зависимость (то есть не входящая в core), широко используемая в экосистеме Embedded Rust.

Чтобы читать и декодировать сообщения, которые `defmt` выдаёт на хосте, нам нужно переключить транспорт вывода RTT на semihosting. При работе с реальным оборудованием для этого нужна отладочная сессия, а при использовании QEMU всё работает само собой.

Переключим зависимости:

```console
cargo remove defmt-rtt
cargo add defmt-semihosting
```

Откройте `src/lib.rs` и замените `use defmt_rtt as _;` на `use defmt_semihosting as _;`

Теперь можно собрать пример:

```console
cargo build --bin hello
```

Выходной бинарный файл появится по пути `target/thumbv7m-none-eabi/debug/hello`.

Чтобы запустить этот бинарный файл на QEMU, обычно достаточно такой команды:

```console
qemu-system-arm \
  -cpu cortex-m3 \
  -machine lm3s6965evb \
  -nographic \
  -semihosting-config enable=on,target=native \
  -kernel target/thumbv7m-none-eabi/debug/hello
```

В нашем случае, поскольку мы используем `defmt`, хост не сможет декодировать вывод. Вместо этого нам понадобится инструмент от Ferrous Systems под названием [`qemu-run`]:

[`qemu-run`]: https://github.com/knurling-rs/defmt/tree/main/qemu-run/

```console
git clone git@github.com:knurling-rs/defmt.git
cd defmt/qemu-run/
cargo run -- --machine lm3s6965evb ../qemu-rs/target/thumbv7m-none-eabi/debug/hello
```

```text
Hello, world!
```

Команда должна успешно завершиться (код выхода = 0) после вывода текста. В *nix-системах это можно проверить такой командой:

```console
echo $?
```

```text
0
```

Разберём эту команду QEMU по частям:

- `qemu-system-arm`. Это эмулятор QEMU. Существует несколько вариантов этих бинарных файлов QEMU; этот выполняет полную *системную* эмуляцию машин *ARM*, отсюда и название.

- `-cpu cortex-m3`. Указывает QEMU эмулировать процессор Cortex-M3. Указание модели процессора позволяет поймать некоторые ошибки компиляции: например, запуск программы, скомпилированной для Cortex-M4F, у которого есть аппаратный FPU, приведёт к ошибке QEMU во время выполнения.

- `-machine lm3s6965evb`. Указывает QEMU эмулировать LM3S6965EVB — отладочную плату, на которой стоит микроконтроллер LM3S6965.

- `-nographic`. Указывает QEMU не запускать графический интерфейс.

- `-semihosting-config (..)`. Указывает QEMU включить semihosting. Semihosting позволяет эмулируемому устройству, среди прочего, использовать stdout, stderr и stdin хоста, а также создавать файлы на хосте.

- `-kernel $file`. Указывает QEMU, какой бинарный файл загрузить и запустить на эмулируемой машине.

Вводить такую длинную команду QEMU слишком утомительно! Мы можем задать пользовательский runner, чтобы упростить процесс. В `.cargo/config.toml` есть закомментированный runner, который запускает QEMU; раскомментируем его:

```console
head -n3 .cargo/config.toml
```

```toml
[target.thumbv7m-none-eabi]
# раскомментируйте, чтобы `cargo run` запускал программы на QEMU
runner = "qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel"
```

Этот runner применяется только к целевой платформе `thumbv7m-none-eabi`, которая у нас выбрана по умолчанию. Теперь `cargo run` будет компилировать программу и запускать её на QEMU:

```console
cargo run --example hello --release
```

```text
   Compiling app v0.1.0 (file:///tmp/app)
    Finished release [optimized + debuginfo] target(s) in 0.26s
     Running `qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel target/thumbv7m-none-eabi/release/examples/hello`
Hello, world!
```

## Отладка

Отладка критически важна для разработки встраиваемых систем. Давайте посмотрим, как её выполнять.

Отладка встраиваемого устройства подразумевает *удалённую* отладку, потому что программа, которую мы хотим отлаживать, не будет работать на той же машине, где запущен отладчик (GDB или LLDB).

Удалённая отладка включает клиента и сервер. В конфигурации с QEMU клиентом будет процесс GDB (или LLDB), а сервером — процесс QEMU, который также запускает встраиваемую программу.

В этом разделе мы будем использовать уже собранный пример `hello`.

Первый шаг отладки — запустить QEMU в режиме отладки:

```console
qemu-system-arm \
  -cpu cortex-m3 \
  -machine lm3s6965evb \
  -nographic \
  -semihosting-config enable=on,target=native \
  -gdb tcp::3333 \
  -S \
  -kernel target/thumbv7m-none-eabi/debug/examples/hello
```

Эта команда ничего не выведет в консоль и заблокирует терминал. На этот раз мы передали два дополнительных флага:

- `-gdb tcp::3333`. Указывает QEMU ждать подключения GDB по TCP-порту 3333.

- `-S`. Указывает QEMU заморозить машину при запуске. Без этого программа дошла бы до конца main раньше, чем у нас появилась бы возможность подключить отладчик!

Далее запустите GDB в другом терминале и укажите ему загрузить отладочные символы примера:

```console
gdb-multiarch -q target/thumbv7m-none-eabi/debug/examples/hello
```

**ПРИМЕЧАНИЕ**: в зависимости от того, что вы установили в главе об установке, вместо `gdb-multiarch` может понадобиться другая версия gdb. Это также может быть `arm-none-eabi-gdb` или просто `gdb`.

Затем в оболочке GDB подключаемся к QEMU, который ждёт подключения на TCP-порту 3333.

```console
target remote :3333
```

```text
Remote debugging using :3333
Reset () at $REGISTRY/cortex-m-rt-0.6.1/src/lib.rs:473
473     pub unsafe extern "C" fn Reset() -> ! {
```

Вы увидите, что процесс остановлен, а счётчик команд указывает на функцию с именем `Reset`. Это обработчик сброса — то, что ядра Cortex-M выполняют при загрузке.

>  Обратите внимание, что в некоторых конфигурациях вместо строки `Reset () at $REGISTRY/cortex-m-rt-0.6.1/src/lib.rs:473`, как показано выше, gdb может выводить предупреждения вида:
>
>`core::num::bignum::Big32x40::mul_small () at src/libcore/num/bignum.rs:254`
> `    src/libcore/num/bignum.rs: No such file or directory.`
>
> Это известная особенность. Такие предупреждения можно спокойно игнорировать: скорее всего, вы находитесь в `Reset()`.

Этот обработчик сброса в итоге вызовет нашу функцию main. Давайте перейдём туда с помощью точки останова и команды `continue`. Чтобы установить точку останова, сначала посмотрим, где бы мы хотели остановиться в коде, с помощью команды `list`.

```console
list main
```
Это покажет исходный код из файла examples/hello.rs.

```text
6       use panic_halt as _;
7
8       use cortex_m_rt::entry;
9       use cortex_m_semihosting::{debug, hprintln};
10
11      #[entry]
12      fn main() -> ! {
13          hprintln!("Hello, world!").unwrap();
14
15          // exit QEMU
```
Мы хотим добавить точку останова непосредственно перед «Hello, world!», то есть на строке 13. Делаем это командой `break`:

```console
break 13
```

Теперь можно велеть gdb выполнить код до нашей функции main командой `continue`:

```console
continue
```

```text
Continuing.

Breakpoint 1, hello::__cortex_m_rt_main () at examples\hello.rs:13
13          hprintln!("Hello, world!").unwrap();
```

Мы находимся близко к коду, который выводит «Hello, world!». Продвинемся дальше командой `next`.

``` console
next
```

```text
16          debug::exit(debug::EXIT_SUCCESS);
```

В этот момент вы должны увидеть «Hello, world!», выведенное в терминале, где запущен `qemu-system-arm`.

```text
$ qemu-system-arm (..)
Hello, world!
```

Повторный вызов `next` завершит процесс QEMU.

```console
next
```

```text
[Inferior 1 (Remote target) exited normally]
```

Теперь можно выйти из сеанса GDB.

``` console
quit
```
