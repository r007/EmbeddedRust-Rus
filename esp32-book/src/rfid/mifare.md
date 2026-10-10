{{#title Структура памяти MIFARE Classic 1K для ESP32}}

# MIFARE

MIFARE — это серия микросхем (IC), используемых в бесконтактных смарт-картах и карточках доступа. Её разработала компания NXP Semiconductors. Карты MIFARE соответствуют стандарту ISO/IEC 14443A и используют методы шифрования, например алгоритм Crypto-1. Самое распространённое семейство — MIFARE Classic, у него есть подтип [MIFARE Classic EV1](https://www.nxp.com/products/rfid-nfc/mifare-hf/mifare-classic/mifare-classic-ev1-1k-4k:MF1S50YYX_V1).

## Структура памяти
Карта MIFARE Classic 1K разделена на 16 секторов, и каждый сектор содержит 4 блока. Каждый блок может хранить до 16 байт, поэтому общий объём памяти составляет 1 КБ.

16 секторов × 4 блока/сектор × 16 байт/блок = 1024 байта = 1 КБ

<a href="./images/mifare-memory-layout.png"><img style="display: block; margin: auto;" alt="Структура памяти MIFARE" src="./images/mifare-memory-layout.png"/></a>

### Трейлер сектора

Последний блок каждого сектора, известный как «трейлер», хранит два секретных ключа и программируемые условия доступа для блоков этого сектора. У каждого сектора есть собственная пара ключей (KeyA и KeyB), что позволяет поддерживать несколько приложений с иерархией ключей.

> [!Note]
> Карта MIFARE Classic 1K по умолчанию настроена на ключ FF FF FF FF FF FF для KeyA и KeyB. При чтении блока трейлера значение KeyA возвращается как нулевое (00 00 00 00 00 00), а KeyB возвращается как есть.


По умолчанию байты управления доступом (6, 7 и 8 трейлера) установлены в FF 07 80h. Подробнее см. 10-ю страницу [даташита](https://www.nxp.com/docs/en/data-sheet/MF1S50YYX_V1.pdf). А 9-й байт можно использовать для хранения данных.

<table border="1" cellspacing="0" cellpadding="5" style="border-collapse: collapse; width: 100%; text-align: center;">
  <thead>
    <tr>
      <th rowspan="2" style="background-color: #607D8B; color: #000;">Номер байта</th>
    </tr>
    <tr>
      <th style="background-color: #1B5E20;">0</th>
      <th style="background-color: #1B5E20;">1</th>
      <th style="background-color: #1B5E20;">2</th>
      <th style="background-color: #1B5E20;">3</th>
      <th style="background-color: #1B5E20;">4</th>
      <th style="background-color: #1B5E20;">5</th>
      <th style="background-color: #FF6F00;">6</th>
      <th style="background-color: #FF6F00;">7</th>
      <th style="background-color: #FF6F00;">8</th>
      <th style="background-color: #FF6F00;">9</th>
      <th style="background-color: #2962FF;">10</th>
      <th style="background-color: #2962FF;">11</th>
      <th style="background-color: #2962FF;">12</th>
      <th style="background-color: #2962FF;">13</th>
      <th style="background-color: #2962FF;">14</th>
      <th style="background-color: #2962FF;">15</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td style="background-color: #607D8B; color: #000;">Описание</td>
      <td colspan="6" style="background-color: #1B5E20; color: #000;">KEY A</td>
      <td colspan="3" style="background-color: #FF6F00; color: #000;">Биты доступа</td>
      <td style="background-color: #FF6F00; color: #000;">Пользовательские данные</td>
      <td colspan="6" style="background-color: #2962FF; color: #000;">KEY B</td>
    </tr>
    <tr>
      <td style="background-color: #607D8B; color: #000;">Значения по умолчанию</td>
      <td style="background-color: #1B5E20;">FF</td>
      <td style="background-color: #1B5E20;">FF</td>
      <td style="background-color: #1B5E20;">FF</td>
      <td style="background-color: #1B5E20;">FF</td>
      <td style="background-color: #1B5E20;">FF</td>
      <td style="background-color: #1B5E20;">FF</td>
      <td style="background-color: #FF6F00;">FF</td>
      <td style="background-color: #FF6F00;">07</td>
      <td style="background-color: #FF6F00;">80</td>
      <td style="background-color: #FF6F00;">69</td>
      <td style="background-color: #2962FF;">FF</td>
      <td style="background-color: #2962FF;">FF</td>
      <td style="background-color: #2962FF;">FF</td>
      <td style="background-color: #2962FF;">FF</td>
      <td style="background-color: #2962FF;">FF</td>
      <td style="background-color: #2962FF;">FF</td>
    </tr>
  </tbody>
</table>



### Блок производителя

Первый блок (блок 0) первого сектора (сектор 0) содержит данные производителя микросхемы, включая UID. Этот блок защищён от записи.

### Блок данных

У каждого сектора есть блок трейлера, поэтому в каждом секторе для хранения данных можно использовать только 3 блока. Однако в первом секторе доступно только 2 блока, потому что первый блок хранит данные производителя.

Чтобы читать или записывать данные, сначала нужно пройти аутентификацию с помощью KeyA или KeyB этого сектора.

Блоки данных можно разделить на две категории по битам доступа (о них расскажем позже):
- блок чтения/записи: обычные блоки данных, которые позволяют выполнять базовые операции чтения и записи;
- блок значения (value block): такие блоки подходят для приложений вроде электронных кошельков, где их обычно используют для хранения числовых значений, например баланса счёта. Поэтому можно выполнять инкремент (например, добавить 10 к балансу) или декремент (например, списать 5 за покупку).

## Ссылки
- Даташит: [MIFARE Classic EV1 1K — массовая бесконтактная смарт-карта для быстрой и простой разработки решений](https://www.nxp.com/docs/en/data-sheet/MF1S50YYX_V1.pdf)
