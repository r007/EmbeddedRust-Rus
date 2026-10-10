# Windows

## `arm-none-eabi-gdb`

ARM предоставляет установщики `.exe` для Windows. Скачайте один из них [здесь][gcc] и следуйте инструкциям.
Перед самым завершением установки отметьте опцию «Add path to environment variable» (добавить путь в переменную окружения). Затем убедитесь, что инструменты доступны в вашем `%PATH%`:

``` text
$ arm-none-eabi-gdb -v
GNU gdb (GNU Tools for Arm Embedded Processors 7-2018-q2-update) 8.1.0.20180315-git
(..)
```

[gcc]: https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads

## OpenOCD

Официального бинарного релиза OpenOCD для Windows нет. Но если вам не хочется собирать его самостоятельно, проект xPack предоставляет готовую бинарную сборку, [здесь][openocd]. Следуйте приведённым там инструкциям по установке. Затем обновите переменную окружения `%PATH%`, добавив в неё путь, по которому установлены исполняемые файлы (`C:\Users\USERNAME\AppData\Roaming\xPacks\@xpack-dev-tools\openocd\0.10.0-13.1\.content\bin\`, если вы использовали простую установку).

[openocd]: https://xpack.github.io/openocd/

Убедитесь, что OpenOCD доступен в вашем `%PATH%`:

``` text
$ openocd -v
Open On-Chip Debugger 0.10.0
(..)
```

## QEMU

Скачайте QEMU [с официального сайта][qemu].

[qemu]: https://www.qemu.org/download/#windows

## Драйвер USB для ST-LINK

Также вам нужно установить [этот USB-драйвер], иначе OpenOCD не будет работать. Следуйте инструкциям установщика и убедитесь, что вы выбрали подходящую версию драйвера (32-битную или 64-битную).

[этот USB-драйвер]: http://www.st.com/en/embedded-software/stsw-link009.html

Всё! Перейдите к [следующему разделу].

[следующему разделу]: verify.md
