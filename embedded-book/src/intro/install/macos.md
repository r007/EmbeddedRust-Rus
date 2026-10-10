# macOS

Все инструменты можно установить с помощью [Homebrew] или [MacPorts]:

[Homebrew]: http://brew.sh/
[MacPorts]: https://www.macports.org/

## Установка инструментов через [Homebrew]

``` text
$ # GDB
$ brew install arm-none-eabi-gdb

$ # OpenOCD
$ brew install openocd

$ # QEMU
$ brew install qemu
```

> **ПРИМЕЧАНИЕ** Если OpenOCD аварийно завершается, может понадобиться установить последнюю версию командой:
```text
$ brew install --HEAD openocd
```

## Установка инструментов через [MacPorts]

``` text
$ # GDB
$ sudo port install arm-none-eabi-gcc

$ # OpenOCD
$ sudo port install openocd

$ # QEMU
$ sudo port install qemu
```



Всё! Перейдите к [следующему разделу].

[следующему разделу]: verify.md
