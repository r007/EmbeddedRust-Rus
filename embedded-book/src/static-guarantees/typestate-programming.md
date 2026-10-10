# Программирование на typestate

Концепция [typestates] описывает кодирование информации о текущем состоянии объекта в типе этого объекта. Хотя это может звучать немного заумно, если вы использовали [паттерн Builder](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html) в Rust, то вы уже начали использовать программирование на typestate!

[typestates]: https://en.wikipedia.org/wiki/Typestate_analysis
[Builder Pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html

```rust
pub mod foo_module {
    #[derive(Debug)]
    pub struct Foo {
        inner: u32,
    }

    pub struct FooBuilder {
        a: u32,
        b: u32,
    }

    impl FooBuilder {
        pub fn new(starter: u32) -> Self {
            Self {
                a: starter,
                b: starter,
            }
        }

        pub fn double_a(self) -> Self {
            Self {
                a: self.a * 2,
                b: self.b,
            }
        }

        pub fn into_foo(self) -> Foo {
            Foo {
                inner: self.a + self.b,
            }
        }
    }
}

fn main() {
    let x = foo_module::FooBuilder::new(10)
        .double_a()
        .into_foo();

    println!("{:#?}", x);
}
```

В этом примере нет прямого способа создать объект `Foo`. Нужно создать `FooBuilder` и правильно инициализировать его, прежде чем мы сможем получить нужный объект `Foo`.

Этот минимальный пример кодирует два состояния:

* `FooBuilder`, которое представляет состояние «не настроено» или «настройка в процессе»;
* `Foo`, которое представляет состояние «настроено» или «готово к использованию».

## Строгие типы

Поскольку в Rust есть [строгая система типов][Strong Type System], нет простого способа волшебным образом создать экземпляр `Foo` или превратить `FooBuilder` в `Foo`, не вызвав метод `into_foo()`. Кроме того, вызов метода `into_foo()` поглощает исходную структуру `FooBuilder`, а значит, её нельзя переиспользовать без создания нового экземпляра.

[Strong Type System]: https://en.wikipedia.org/wiki/Strong_and_weak_typing

Это позволяет представлять состояния нашей системы в виде типов и включать необходимые действия для переходов между состояниями в методы, которые обменивают один тип на другой. Создав `FooBuilder` и обменяв его на объект `Foo`, мы прошли все шаги простейшего конечного автомата.
