# Контроль доступа

Метка содержит биты доступа, которые управляют доступом к данным, хранящимся на метке. В этой главе мы разберём, как работают эти биты. Этот раздел может показаться немного сложным, поэтому я постараюсь объяснить его как можно проще.

> [!CAUTION]
> **Изменение битов доступа:** будьте осторожны при записи битов доступа, потому что неверные значения могут сделать сектор непригодным для использования.

## Разрешения

Это базовые разрешения, на основе которых задаются условия доступа. В таблице описана каждая операция с разрешением и указаны блоки, к которым она применима: обычные блоки данных (чтение/запись), блоки значений или служебные блоки секторов.

| **Операция** | **Описание**                                                          | **Применимо к типу блока**           |
|--------------|-----------------------------------------------------------------------|--------------------------------------|
| **Read**     | Чтение одного блока памяти                                            | Чтение/запись, значение, служебный блок |
| **Write**    | Запись одного блока памяти                                            | Чтение/запись, значение, служебный блок |
| **Increment**| Увеличение содержимого блока и сохранение результата во внутреннем буфере передачи | Значение                             |
| **Decrement**| Уменьшение содержимого блока и сохранение результата во внутреннем буфере передачи | Значение                             |
| **Restore**  | Чтение содержимого блока во внутренний буфер передачи                 | Значение                             |
| **Transfer** | Запись содержимого внутреннего буфера передачи в блок                 | Значение, чтение/запись              |

## Условия доступа

Начнём с главного вопроса: условия доступа. В ходе своих исследований я заметил, что многие люди с трудом разбираются в разделе об условиях доступа в даташите. Вот моя попытка объяснить его понятно 🤞.

Для управления разрешениями каждого блока используются всего 3 битовые комбинации. В официальном даташите это обозначается с помощью записи вида CX<sub>Y</sub> (C1₀, C1₂... C3₃) для битов доступа. Первое число (X) в этой записи — номер бита доступа, от 1 до 3, и каждый соответствует определённому типу разрешения. Однако смысл этих разрешений различается в зависимости от того, является ли блок блоком данных или служебным блоком. Второе число (Y) в нижнем индексе обозначает относительный номер блока, от 0 до 3.

### Таблица 1: Условия доступа для служебного блока сектора

В исходном даташите номер в нижнем индексе в таблице не указан. Я добавил индекс «3», так как служебный блок сектора находится в блоке 3.

> [!IMPORTANT]
> **Читаемый ключ:** если ключ можно прочитать, его нельзя использовать для аутентификации. Поэтому в этой таблице, когда ключ B читаем, он не может выполнять роль ключа аутентификации. Как вы могли заметить, да, ключ A никогда нельзя прочитать.

<table class="table-bordered">
  <thead>
    <tr >
      <th colspan="3" rowspan="2">Биты доступа</th>
      <th colspan="6">Условие доступа для</th>
      <th rowspan="3">Примечание</th>
    </tr>
    <tr >
      <th colspan="2">Ключ A</th>
      <th colspan="2">Биты доступа</th>
      <th colspan="2">Ключ B</th>
    </tr>
    <tr >
      <th>C1<sub>3</sub></th>
      <th>C2<sub>3</sub></th>
      <th>C3<sub>3</sub></th>
      <th>Чтение</th>
      <th>Запись</th>
      <th>Чтение</th>
      <th>Запись</th>
      <th>Чтение</th>
      <th>Запись</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>0</td>
      <td>0</td>
      <td>0</td>
      <td>никогда</td>
      <td>ключ A</td>
      <td>ключ A</td>
      <td>никогда</td>
      <td>ключ A</td>
      <td>ключ A</td>
      <td>Ключ B может быть прочитан</td>
    </tr>
    <tr>
      <td>0</td>
      <td>1</td>
      <td>0</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>ключ A</td>
      <td>никогда</td>
      <td>ключ A</td>
      <td>никогда</td>
      <td>Ключ B может быть прочитан</td>
    </tr>
    <tr>
      <td>1</td>
      <td>0</td>
      <td>0</td>
      <td>никогда</td>
      <td>ключ B</td>
      <td>ключ A|B</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>ключ B</td>
      <td></td>
    </tr>
    <tr>
      <td>1</td>
      <td>1</td>
      <td>0</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>ключ A|B</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>никогда</td>
      <td></td>
    </tr>
    <tr>
      <td>0</td>
      <td>0</td>
      <td>1</td>
      <td>никогда</td>
      <td>ключ A</td>
      <td>ключ A</td>
      <td>ключ A</td>
      <td>ключ A</td>
      <td>ключ A</td>
      <td>Ключ B может быть прочитан; конфигурация по умолчанию</td>
    </tr>
    <tr>
      <td>0</td>
      <td>1</td>
      <td>1</td>
      <td>никогда</td>
      <td>ключ B</td>
      <td>ключ A|B</td>
      <td>ключ B</td>
      <td>никогда</td>
      <td>ключ B</td>
      <td></td>
    </tr>
    <tr>
      <td>1</td>
      <td>0</td>
      <td>1</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>ключ A|B</td>
      <td>ключ B</td>
      <td>никогда</td>
      <td>никогда</td>
      <td></td>
    </tr>
    <tr>
      <td>1</td>
      <td>1</td>
      <td>1</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>ключ A|B</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>никогда</td>
      <td></td>
    </tr>
  </tbody>
</table>

**Как разобраться в этой таблице?**

Это простая таблица, которая показывает связь между битовыми комбинациями и разрешениями.

Например, пусть вы выбрали «1 0 0» (третья строка таблицы). Тогда ключ A и ключ B нельзя прочитать. Однако ключом B можно изменить и ключ A, и ключ B. Биты доступа можно прочитать и ключом A, и ключом B. Но сами биты доступа изменить нельзя никогда.

Теперь вопрос: где хранить эти биты? Мы разместим их в 6-м, 7-м и 8-м байтах в определённом месте, которое будет объяснено чуть позже.

### Таблица 2: Условия доступа для блоков данных

Это относится ко всем блокам данных. В исходном даташите нет индекса «Y», я добавил его для ясности. Здесь «Y» обозначает номер блока (от 0 до 2).

Конфигурация по умолчанию означает, что и ключ A, и ключ B могут выполнять все операции. Однако, как видно из предыдущей таблицы, ключ B читаем (в конфигурации по умолчанию), поэтому он не может использоваться для аутентификации. Следовательно, можно использовать только ключ A.

<table class="table-bordered">
  <thead>
    <tr >
      <th colspan="3">Биты доступа</th>
      <th colspan="4">Условие доступа для</th>
      <th rowspan="2">Применение</th>
    </tr>
    <tr >
      <th>C1<sub>Y</sub></th>
      <th>C2<sub>Y</sub></th>
      <th>C3<sub>Y</sub></th>
      <th>Чтение</th>
      <th>Запись</th>
      <th>Инкремент</th>
      <th>Декремент, перенос/восстановление</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>0</td>
      <td>0</td>
      <td>0</td>
      <td>ключ A|B</td>
      <td>ключ A|B</td>
      <td>ключ A|B</td>
      <td>ключ A|B</td>
      <td>Конфигурация по умолчанию</td>
    </tr>
    <tr>
      <td>0</td>
      <td>1</td>
      <td>0</td>
      <td>ключ A|B</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>блок чтения/записи</td>
    </tr>
    <tr>
      <td>1</td>
      <td>0</td>
      <td>0</td>
      <td>ключ A|B</td>
      <td>ключ B</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>блок чтения/записи</td>
    </tr>
    <tr>
      <td>1</td>
      <td>1</td>
      <td>0</td>
      <td>ключ A|B</td>
      <td>ключ B</td>
      <td>ключ B</td>
      <td>ключ A|B</td>
      <td>блок значения</td>
    </tr>
    <tr>
      <td>0</td>
      <td>0</td>
      <td>1</td>
      <td>ключ A|B</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>ключ A|B</td>
      <td>блок значения</td>
    </tr>
    <tr>
      <td>0</td>
      <td>1</td>
      <td>1</td>
      <td>ключ B</td>
      <td>ключ B</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>блок чтения/записи</td>
    </tr>
    <tr>
      <td>1</td>
      <td>0</td>
      <td>1</td>
      <td>ключ B</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>блок чтения/записи</td>
    </tr>
    <tr>
      <td>1</td>
      <td>1</td>
      <td>1</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>никогда</td>
      <td>блок чтения/записи</td>
    </tr>
  </tbody>
</table>
Примечание: «Если ключ B можно прочитать в служебном блоке сектора, его нельзя использовать для аутентификации. В результате, если считыватель использует ключ B для аутентификации блока с условиями доступа, которые используют ключ B, карта откажет в любом дальнейшем доступе к памяти после аутентификации».

**Как разобраться в этой таблице?**

Она похожа на предыдущую: показывает связь между битовыми комбинациями и разрешениями.

Например, если выбрать «0 1 0» (вторая строка таблицы) и применить это разрешение к блоку 1, то для чтения блока 1 можно использовать и ключ A, и ключ B. Однако никакие другие операции с блоком 1 выполнить нельзя.

Обозначение такое: номер блока записывается как нижний индекс к меткам битов (например, C1<sub>1</sub>, C2<sub>1</sub>, C3<sub>1</sub>). Здесь индекс «1» обозначает блок 1. Для выбранной комбинации «0 1 0» это означает:

- C1<sub>1</sub> = 0
- C2<sub>1</sub> = 1
- C3<sub>1</sub> = 0

Эти биты также будут размещены в 6-м, 7-м и 8-м байтах в определённом месте, которое будет объяснено чуть позже.

### Таблица 3: Таблица условий доступа

Раскрасим исходную таблицу, чтобы лучше увидеть, что означает каждый бит. Седьмой и третий биты в каждом байте относятся к служебному блоку сектора. Шестой и второй биты соответствуют блоку 2. Пятый и первый биты связаны с блоком 1. Четвёртый и нулевой биты относятся к блоку 0.

Надчёркивание в обозначении означает инвертированное значение. Это значит, что если значение CX<sub>y</sub> равно 0, то <span style="text-decoration: overline;">CX</span><sub>y</sub> становится равным 1.

<table >
  <thead>
    <tr>
      <th>Байт</th>
      <th>7</th>
      <th>6</th>
      <th>5</th>
      <th>4</th>
      <th>3</th>
      <th>2</th>
      <th>1</th>
      <th>0</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Байт 6</td>
      <td style="color:white;background-color:#8B0000"><span style="text-decoration: overline;">C2</span><sub>3</sub></td>
      <td style="color:white;background-color:#002D62"><span style="text-decoration: overline;">C2</span><sub>2</sub></td>
      <td style="color:white;background-color:#78184A" ><span style="text-decoration: overline;">C2</span><sub>1</sub></td>
      <td style="color:white;background-color:#234F1E"><span style="text-decoration: overline;">C2</span><sub>0</sub></td>
      <td style="color:white;background-color:#8B0000"><span style="text-decoration: overline;">C1</span><sub>3</sub></td>
      <td style="color:white;background-color:#002D62"><span style="text-decoration: overline;">C1</span><sub>2</sub></td>
      <td style="color:white;background-color:#78184A"><span style="text-decoration: overline;">C1</span><sub>1</sub></td>
      <td style="color:white;background-color:#234F1E"><span style="text-decoration: overline;">C1</span><sub>0</sub></td>
    </tr>
    <tr>
      <td>Байт 7</td>
      <td style="color:white;background-color:#8B0000">C1<sub>3</sub></td>
      <td style="color:white;background-color:#002D62">C1<sub>2</sub></td>
      <td style="color:white;background-color:#78184A">C1<sub>1</sub></td>
      <td style="color:white;background-color:#234F1E">C1<sub>0</sub></td>
      <td style="color:white;background-color:#8B0000"><span style="text-decoration: overline;">C3</span><sub>3</sub></td>
      <td style="color:white;background-color:#002D62"><span style="text-decoration: overline;">C3</span><sub>2</sub></td>
      <td style="color:white;background-color:#78184A"><span style="text-decoration: overline;">C3</span><sub>1</sub></td>
      <td style="color:white;background-color:#234F1E"><span style="text-decoration: overline;">C3</span><sub>0</sub></td>
    </tr>
    <tr>
      <td>Байт 8</td>
      <td style="color:white;background-color:#8B0000">C3<sub>3</sub></td>
      <td style="color:white;background-color:#002D62">C3<sub>2</sub></td>
      <td style="color:white;background-color:#78184A">C3<sub>1</sub></td>
      <td style="color:white;background-color:#234F1E">C3<sub>0</sub></td>
      <td style="color:white;background-color:#8B0000">C2<sub>3</sub></td>
      <td style="color:white;background-color:#002D62">C2<sub>2</sub></td>
      <td style="color:white;background-color:#78184A">C2<sub>1</sub></td>
      <td style="color:white;background-color:#234F1E">C2<sub>0</sub></td>
    </tr>
  </tbody>
</table>

Биты доступа по умолчанию — «FF 07 80». Попробуем понять, что это значит.
<table border="1">
  <thead>
    <tr>
      <th>Байт</th>
      <th>7</th>
      <th>6</th>
      <th>5</th>
      <th>4</th>
      <th>3</th>
      <th>2</th>
      <th>1</th>
      <th>0</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Байт 6</td>
      <td style="color:white;background-color:#8B0000">1</td>
      <td style="color:white;background-color:#002D62">1</td>
      <td style="color:white;background-color:#78184A" >1</td>
      <td style="color:white;background-color:#234F1E">1</td>
      <td style="color:white;background-color:#8B0000">1</td>
      <td style="color:white;background-color:#002D62">1</td>
      <td style="color:white;background-color:#78184A">1</td>
      <td style="color:white;background-color:#234F1E">1</td>
    </tr>
    <tr>
      <td>Байт 7</td>
      <td style="color:white;background-color:#8B0000">0</td>
      <td style="color:white;background-color:#002D62">0</td>
      <td style="color:white;background-color:#78184A">0</td>
      <td style="color:white;background-color:#234F1E">0</td>
      <td style="color:white;background-color:#8B0000">0</td>
      <td style="color:white;background-color:#002D62">1</td>
      <td style="color:white;background-color:#78184A">1</td>
      <td style="color:white;background-color:#234F1E">1</td>
    </tr>
    <tr>
      <td>Байт 8</td>
      <td style="color:white;background-color:#8B0000">1</td>
      <td style="color:white;background-color:#002D62">0</td>
      <td style="color:white;background-color:#78184A">0</td>
      <td style="color:white;background-color:#234F1E">0</td>
      <td style="color:white;background-color:#8B0000">0</td>
      <td style="color:white;background-color:#002D62">0</td>
      <td style="color:white;background-color:#78184A">0</td>
      <td style="color:white;background-color:#234F1E">0</td>
    </tr>
  </tbody>
</table>

Из таблицы выше можно вывести значения CX<sub>Y</sub>. Обратите внимание: только C3<sub>3</sub> равно 1, а все остальные значения равны 0. Теперь посмотрите Таблицу 1 и Таблицу 2, чтобы понять, какому разрешению это соответствует.

<table border="1">
  <thead>
    <tr>
      <th>Блок</th>
      <th>C1<sub>Y</sub></th>
      <th>C2<sub>Y</sub></th>
      <th>C3<sub>Y</sub></th>
      <th>Доступ</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Блок 0</td>
      <td>0</td>
      <td>0</td>
      <td>0</td>
      <td>Все разрешения с ключом A</td>
    </tr>
    <tr>
      <td>Блок 1</td>
      <td>0</td>
      <td>0</td>
      <td>0</td>
      <td>Все разрешения с ключом A</td>
    </tr>
    <tr>
      <td>Блок 2</td>
      <td>0</td>
      <td>0</td>
      <td>0</td>
      <td>Все разрешения с ключом A</td>
    </tr>
    <tr>
      <td>Блок 3 (служебный)</td>
      <td>0</td>
      <td>0</td>
      <td>1</td>
      <td>Ключ A можно записать с помощью ключа A. Биты доступа и ключ B можно читать и записывать только с помощью ключа A. </td>
    </tr>
  </tbody>
</table>

Поскольку ключ B читаем, его нельзя использовать для аутентификации.

### Калькулятор на следующей странице

Всё ещё сложно? Используйте калькулятор на следующей странице, чтобы поэкспериментировать с различными комбинациями. Меняйте разрешения для каждого блока и смотрите, как соответственно меняются значения битов доступа.

### Ссылка

- [11-я страница даташита](https://www.nxp.com/docs/en/data-sheet/MF1S50YYX_V1.pdf)
