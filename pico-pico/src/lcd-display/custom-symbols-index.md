# Индекс символов

Ниже приведён список пользовательских символов с соответствующими массивами байтов. Если вы придумали интересный символ и хотите добавить его в этот список, отправляйте pull request. Пожалуйста, используйте [генератор пользовательских символов](./lcd-custom-char-generator.md), чтобы результаты были единообразными.

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
            <td><img class="preview-symbol" src="./images/custom-chars/heart.png" alt="Сердце"></td>
            <td>[ 0b00000, 0b01010, 0b11111, 0b11111, 0b01110, 0b00100, 0b00000, 0b00000,]</td>
        </tr>
        <tr>
            <td>Замок</td>
            <td><img class="preview-symbol" src="./images/custom-chars/lock.png" alt="Замок"></td>
            <td>[ 0b01110, 0b10001, 0b10001, 0b11111, 0b11011, 0b11011, 0b11011, 0b11111, ]</td>
        </tr>
        <tr>
            <td>Контурное сердце</td>
            <td><img class="preview-symbol" src="./images/custom-chars/hollow-heart.png" alt="Контурное сердце"></td>
            <td>[ 0b00000, 0b01010, 0b10101, 0b10001, 0b10001, 0b01010, 0b00100, 0b00000, ]</td>
        </tr>
        <tr>
            <td>Батарея</td>
            <td><img class="preview-symbol" src="./images/custom-chars/battery.png" alt="Батарея"></td>
            <td>[ 0b01110, 0b11011, 0b10001, 0b10001, 0b10001, 0b11111, 0b11111, 0b11111, ]</td>
        </tr>
        <tr>
            <td>Автобус</td>
            <td><img class="preview-symbol" src="./images/custom-chars/bus.png" alt="Автобус"></td>
            <td>[ 0b01110, 0b11111, 0b10001, 0b10001, 0b11111, 0b10101, 0b11111, 0b01010, ]</td>
        </tr>
        <tr>
            <td>Колокольчик</td>
            <td><img class="preview-symbol" src="./images/custom-chars/bell.png" alt="Колокольчик"></td>
            <td>[ 0b00100, 0b01110, 0b01110, 0b01110, 0b11111, 0b00000, 0b00100, 0b00000, ]</td>
        </tr>
        <tr>
            <td>Песочные часы</td>
            <td><img class="preview-symbol" src="./images/custom-chars/hour-glass.png" alt="Песочные часы"></td>
            <td>[ 0b00000, 0b11111, 0b10001, 0b01010, 0b00100, 0b01010, 0b10101, 0b11111, ]</td>
        </tr>
        <tr>
            <td>Зарядка</td>
            <td><img class="preview-symbol" src="./images/custom-chars/charger.png" alt="Зарядка"></td>
            <td>[ 0b01010, 0b01010, 0b11111, 0b10001, 0b10001, 0b01110, 0b00100, 0b00100, ]</td>
        </tr>
        <tr>
            <td>Галочка</td>
            <td><img class="preview-symbol" src="./images/custom-chars/tick-mark.png" alt="Галочка"></td>
            <td>[ 0b00000, 0b00000, 0b00001, 0b00011, 0b10110, 0b11100, 0b01000, 0b00000, ]</td>
        </tr>
        <tr>
            <td>Нота</td>
            <td><img class="preview-symbol" src="./images/custom-chars/music-note.png" alt="Нота"></td>
            <td>[ 0b00011, 0b00010, 0b00010, 0b00010, 0b00010, 0b01110, 0b11110, 0b01110, ]</td>
        </tr>
    </tbody>
</table>
