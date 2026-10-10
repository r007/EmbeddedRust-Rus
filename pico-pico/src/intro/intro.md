# Введение

Если вы ещё не читали «The Embedded Rust Book», очень рекомендую заглянуть в неё.
[https://docs.rust-embedded.org/book/intro/index.html](https://docs.rust-embedded.org/book/intro/index.html)

## #![no_std]
Атрибут `#![no_std]` отключает использование стандартной библиотеки (std). Во встраиваемых системах это почти всегда необходимо, потому что окружению обычно не хватает многих ресурсов (операционной системы, файловой системы или выделения памяти в куче), которые стандартная библиотека считает доступными.

**Связанные ресурсы:**
- [Официальная документация Rust](https://doc.rust-lang.org/reference/names/preludes.html#the-no_std-attribute)
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/intro/no-std.html)
- [Раздел из книги «Writing an OS in Rust»](https://os.phil-opp.com/freestanding-rust-binary/#the-no-std-attribute)

## #![no_main]
Атрибут `#![no_main]` указывает, что программа не будет использовать стандартную точку входа (fn main). Вместо этого предоставляется собственная точку входа. Это обычно требуется во встраиваемых системах, где среда выполнения минимальна или отсутствует вовсе.

**Связанные ресурсы:**
- [Официальная документация Rust](https://doc.rust-lang.org/reference/crates-and-source-files.html?highlight=no_main#the-no_main-attribute)
- [Раздел из книги «Writing an OS in Rust»](https://os.phil-opp.com/freestanding-rust-binary/#overwriting-the-entry-point)
