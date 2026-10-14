{{#title Биты доступа MIFARE: условия доступа и права на секторы}}

# Контроль доступа

На метке есть биты доступа, которые задают правила доступа к данным, хранящимся на ней. В этой главе мы разберём, как работают эти биты. Раздел может показаться немного сложным, поэтому я постараюсь объяснить его как можно проще и понятнее.

> [!Caution]
> Будьте осторожны при записи битов доступа: неверные значения могут сделать сектор недоступным.


## Права
Это основные права, которыми задаются условия доступа. В таблице описано каждое действие и указано, к каким блокам оно применимо: обычным блокам данных (чтение/запись), блокам значений или трейлерам секторов.

| **Операция**  | **Описание**                                               | **Применимо к типу блока**        |
|---------------|------------------------------------------------------------|-----------------------------------|
| **Read** (чтение)      | Читает один блок памяти                                     | Чтение/запись, значение, трейлер сектора |
| **Write** (запись)     | Записывает один блок памяти                                    | Чтение/запись, значение, трейлер сектора |
| **Increment** (инкремент) | Увеличивает содержимое блока и сохраняет результат во внутреннем буфере передачи | Значение                             |
| **Decrement** (декремент) | Уменьшает содержимое блока и сохраняет результат во внутреннем буфере передачи | Значение                             |
| **Restore** (восстановление)   | Читает содержимое блока во внутренний буфер передачи | Значение                             |
| **Transfer** (передача)  | Записывает содержимое внутреннего буфера передачи в блок | Значение, чтение/запись                 |

## Условия доступа
Начнём с главного: с условий доступа. Во время исследования я заметил, что многим людям трудно разобраться в разделе об условиях доступа в даташите. Вот моя попытка объяснить его понятно 🤞.

Для управления правами каждого блока используется всего три бита, то есть одна из восьми комбинаций. В официальном даташите это обозначается как CX<sub>Y</sub> (C1₀, C1₂... C3₃) для битов доступа. Первое число (X) в этой записи — номер бита доступа, от 1 до 3, каждый из которых соответствует определённому типу прав. Однако смысл этих прав меняется в зависимости от того, является ли блок блоком данных или трейлером. Второе число (Y) в подстрочном индексе обозначает относительный номер блока — от 0 до 3.

### Таблица 1: условия доступа для трейлера сектора
В оригинальном даташите подстрочный индекс в таблице не указан. Я добавил индекс «3», так как трейлер сектора находится в блоке 3.

> [!Important]
> Если ключ можно прочитать, его нельзя использовать как ключ аутентификации. Поэтому в этой таблице всякий раз, когда ключ B можно прочитать, он не может служить ключом аутентификации. Если вы заметили, да: ключ A прочитать нельзя никогда.

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
      <td>Ключ B можно прочитать</td>
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
      <td>Ключ B можно прочитать</td>
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
      <td>Ключ B можно прочитать; конфигурация по умолчанию</td>
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

Это простая таблица, которая показывает связь между битовыми комбинациями и правами.
 
Например:
Допустим, вы выбрали «1 0 0» (3-я строка таблицы). Тогда вы не можете прочитать ни ключ A, ни ключ B. Однако вы можете изменить значение ключа A, а также ключа B, используя ключ B. Биты доступа можно прочитать любым из ключей, A или B. Но изменить биты доступа нельзя никогда.

Теперь вопрос: где хранить эти биты? Мы разместим их в 6-м, 7-м и 8-м байтах в определённом месте, которое будет объяснено чуть позже.

### Таблица 2: условия доступа для блоков данных
Это относится ко всем блокам данных. В оригинальном даташите подстрочный индекс «Y» не указан, я добавил его для наглядности. Здесь «Y» означает номер блока (от 0 до 2).

Конфигурация по умолчанию означает, что и ключ A, и ключ B могут выполнять все операции. Однако, как видно из предыдущей таблицы, ключ B можно прочитать (в конфигурации по умолчанию), поэтому его нельзя использовать для аутентификации. Значит, использовать можно только ключ A.

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
      <th>Декремент, передача/восстановление</th>
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
Примечание: «Если ключ B можно прочитать в трейлере сектора, его нельзя использовать для аутентификации. В результате, если считыватель использует ключ B для аутентификации блока с условиями доступа, которые используют ключ B, карта откажет в любом дальнейшем доступе к памяти после аутентификации».

**Как разобраться в этой таблице?** 

Она похожа на предыдущую: показывает связь между битовыми комбинациями и правами.

Например:
Если выбрать «0 1 0» (2-я строка таблицы) и применить это право к блоку 1, то читать блок 1 можно будет либо ключом A, либо ключом B. Однако никакие другие операции с блоком 1 выполнять нельзя.  

Обозначение такое: номер блока записывается как подстрочный индекс к названиям битов (например, C1<sub>1</sub>, C2<sub>1</sub>, C3<sub>1</sub>). Здесь подстрочный индекс «1» означает блок 1. Для выбранной комбинации «0 1 0» это значит:  
- C1<sub>1</sub> = 0  
- C2<sub>1</sub> = 1  
- C3<sub>1</sub> = 0  

Эти биты тоже будут размещены в 6-м, 7-м и 8-м байтах в определённом месте, которое будет объяснено чуть позже.

### Таблица 3: таблица условий доступа
Раскрасим исходную таблицу, чтобы лучше видеть, что означает каждый бит. 7-й и 3-й биты каждого байта относятся к трейлеру сектора. 6-й и 2-й биты соответствуют блоку 2. 5-й и 1-й биты связаны с блоком 1. 4-й и 0-й биты относятся к блоку 0.

Черта над обозначением означает инвертированное значение. То есть если значение CX<sub>y</sub> равно 0, то <span style="text-decoration: overline;">CX</span><sub>y</sub> становится 1.

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

По умолчанию биты доступа равны «FF 07 80». Попробуем понять, что это значит.
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

Из таблицы выше можно вывести значения CX<sub>Y</sub>. Обратите внимание, что только C3<sub>3</sub> равен 1, а все остальные значения равны 0. Теперь посмотрите таблицы 1 и 2, чтобы понять, какому праву это соответствует.
 
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
      <td>Все права с ключом A</td>
    </tr>
    <tr>
      <td>Блок 1</td>
      <td>0</td>
      <td>0</td>
      <td>0</td>
      <td>Все права с ключом A</td>
    </tr>
    <tr>
      <td>Блок 2</td>
      <td>0</td>
      <td>0</td>
      <td>0</td>
      <td>Все права с ключом A</td>
    </tr>
    <tr>
      <td>Блок 3 (трейлер)</td>
      <td>0</td>
      <td>0</td>
      <td>1</td>
      <td>Ключ A можно записать с помощью ключа A. Биты доступа и ключ B можно читать и записывать только с помощью ключа A.</td>
    </tr>
  </tbody>
</table>

Поскольку ключ B можно прочитать, использовать его для аутентификации нельзя.

### Калькулятор на следующей странице
Всё ещё запутались? Используйте калькулятор на следующей странице, чтобы поэкспериментировать с разными комбинациями. Меняйте права для каждого блока и смотрите, как соответственно меняются значения битов доступа.

### Ссылки
  - [11-я страница даташита](https://www.nxp.com/docs/en/data-sheet/MF1S50YYX_V1.pdf)
