# Содержание

<!--

Структура этой книги всё ещё находится в процессе определения.

Подробности и координацию см. на https://github.com/r007/EmbeddedRust-Rus/issues

-->

- [Введение](./intro/index.md)
    - [Знакомство с оборудованием](./intro/hardware.md)
    - [`no_std`](./intro/no-std.md)
    - [Инструменты](./intro/tooling.md)
    - [Установка](./intro/install.md)
        - [Linux](./intro/install/linux.md)
        - [MacOS](./intro/install/macos.md)
        - [Windows](./intro/install/windows.md)
        - [Проверка установки](./intro/install/verify.md)
- [Начало работы](./start/index.md)
  - [QEMU](./start/qemu.md)
  - [Оборудование](./start/hardware.md)
  - [Регистры, отображённые в память](./start/registers.md)
  - [Semihosting](./start/semihosting.md)
  - [Паника](./start/panicking.md)
  - [Исключения](./start/exceptions.md)
  - [Прерывания](./start/interrupts.md)
- [Периферия](./peripherals/index.md)
    - [Первая попытка на Rust](./peripherals/a-first-attempt.md)
    - [Проверка заимствований](./peripherals/borrowck.md)
    - [Синглтоны](./peripherals/singletons.md)
- [Статические гарантии](./static-guarantees/index.md)
    - [Программирование на typestate](./static-guarantees/typestate-programming.md)
    - [Периферия как конечные автоматы](./static-guarantees/state-machines.md)
    - [Проектные контракты](./static-guarantees/design-contracts.md)
    - [Абстракции с нулевой стоимостью](./static-guarantees/zero-cost-abstractions.md)
- [Переносимость](./portability/index.md)
- [Конкурентность](./concurrency/index.md)
- [Коллекции](./collections/index.md)
- [Паттерны проектирования](./design-patterns/index.md)
    - [HAL](./design-patterns/hal/index.md)
        - [Чек-лист](./design-patterns/hal/checklist.md)
        - [Именование](./design-patterns/hal/naming.md)
        - [Совместимость](./design-patterns/hal/interoperability.md)
        - [Предсказуемость](./design-patterns/hal/predictability.md)
        - [GPIO](./design-patterns/hal/gpio.md)
- [Советы для разработчиков встраиваемого C](./c-tips/index.md)
    <!-- TODO: определить разделы -->
- [Взаимодействие](./interoperability/index.md)
    - [Немного C в вашем Rust](./interoperability/c-with-rust.md)
    - [Немного Rust в вашем C](./interoperability/rust-with-c.md)
- [Разное](./unsorted/index.md)
  - [Оптимизации: компромисс между скоростью и размером](./unsorted/speed-vs-size.md)
  - [Математические вычисления](./unsorted/math.md)

---

[Приложение A: Глоссарий](./appendix/glossary.md)
