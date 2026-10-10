# Установка инструментов

На этой странице приведены не зависящие от ОС инструкции по установке некоторых инструментов:

### Тулчейн Rust

Установите rustup, следуя инструкциям на [https://rustup.rs](https://rustup.rs).

**ПРИМЕЧАНИЕ** Убедитесь, что версия компилятора не ниже `1.31`. Команда `rustc -V` должна выводить дату новее указанной ниже.

``` text
$ rustc -V
rustc 1.31.1 (b6c32da9b 2018-12-18)
```

Для экономии трафика и места на диске стандартная установка поддерживает только нативную компиляцию. Чтобы добавить поддержку кросс-компиляции для архитектур ARM Cortex-M, выберите один из следующих таргетов (compilation targets). Для платы STM32F3DISCOVERY, которую мы используем в примерах этой книги, нужен таргет `thumbv7em-none-eabihf`.
[Выберите подходящий для вас Cortex-M.](https://developer.arm.com/ip-products/processors/cortex-m#c-7d3b69ce-5b17-4c9e-8f06-59b605713133)

Cortex-M0, M0+ и M1 (архитектура ARMv6-M):
``` console
rustup target add thumbv6m-none-eabi
```

Cortex-M3 (архитектура ARMv7-M):
``` console
rustup target add thumbv7m-none-eabi
```

Cortex-M4 и M7 без аппаратной поддержки операций с плавающей точкой (архитектура ARMv7E-M):
``` console
rustup target add thumbv7em-none-eabi
```

Cortex-M4F и M7F с аппаратной поддержкой операций с плавающей точкой (архитектура ARMv7E-M):
``` console
rustup target add thumbv7em-none-eabihf
```

Cortex-M23 (архитектура ARMv8-M):
``` console
rustup target add thumbv8m.base-none-eabi
```

Cortex-M33 и M35P (архитектура ARMv8-M):
``` console
rustup target add thumbv8m.main-none-eabi
```

Cortex-M33F и M35PF с аппаратной поддержкой операций с плавающей точкой (архитектура ARMv8-M):
``` console
rustup target add thumbv8m.main-none-eabihf
```


### `cargo-binutils`

``` text
cargo install cargo-binutils

rustup component add llvm-tools
```
WINDOWS: предполагается, что установлены C++ Build Tools для Visual Studio 2019. https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16
### `cargo-generate`

Мы воспользуемся этим инструментом позже, чтобы создать проект из шаблона.

``` console
cargo install cargo-generate
```

Примечание: в некоторых дистрибутивах Linux (например, Ubuntu) перед установкой cargo-generate может понадобиться установить пакеты `libssl-dev` и `pkg-config`.

### Инструкции для конкретной ОС

Теперь следуйте инструкциям для вашей операционной системы:

- [Linux](install/linux.md)
- [Windows](install/windows.md)
- [macOS](install/macos.md)
