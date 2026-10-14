{{#title Атрибут no_std в embedded Rust для Raspberry Pi Pico 2 | impl Rust для RP2350}}

# no_std

В Rust есть два основных фундаментальных крейта: std и core.

- Крейт std — это стандартная библиотека. Она даёт такие возможности, как выделение памяти в куче, доступ к файловой системе, потоки и `println!()`.

- Крейт core — минимальное подмножество. Он содержит только самые необходимые возможности Rust: базовые типы (Option, Result и т. п.), трейты и некоторые другие операции. Он не зависит от операционной системы или среды выполнения.

Если сейчас попробовать собрать проект, вы получите целую кучу ошибок. Вот как это выглядит:

```text
error[E0463]: can't find crate for `std`
  |
  = note: the `thumbv8m.main-none-eabihf` target may not support the standard library
  = note: `std` is required by `pico_from_scratch` because it does not declare `#![no_std]`

error: cannot resolve a prelude import

error: cannot find macro `println` in this scope
 --> src/main.rs:2:5
  |
2 |     println!("Hello, world!");
  |     ^^^^^^^

error: `#[panic_handler]` function required, but not found

For more information about this error, try `rustc --explain E0463`.
error: could not compile `pico-from-scratch` (bin "pico-from-scratch") due to 4 previous errors
```

Ошибок здесь очень много. Давайте исправим их по одной. Первая ошибка говорит, что целевая платформа, возможно, не поддерживает стандартную библиотеку. Это верно, мы уже это знаем. Проблема в том, что мы не сообщили Rust, что не хотим использовать `std`. Именно для этого служит атрибут `no_std`.

## #![no_std]

Атрибут `#![no_std]` отключает использование стандартной библиотеки (`std`). Это почти всегда необходимо при разработке встраиваемых систем, где среде обычно не хватает многих ресурсов (например, операционной системы, файловой системы или выделения памяти в куче), которые стандартная библиотека предполагает доступными.

В начало файла `src/main.rs` добавьте эту строку:

```rs
#![no_std]
```

На этом всё. Теперь Rust знает, что проект будет использовать только библиотеку `core`, а не `std`.

## Println

Макрос `println!` приходит из [крейта std](https://doc.rust-lang.org/std/macro.println.html). Поскольку в нашем проекте std не используется, `println!` мы использовать не можем. Давайте удалим его из кода.

Теперь код должен выглядеть так:

```rust
#![no_std]

fn main() {

}
```

С этим исправлением мы устранили три ошибки и сократили список. Осталась ещё одна проблема, которую мы исправим в следующем разделе.

**Ресурсы:**

- [Официальная документация Rust, раздел о прелюдиях и no_std](https://doc.rust-lang.org/reference/names/preludes.html#the-no_std-attribute) (на английском языке)
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/intro/no-std.html) (на английском языке)
- [Writing an OS in Rust, раздел об атрибуте no_std](https://os.phil-opp.com/freestanding-rust-binary/#the-no-std-attribute) (на английском языке)
