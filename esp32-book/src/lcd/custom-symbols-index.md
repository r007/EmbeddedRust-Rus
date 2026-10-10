{{#title Библиотека пользовательских символов HD44780 для ESP32 и Rust}}

# Индекс символов

Здесь приведён список пользовательских символов с соответствующими байтовыми массивами. Если вы придумали интересный символ и хотите добавить его в этот список, присылайте pull request. Пожалуйста, используйте [генератор пользовательских символов](./lcd-custom-char-generator.md), который приведён здесь, чтобы сохранить единообразие.

<style>
    /* .preview-symbol{
        width:50px;
        height:80px;
    } */
    .preview-symbol {
        width: 50px;
        height: 80px;
        display: block;
        margin-left: auto;
        margin-right: auto;
        vertical-align: middle;

    }
</style>
<table>
    <thead>
        <tr>
            <th>Название</th>
            <th>Превью</th>
            <th>Массив байтов</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>Сердце</td>
            <td><img class="preview-symbol" src="./images/custom-chars/heart.png" alt="сердце"></td>
            <td>[ 0b00000, 0b01010, 0b11111, 0b11111, 0b01110, 0b00100, 0b00000, 0b00000,]</td>
        </tr>
        <tr>
            <td>Замок</td>
            <td><img class="preview-symbol" src="./images/custom-chars/lock.png" alt="замок"></td>
            <td>[ 0b01110, 0b10001, 0b10001, 0b11111, 0b11011, 0b11011, 0b11011, 0b11111, ]</td>
        </tr>
        <tr>
            <td>Пустое сердце</td>
            <td><img class="preview-symbol" src="./images/custom-chars/hollow-heart.png" alt="пустое сердце"></td>
            <td>[ 0b00000, 0b01010, 0b10101, 0b10001, 0b10001, 0b01010, 0b00100, 0b00000, ]</td>
        </tr>
        <tr>
            <td>Батарея</td>
            <td><img class="preview-symbol" src="./images/custom-chars/battery.png" alt="батарея"></td>
            <td>[ 0b01110, 0b11011, 0b10001, 0b10001, 0b10001, 0b11111, 0b11111, 0b11111, ]</td>
        </tr>
        <tr>
            <td>Шина</td>
            <td><img class="preview-symbol" src="./images/custom-chars/bus.png" alt="шина"></td>
            <td>[ 0b01110, 0b11111, 0b10001, 0b10001, 0b11111, 0b10101, 0b11111, 0b01010, ]</td>
        </tr>
        <tr>
            <td>Колокольчик</td>
            <td><img class="preview-symbol" src="./images/custom-chars/bell.png" alt="колокольчик"></td>
            <td>[ 0b00100, 0b01110, 0b01110, 0b01110, 0b11111, 0b00000, 0b00100, 0b00000, ]</td>
        </tr>
        <tr>
            <td>Песочные часы</td>
            <td><img class="preview-symbol" src="./images/custom-chars/hour-glass.png" alt="песочные часы"></td>
            <td>[ 0b00000, 0b11111, 0b10001, 0b01010, 0b00100, 0b01010, 0b10101, 0b11111, ]</td>
        </tr>
        <tr>
            <td>Зарядка</td>
            <td><img class="preview-symbol" src="./images/custom-chars/charger.png" alt="зарядка"></td>
            <td>[ 0b01010, 0b01010, 0b11111, 0b10001, 0b10001, 0b01110, 0b00100, 0b00100, ]</td>
        </tr>
        <tr>
            <td>Галочка</td>
            <td><img class="preview-symbol" src="./images/custom-chars/tick-mark.png" alt="галочка"></td>
            <td>[ 0b00000, 0b00000, 0b00001, 0b00011, 0b10110, 0b11100, 0b01000, 0b00000, ]</td>
        </tr>
        <tr>
            <td>Нота</td>
            <td><img class="preview-symbol" src="./images/custom-chars/music-note.png" alt="музыкальная нота"></td>
            <td>[ 0b00011, 0b00010, 0b00010, 0b00010, 0b00010, 0b01110, 0b11110, 0b01110, ]</td>
        </tr>
    </tbody>
</table>
