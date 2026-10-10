# Переворот строки

Ну что ж, сделаем сервер поинтереснее: пусть он отвечает клиенту перевёрнутым текстом, который тот прислал. Сервер будет отвечать каждый раз, когда клиент нажмёт клавишу ENTER. Каждый ответ сервера будет выводиться с новой строки.

На этот раз понадобится буфер; можно использовать [`heapless::Vec`]. Вот стартовый код:

[`heapless::Vec`]: https://docs.rs/heapless/latest/heapless/struct.Vec.html

``` rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use core::fmt::Write;
use heapless::Vec;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;

use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

mod serial_setup;
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // Буфер ёмкостью 32 байта
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        // TODO Принять запрос пользователя. Каждый запрос заканчивается нажатием ENTER
        // NOTE `buffer.push` возвращает `Result`. Обработайте ошибку, ответив
        // сообщением об ошибке.

        // TODO Отправить обратно строку в обратном порядке
    }
}
```
