# Приложение A: Глоссарий

Экосистема встраиваемых систем полна разных протоколов, аппаратных компонентов и специфичных для производителей вещей, которые используют собственные термины и сокращения. Этот глоссарий пытается перечислить их и даёт ссылки для лучшего понимания.

### BSP

Board Support Crate (BSP) предоставляет высокоуровневый интерфейс, настроенный для конкретной платы. Обычно он зависит от крейта [HAL](#hal).
Более подробное описание есть на [странице про регистры, отображённые в память](../start/registers.md), а для общего обзора см. [это видео](https://youtu.be/vLYit_HHPaY).

### FPU

Floating-point Unit (блок вычислений с плавающей точкой). «Математический процессор», который выполняет только операции с числами с плавающей точкой.

### HAL

Hardware Abstraction Layer crate (крейт слоя аппаратной абстракции) предоставляет удобный для разработчика интерфейс к возможностям и периферии микроконтроллера. Обычно он реализуется поверх [Peripheral Access Crate (PAC)](#pac). Также он может реализовывать трейты из крейта [`embedded-hal`](https://crates.io/crates/embedded-hal).
Более подробное описание есть на [странице про регистры, отображённые в память](../start/registers.md), а для общего обзора см. [это видео](https://youtu.be/vLYit_HHPaY).

### I2C

Иногда обозначается как `I²C` или Inter-IC. Это протокол для обмена данными между микросхемами на одной плате. Подробнее см. [здесь][i2c].

[i2c]: https://en.wikipedia.org/wiki/I2c

### PAC

Peripheral Access Crate (крейт доступа к периферии) предоставляет доступ к периферии микроконтроллера. Это один из крейтов самого низкого уровня, обычно он генерируется непосредственно из предоставленного файла [SVD](#svd), часто с помощью [svd2rust](https://github.com/rust-embedded/svd2rust/). [Слой аппаратной абстракции](#hal) обычно зависит от этого крейта.
Более подробное описание есть на [странице про регистры, отображённые в память](../start/registers.md), а для общего обзора см. [это видео](https://youtu.be/vLYit_HHPaY).

### SPI

Serial Peripheral Interface (последовательный периферийный интерфейс). Подробнее см. [здесь][spi].

[spi]: https://en.wikipedia.org/wiki/Serial_peripheral_interface

### SVD

System View Description — формат файла на основе XML, который описывает представление программиста о устройстве микроконтроллера. Подробнее о нём можно прочитать на [сайте документации ARM CMSIS](https://www.keil.com/pack/doc/CMSIS/SVD/html/index.html).

### UART

Universal asynchronous receiver-transmitter (универсальный асинхронный приёмопередатчик). Подробнее см. [здесь][uart].

[uart]: https://en.wikipedia.org/wiki/Universal_asynchronous_receiver-transmitter

### USART

Universal synchronous and asynchronous receiver-transmitter (универсальный синхронный и асинхронный приёмопередатчик). Подробнее см. [здесь][usart].

[usart]: https://en.wikipedia.org/wiki/Universal_synchronous_and_asynchronous_receiver-transmitter
