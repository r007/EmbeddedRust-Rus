# Проверка оборудования

Подключите Espressif Rust Board к компьютеру. Убедитесь, что загорается маленький красный контрольный светодиод.

Плата также должна предоставлять UART-порт через USB:

**Windows**: USB Serial Device (COM-порт) в Диспетчере устройств в разделе «Порты (COM и LPT)».

**Linux**: USB-устройство в выводе `lsusb`.
У устройства будет VID (Vendor ID) `303a` и PID (Product ID) `1001`. Префикс `0x` в выводе `lsusb` не указывается:

``` console
$ lsusb | grep USB
Bus 006 Device 035: ID 303a:1001 Espressif USB JTAG/serial debug unit
```

Другой способ увидеть, какой порт и какие права назначены устройству, — проверить папку `/by-id`:
``` console
$ ls -l /dev/serial/by-id
lrwxrwxrwx 1 root root .... usb-Espressif_USB_JTAG_serial_debug_unit_60:55:F9:C0:27:18-if00 -> ../../ttyACM0

```
> Если вы используете ESP32-C3-DevKitC-02, команда будет `$ ls /dev/ttyUSB*`

**macOS**: Устройство отображается как часть дерева USB в `system_profiler`:

```console
$ system_profiler SPUSBDataType | grep -A 11 "USB JTAG"

USB JTAG/serial debug unit:

  Product ID: 0x1001
  Vendor ID: 0x303a
  (...)
```

Устройство также появится в каталоге `/dev` как `tty.usbmodem`:

``` console
$ ls /dev/tty.usbmodem*
/dev/tty.usbmodem0
```
