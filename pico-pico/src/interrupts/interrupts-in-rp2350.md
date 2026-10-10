{{#title Обзор прерываний RP2350 для Raspberry Pi Pico 2 | impl Rust для RP2350}}

# Прерывания в RP2350

В предыдущей главе мы рассмотрели, что такое прерывания и какую роль играет NVIC. Теперь давайте посмотрим, какие прерывания на самом деле доступны в RP2350.

Прерывания делятся на две группы: системные исключения и внешние прерывания.

Системные исключения определены самой архитектурой процессора. К ним относятся сброс, обработчики сбоев и системный таймер. На большинстве чипов Cortex-M они работают одинаково.

Внешние прерывания приходят от периферии RP2350. У каждого периферийного модуля, который может генерировать прерывание, есть номер IRQ и имя вектора. Именно эти имена вы будете видеть в коде.

Таблица ниже показывает внешние прерывания RP2350, пронумерованные от 0 до 51. Они охватывают распространённую периферию, например таймеры, GPIO, DMA и интерфейсы связи вроде I2C, SPI и UART.

Запоминать эту таблицу не нужно. Её назначение — помочь понять, откуда берутся такие имена, как `I2C0_IRQ` или `UART0_IRQ`, когда вы встречаете их в примерах или документации.

Полные подробности см. в [разделе 3.2 даташита RP2350](https://pip-assets.raspberrypi.com/categories/1214-rp2350/documents/RP-008373-DS-2-rp2350-datasheet.pdf?disposition=inline#page=83) на странице 82.

В следующей главе мы увидим, как Embassy использует эти прерывания, не требуя от вас писать обработчики прерываний вручную.

<div class="interrupt-container">
<header>
    <h1>Внешние прерывания RP2350</h1>
</header>

> [!IMPORTANT]
> Некоторые описания прерываний здесь упрощены для удобного обзора для начинающих. Более точные и подробные сведения см. в даташите RP2350.

<div class="interrupt-content">

<h2 class="interrupt-section-title">Таймеры</h2>
<p>Аварийные сигналы таймеров, используемые для задержек и планирования.</p>
<table>
<thead>
<tr><th>IRQ</th><th>Вектор</th><th>Описание</th></tr>
</thead>
<tbody>
<tr><td>0</td><td>TIMER0_IRQ_0</td><td>Прерывание аварийного сигнала таймера 0</td></tr>
<tr><td>1</td><td>TIMER0_IRQ_1</td><td>Прерывание аварийного сигнала таймера 0</td></tr>
<tr><td>2</td><td>TIMER0_IRQ_2</td><td>Прерывание аварийного сигнала таймера 0</td></tr>
<tr><td>3</td><td>TIMER0_IRQ_3</td><td>Прерывание аварийного сигнала таймера 0</td></tr>
<tr><td>4</td><td>TIMER1_IRQ_0</td><td>Прерывание аварийного сигнала таймера 1</td></tr>
<tr><td>5</td><td>TIMER1_IRQ_1</td><td>Прерывание аварийного сигнала таймера 1</td></tr>
<tr><td>6</td><td>TIMER1_IRQ_2</td><td>Прерывание аварийного сигнала таймера 1</td></tr>
<tr><td>7</td><td>TIMER1_IRQ_3</td><td>Прерывание аварийного сигнала таймера 1</td></tr>
</tbody>
</table>

<h2 class="interrupt-section-title">ШИМ</h2>
<p>События переполнения счётчика ШИМ.</p>
<table>
<thead>
<tr><th>IRQ</th><th>Вектор</th><th>Описание</th></tr>
</thead>
<tbody>
<tr><td>8</td><td>PWM_IRQ_WRAP_0</td><td>Прерывание переполнения ШИМ</td></tr>
<tr><td>9</td><td>PWM_IRQ_WRAP_1</td><td>Прерывание переполнения ШИМ</td></tr>
</tbody>
</table>

<h2 class="interrupt-section-title">DMA</h2>
<p>События передачи DMA.</p>
<table>
<thead>
<tr><th>IRQ</th><th>Вектор</th><th>Описание</th></tr>
</thead>
<tbody>
<tr><td>10</td><td>DMA_IRQ_0</td><td>Прерывание передачи DMA</td></tr>
<tr><td>11</td><td>DMA_IRQ_1</td><td>Прерывание передачи DMA</td></tr>
<tr><td>12</td><td>DMA_IRQ_2</td><td>Прерывание передачи DMA</td></tr>
<tr><td>13</td><td>DMA_IRQ_3</td><td>Прерывание передачи DMA</td></tr>
</tbody>
</table>

<h2 class="interrupt-section-title">USB</h2>
<p>События контроллера USB.</p>
<table>
<thead>
<tr><th>IRQ</th><th>Вектор</th><th>Описание</th></tr>
</thead>
<tbody>
<tr><td>14</td><td>USBCTRL_IRQ</td><td>Прерывание контроллера USB</td></tr>
</tbody>
</table>

<h2 class="interrupt-section-title">PIO</h2>
<p>События конечных автоматов PIO.</p>
<table>
<thead>
<tr><th>IRQ</th><th>Вектор</th><th>Описание</th></tr>
</thead>
<tbody>
<tr><td>15</td><td>PIO0_IRQ_0</td><td>Прерывание PIO 0</td></tr>
<tr><td>16</td><td>PIO0_IRQ_1</td><td>Прерывание PIO 0</td></tr>
<tr><td>17</td><td>PIO1_IRQ_0</td><td>Прерывание PIO 1</td></tr>
<tr><td>18</td><td>PIO1_IRQ_1</td><td>Прерывание PIO 1</td></tr>
<tr><td>19</td><td>PIO2_IRQ_0</td><td>Прерывание PIO 2</td></tr>
<tr><td>20</td><td>PIO2_IRQ_1</td><td>Прерывание PIO 2</td></tr>
</tbody>
</table>

<h2 class="interrupt-section-title">GPIO и базовый ввод-вывод</h2>
<p>События GPIO и межъядерной сигнализации.</p>
<table>
<thead>
<tr><th>IRQ</th><th>Вектор</th><th>Описание</th></tr>
</thead>
<tbody>
<tr><td>21</td><td>IO_IRQ_BANK0</td><td>Прерывание GPIO</td></tr>
<tr><td>22</td><td>IO_IRQ_BANK0_NS</td><td>Прерывание GPIO</td></tr>
<tr><td>23</td><td>IO_IRQ_QSPI</td><td>Прерывание GPIO QSPI</td></tr>
<tr><td>24</td><td>IO_IRQ_QSPI_NS</td><td>Прерывание GPIO QSPI</td></tr>
<tr><td>25</td><td>SIO_IRQ_FIFO</td><td>Прерывание FIFO между ядрами</td></tr>
<tr><td>26</td><td>SIO_IRQ_BELL</td><td>Прерывание doorbell между ядрами</td></tr>
<tr><td>27</td><td>SIO_IRQ_FIFO_NS</td><td>Прерывание FIFO между ядрами</td></tr>
<tr><td>28</td><td>SIO_IRQ_BELL_NS</td><td>Прерывание doorbell между ядрами</td></tr>
<tr><td>29</td><td>SIO_IRQ_MTIMECMP</td><td>Прерывание системного таймера</td></tr>
</tbody>
</table>

<h2 class="interrupt-section-title">Периферия связи</h2>
<p>События интерфейсов связи.</p>
<table>
<thead>
<tr><th>IRQ</th><th>Вектор</th><th>Описание</th></tr>
</thead>
<tbody>
<tr><td>30</td><td>CLOCKS_IRQ</td><td>Прерывание системы тактирования</td></tr>
<tr><td>31</td><td>SPI0_IRQ</td><td>Прерывание SPI</td></tr>
<tr><td>32</td><td>SPI1_IRQ</td><td>Прерывание SPI</td></tr>
<tr><td>33</td><td>UART0_IRQ</td><td>Прерывание UART</td></tr>
<tr><td>34</td><td>UART1_IRQ</td><td>Прерывание UART</td></tr>
<tr><td>35</td><td>ADC_IRQ_FIFO</td><td>Прерывание FIFO АЦП</td></tr>
<tr><td>36</td><td>I2C0_IRQ</td><td>Прерывание I2C</td></tr>
<tr><td>37</td><td>I2C1_IRQ</td><td>Прерывание I2C</td></tr>
</tbody>
</table>

<h2 class="interrupt-section-title">Система и питание</h2>
<p>События управления системой и питанием.</p>
<table>
<thead>
<tr><th>IRQ</th><th>Вектор</th><th>Описание</th></tr>
</thead>
<tbody>
<tr><td>38</td><td>OTP_IRQ</td><td>Прерывание OTP</td></tr>
<tr><td>39</td><td>TRNG_IRQ</td><td>Прерывание генератора случайных чисел</td></tr>
<tr><td>40</td><td>Reserved</td><td>Зарезервировано</td></tr>
<tr><td>41</td><td>Reserved</td><td>Зарезервировано</td></tr>
<tr><td>42</td><td>PLL_SYS_IRQ</td><td>Прерывание системной PLL</td></tr>
<tr><td>43</td><td>PLL_USB_IRQ</td><td>Прерывание PLL USB</td></tr>
<tr><td>44</td><td>POWMAN_IRQ_POW</td><td>Прерывание менеджера питания</td></tr>
<tr><td>45</td><td>POWMAN_IRQ_TIMER</td><td>Прерывание таймера менеджера питания</td></tr>
</tbody>
</table>

<h2 class="interrupt-section-title">Программные прерывания</h2>
<p>Прерывания, которые можно вызвать программно.</p>
<table>
<thead>
<tr><th>IRQ</th><th>Вектор</th><th>Описание</th></tr>
</thead>
<tbody>
<tr><td>46</td><td>SPAREIRQ_IRQ_0</td><td>Программное прерывание</td></tr>
<tr><td>47</td><td>SPAREIRQ_IRQ_1</td><td>Программное прерывание</td></tr>
<tr><td>48</td><td>SPAREIRQ_IRQ_2</td><td>Программное прерывание</td></tr>
<tr><td>49</td><td>SPAREIRQ_IRQ_3</td><td>Программное прерывание</td></tr>
<tr><td>50</td><td>SPAREIRQ_IRQ_4</td><td>Программное прерывание</td></tr>
<tr><td>51</td><td>SPAREIRQ_IRQ_5</td><td>Программное прерывание</td></tr>
</tbody>
</table>

</div>
</div>
