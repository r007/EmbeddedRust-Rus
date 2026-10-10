# Windows

## `arm-none-eabi-gdb`

Arm предоставляет установщики `.exe` для Windows. Скачайте нужный [здесь][gcc] и следуйте инструкциям. Незадолго до завершения установки отметьте пункт «Add path to environment variable» (добавить путь в переменную окружения). Затем проверьте, что инструменты есть в вашем `%PATH%`:

``` console
$ arm-none-eabi-gcc -v
(..)
gcc version 5.4.1 20160919 (release) (..)
```

[gcc]: https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads

## PuTTY

Скачайте последнюю версию `putty.exe` с [этого сайта](http://www.chiark.greenend.org.uk/~sgtatham/putty/download.html) и поместите его в папку, которая есть в вашем `%PATH%`.

Теперь перейдите к [следующему разделу](verify.md).
