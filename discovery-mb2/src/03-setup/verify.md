# Проверка установки

Давайте проверим, что все инструменты установлены правильно.

## Проверка cargo-embed

Для начала подключите micro:bit к компьютеру с помощью USB-кабеля.

Как минимум должен загореться оранжевый светодиод рядом с USB-разъёмом micro:bit. Кроме того, если вы ещё не прошивали micro:bit другой программой, стандартная программа, которая на нём установлена, должна начать мигать красными светодиодами на задней стороне: их можно не замечать или поиграть с демонстрационным приложением.

Теперь проверим, видит ли probe-rs, а значит и cargo-embed, ваш micro:bit. Для этого выполните команду:

``` console
$ probe-rs list
The following debug probes were found:
[0]: BBC micro:bit CMSIS-DAP -- 0d28:0204:990636020005282030f57fa14252d446000000006e052820 (CMSIS-DAP)
```

Если же хотите узнать больше о возможностях отладки micro:bit, выполните:

``` console
$ probe-rs info
Probing target via JTAG

Error identifying target using protocol JTAG: The probe does not support the JTAG protocol.

Probing target via SWD

Arm Chip with debug port Default:
Debug Port: DPv1, DP Designer: Arm Ltd
├── 0 MemoryAP
│   └── ROM Table (Class 1), Designer: Nordic VLSI ASA
│       ├── Cortex-M4 SCS   (Generic IP component)
│       │   └── CPUID
│       │       ├── IMPLEMENTER: Arm Ltd
│       │       ├── VARIANT: 0
│       │       ├── PARTNO: Cortex-M4
│       │       └── REVISION: 1
│       ├── Cortex-M3 DWT   (Generic IP component)
│       ├── Cortex-M3 FBP   (Generic IP component)
│       ├── Cortex-M3 ITM   (Generic IP component)
│       ├── Cortex-M4 TPIU  (Coresight Component)
│       └── Cortex-M4 ETM   (Coresight Component)
└── 1 Unknown AP (Designer: Nordic VLSI ASA, Class: Undefined, Type: 0x0, Variant: 0x0, Revision: 0x0)


Debugging RISC-V targets over SWD is not supported. For these targets, JTAG is the only supported protocol. RISC-V specific information cannot be printed.
Debugging Xtensa targets over SWD is not supported. For these targets, JTAG is the only supported protocol. Xtensa specific information cannot be printed.
```

Затем убедитесь, что вы находитесь в каталоге `src/03-setup` исходного кода книги, и выполните команды:

```
$ rustup target add thumbv7em-none-eabihf
$ cargo embed --target thumbv7em-none-eabihf
```

Если всё работает правильно, cargo-embed сначала соберёт небольшую программу-пример из этого каталога, затем прошьёт её и, наконец, откроет текстовый интерфейс, который выводит «Hello World».

(Если этого не произошло, обратитесь к [общим инструкциям по устранению неполадок](../appendix/1-general-troubleshooting/index.html).)

Этот вывод выдаёт небольшая программа на Rust, которую вы только что прошили на micro:bit. Всё работает как надо, и можно переходить к следующим главам!
