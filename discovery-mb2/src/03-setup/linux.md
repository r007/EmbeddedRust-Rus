# Linux

Ниже приведены команды установки для нескольких дистрибутивов Linux.

## Ubuntu 20.04 или новее / Debian 10 или новее

> **ПРИМЕЧАНИЕ** `gdb-multiarch` — это команда GDB, которой вы будете отлаживать программы для Arm Cortex-M.
``` console
$ sudo apt install gdb-multiarch minicom libunwind-dev
```

## Fedora 32 или новее

> **ПРИМЕЧАНИЕ** `gdb` — это команда GDB, которой вы будете отлаживать программы для Arm
> Cortex-M.
``` console
$ sudo dnf install gdb minicom libunwind-devel
```

## Arch Linux

> **ПРИМЕЧАНИЕ** `gdb` — это команда GDB, которой вы будете отлаживать программы для Arm
> Cortex-M.
``` console
$ sudo pacman -S arm-none-eabi-gdb minicom libunwind
```

## Другие дистрибутивы

> **ПРИМЕЧАНИЕ** `arm-none-eabi-gdb` — это команда GDB, которой вы будете отлаживать программы для Arm Cortex-M.

Если для вашего дистрибутива нет пакетов [готового набора инструментов Arm](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads), скачайте файл «Linux 64-bit» и добавьте его каталог `bin` в `PATH`. Вот один из способов:

``` console
$ mkdir -p ~/local
$ cd ~/local
$ tar xjf /path/to/downloaded/XXX.tar.bz2
```

Затем с помощью любимого редактора добавьте строку в `PATH` в подходящий файл инициализации оболочки (например, `~/.zshrc` или `~/.bashrc`):

```
PATH=$PATH:$HOME/local/XXX/bin
```

## Правила udev

Эти правила позволяют работать с USB-устройствами, например с micro:bit, без привилегий root, то есть без `sudo`.

Создайте этот файл в `/etc/udev/rules.d` с содержимым, показанным ниже.

``` console
$ cat /etc/udev/rules.d/69-microbit.rules
```

``` text
# CMSIS-DAP для micro:bit
ACTION!="add|change", GOTO="microbit_rules_end"
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", TAG+="uaccess"
LABEL="microbit_rules_end"
```

Затем перезагрузите правила udev командой:

``` console
$ sudo udevadm control --reload
```

Если к компьютеру были подключены платы, отключите их и подключите снова или выполните следующую команду.

``` console
$ sudo udevadm trigger
```

## Проверка прав доступа

Подключите micro:bit к компьютеру с помощью USB-кабеля.

Теперь micro:bit должен появиться как USB-устройство (файл) в `/dev/bus/usb`. Узнаем, под каким номером его зарегистрировала система:

``` console
$ lsusb | grep -i "NXP Arm mbed"
Bus 001 Device 065: ID 0d28:0204 NXP Arm mbed
$ # ^^^        ^^^
```

В моём случае micro:bit подключился к шине №1 и был зарегистрирован как устройство №65. Это значит, что файл `/dev/bus/usb/001/065` *и есть* micro:bit. Проверим права доступа к файлу:

``` console
$ ls -l /dev/bus/usb/001/065
crw-rw-r--+ 1 nobody nobody 189, 64 Sep  5 14:27 /dev/bus/usb/001/065
```

Права должны быть `crw-rw-r--+`, обратите внимание на `+` в конце. Затем посмотрите свои права доступа, выполнив следующую команду:

``` console
$ getfacl /dev/bus/usb/001/065
getfacl: Removing leadin '/' from absolute path names
# file: dev/bus/usb/001/065
# owner: nobody
# group: nobody
user::rw-
user:<YOUR-USER-NAME>:rw-
group::rw-
mask::rw-
other::r-
```

В списке выше должно быть ваше имя пользователя с правами `rw-`. Если его нет, проверьте свои [правила udev] и попробуйте перезагрузить их:

[правила udev]: linux.md#правила-udev

``` console
$ sudo udevadm control --reload
$ sudo udevadm trigger
```

Теперь перейдите к [следующему разделу](verify.md).
