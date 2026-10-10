{{#title Туториал по no_std в Rust для ESP32 и встраиваемых систем}}

# no_std

В Rust есть две основные базовые библиотеки: std и core.

- Крейт std — это стандартная библиотека. Она предоставляет такие возможности, как выделение памяти в куче, доступ к файловой системе, потоки и `println!`.

- Крейт core — это минимальное подмножество. Он содержит только самые необходимые возможности Rust: базовые типы (Option, Result и др.), трейты и некоторые другие операции. Он не зависит от операционной системы или рантайма.


Если вы попытаетесь собрать проект на этом этапе, то получите целую кучу ошибок. Вот как это выглядит:

```sh
error[E0463]: can't find crate for `std`
  |
  = note: the `xtensa-esp32-none-elf` target may not support the standard library
  = note: `std` is required by `std_to_no_std` because it does not declare `#![no_std]`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`

error: cannot find macro `println` in this scope
 --> src/main.rs:2:5
  |
2 |     println!("Hello, world!");
  |     ^^^^^^^

error: `#[panic_handler]` function required, but not found

For more information about this error, try `rustc --explain E0463`.
```

Ошибок здесь очень много. Давайте исправлять их по одной. Первая ошибка говорит, что целевая платформа может не поддерживать стандартную библиотеку. Это правда, мы уже это знаем. Проблема в том, что мы не сообщили Rust, что не хотим использовать std. Для этого и нужен атрибут `no_std`.


## #![no_std]

Атрибут `#![no_std]` отключает использование стандартной библиотеки (std). Это почти всегда необходимо при разработке встраиваемых систем, где окружение обычно не предоставляет многих ресурсов (операционной системы, файловой системы или выделения памяти в куче), на которые рассчитывает стандартная библиотека.

В самом начале файла `src/main.rs` добавьте эту строку:

```rs
#![no_std]
```

Вот и всё. Теперь Rust знает, что проект будет использовать только core, а не std.

## Println

Макрос `println!` входит в [крейт std](https://doc.rust-lang.org/std/macro.println.html). Поскольку в нашем проекте std не используется, `println!` недоступен. Давайте уберём его из кода.

Теперь код должен выглядеть так:

```rust
#![no_std]
fn main() {
    
}
```

Этим исправлением мы устранили две ошибки и сократили список. Осталась ещё одна проблема, которую исправим в следующем разделе.


**Ресурсы:**
- [Официальная документация Rust](https://doc.rust-lang.org/reference/names/preludes.html#the-no_std-attribute)
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/intro/no-std.html)
- [Writing an OS in Rust Book](https://os.phil-opp.com/freestanding-rust-binary/#the-no-std-attribute)
