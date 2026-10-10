# Подключение к Wi-Fi

В этой главе мы подключим плату к локальной сети Wi-Fi, получим IP-адрес и будем отправлять каждое показание датчика на компьютер в той же сети. Готовый код для этой главы находится в `project/part2/`.

Инициализация HAL, настройка I2C и последовательность измерения SHTC3 из главы [Чтение данных с датчика](./reading-sensor-data.md) остаются прежними. Меняется структура проекта, а также появляются новые сетевые компоненты: куча (heap), сетевой стек, фоновые задачи и учётные данные, известные на этапе компиляции.

## От одного файла к модулям

В предыдущей главе весь код находился в `main.rs`. Когда мы добавим Wi-Fi и HTTP, этот файл станет трудно читать, поэтому мы перенесём связанный код в модули:

```rust,ignore
{{#include ../../project/part2/src/main.rs:modules}}
```

- `sensor.rs` содержит вспомогательную функцию измерения SHTC3, которая раньше находилась в основном цикле. Шина I2C по-прежнему создаётся в `main`, но запуск измерения, ожидание результата и его преобразование теперь находятся в `read_sensor`:

```rust,ignore
{{#include ../../project/part2/src/sensor.rs:read_sensor}}
```

- `network.rs` содержит две фоновые задачи, которые поддерживают подключение Wi-Fi-радиомодуля и работу IP-стека.
- `http.rs` формирует JSON-нагрузку и отправляет её с помощью минимального HTTP-запроса `POST`.
- `main.rs` по-прежнему инициализирует аппаратное обеспечение, запускает сетевой стек и выполняет основной цикл приложения.

Такое разделение делает новые сетевые понятия проще для чтения, а те же модули будут расширяться в следующих главах.

## Зачем нужен `esp-alloc`

Программа `no_std` не имеет глобального аллокатора, если мы его не предоставим. В предыдущей главе куча не требовалась: драйвер датчика работает с буферами на стеке, и мы не создавали `String` или `Vec`.

Wi-Fi устроен иначе. [`esp-radio`][esp-radio] — обёртка над предкомпилированным драйвером радиомодуля, и этот драйвер выделяет память во время выполнения. [`embassy-net`][embassy-net] и наш HTTP-помощник тоже требуют крейт [`alloc`][alloc], например когда `format!` формирует тело JSON. Без кучи приложение даже не сможет запустить радиомодуль.

Поэтому мы добавляем [`esp-alloc`][esp-alloc] и включаем крейт `alloc`:

```rust,ignore
{{#include ../../project/part2/src/main.rs:alloc_crate}}
```

```rust,ignore
{{#include ../../project/part2/src/main.rs:alloc_import}}
```

`extern crate alloc;` делает `Vec`, `String` и `format!` доступными в `no_std`. `use esp_alloc as _;` подключает крейт, чтобы его глобальный аллокатор был включён в сборку, хотя по имени мы к нему не обращаемся.

Затем создаём кучу до запуска планировщика и радиомодуля:

```rust,ignore
{{#shiftinclude auto:../../project/part2/src/main.rs:heap_init}}
```

[`heap_allocator!`][heap-allocator] можно вызывать несколько раз. Каждый вызов добавляет регион в один и тот же глобальный аллокатор. Для ESP32-C3, который используется в этом курсе, эти вызовы резервируют два региона кучи:

- 64 КиБ памяти, освобождённой от загрузчика (reclaimed), которая выбирается с помощью `#[ram(reclaimed)]`.
- Ещё 36 КиБ памяти.

Подробнее см. раздел [Allocating Memory][alloc-book] книги *The Rust on ESP Book* (на английском языке).

> Для `esp-radio` нужны одновременно куча и запущенный планировщик. Поэтому мы по-прежнему вызываем `esp_rtos::start(...)` до `esp_radio::wifi::new(...)`, как и требует документация крейта.

## Инициализация сетевого стека

Сеть разделена на два уровня. [`esp-radio`][esp-radio] управляет аппаратным Wi-Fi. [`embassy-net`][embassy-net] — IP-стек, который работает поверх этого интерфейса и предоставляет DHCP, DNS и TCP.

### Создание Wi-Fi-интерфейса

После запуска планировщика мы берём владение периферией Wi-Fi и создаём интерфейс в режиме станции:

```rust,ignore
{{#shiftinclude auto:../../project/part2/src/main.rs:wifi_controller}}
```

[`esp_radio::wifi::new`][wifi-new] возвращает [`WifiController`][wifi-controller] и набор интерфейсов. Контроллер подключается к точке доступа. `interfaces.station` — драйвер embassy-net, то есть Wi-Fi-клиент, который может отправлять и принимать кадры Ethernet.

### Конфигурация `embassy-net`

`embassy-net` настраивается в двух местах: через фичи Cargo и через значение `Config`, которое мы передаём во время выполнения.

Фичи крейта выбирают, какие протоколы будут скомпилированы:

```toml
{{#include ../../project/part2/Cargo.toml:embassy_net}}
```

- `medium-ethernet` обязательна, поскольку Wi-Fi-станция представляет собой интерфейс, похожий на Ethernet.
- `dhcpv4` позволяет стеку запрашивать IPv4-адрес у точки доступа.
- `udp` используется DHCP и DNS.
- `tcp` и `dns` понадобятся позже, когда мы откроем сокет и разрешим имя хоста.

Также включаем поддержку Embassy в `esp-rtos` и фичу `wifi` в `esp-radio`:

```toml
{{#include ../../project/part2/Cargo.toml:esp_radio}}
```

```toml
{{#include ../../project/part2/Cargo.toml:esp_rtos}}
```

Во время выполнения мы запрашиваем аренду адреса DHCP и инициализируем генератор стека случайным значением из аппаратного генератора случайных чисел (RNG):

```rust,ignore
{{#shiftinclude auto:../../project/part2/src/main.rs:embassy_net_config}}
```

[`Config::dhcpv4`][embassy-net-config] — аналог в `embassy-net` для фразы «получить адрес от роутера». Статический адрес тоже сработал бы, но DHCP — обычная настройка для сети разработки, и она позволяет не прописывать IP-адрес в прошивке.

Случайный `seed` не является криптографическим ключом. `embassy-net` использует его для значений протокола, которые не должны быть предсказуемыми для разных устройств, например для начальных номеров последовательности TCP и идентификаторов транзакций DHCP.

### Почему ресурсы стека статические

[`embassy_net::new`][embassy-net-new] требует драйвер Wi-Fi, конфигурацию, случайное зерно и буфер [`StackResources<N>`][stack-resources]. `N` — количество сокетов, которые стек может держать открытыми одновременно. Мы используем `3`: этого хватает для DHCP, DNS и одного TCP-соединения.

```rust,ignore
{{#shiftinclude auto:../../project/part2/src/main.rs:stack_init}}
```

Эти ресурсы, `Runner` и запускаемые задачи требуют времени жизни `'static`. Задачи Embassy не могут заимствовать данные из стека `main`: после запуска задача может работать до конца программы, поэтому все её аргументы должны жить дольше, чем `main`.

Обычный локальный `let resources = StackResources::new();` был бы уничтожен при выходе из `main` (хотя этого никогда не происходит, система типов не может этим воспользоваться), и его нельзя было бы передать задаче `'static`. `static mut` скомпилировался бы только с `unsafe`.

[`StaticCell`][static-cell] решает эту проблему. Это ячейка одноразовой инициализации: мы инициализируем её во время выполнения и получаем `&'static mut StackResources<3>`. После этого [`embassy_net::new`][embassy-net-new] возвращает [`Stack`][embassy-net-stack], который можно использовать из кода приложения, и [`Runner`][embassy-net-runner], который нужно непрерывно опрашивать.

## Зачем нужны задачи

Стек не будет полезен, пока в фоне не запущены два цикла:

```rust,ignore
{{#shiftinclude auto:../../project/part2/src/main.rs:spawn_tasks}}
```

Обе функции — задачи Embassy, созданные с помощью [`#[embassy_executor::task]`][embassy-task].

`connection` владеет `WifiController`. Она настраивает режим станции, подключается и, если связь разрывается, ждёт и пробует снова:

```rust,ignore
{{#include ../../project/part2/src/network.rs:connection_task}}
```

`net_task` владеет `Runner`. Её единственная задача — непрерывно опрашивать IP-стек:

```rust,ignore
{{#include ../../project/part2/src/network.rs:net_task}}
```

Мы используем задачи, потому что эти циклы никогда не завершаются и должны продвигаться одновременно с приложением. Если бы `main` сам вызывал `runner.run().await`, он никогда не считывал бы датчик и не отправлял бы HTTP. Если бы он выполнял только цикл подключения, DHCP и TCP зависли бы. Исполнитель Embassy переключается между задачами, как только одна из них вызывает `.await`, поэтому радиомодуль, IP-стек и основной цикл приложения могут работать на одном ядре.

Времена жизни `'static` у `WifiController` и `Runner` — то же ограничение, что и раньше: запущенная задача может пережить функцию, которая её создала.

## Переменные окружения

Имя сети и пароль не записаны строковыми литералами. Они считываются во время компиляции:

```rust,ignore
{{#include ../../project/part2/src/network.rs:wifi_env}}
```

[`env!`][env-macro] раскрывается в `&'static str`, взятый из окружения во время работы `rustc`. Если `SSID` или `PASSWORD` не заданы, сборка завершится с ошибкой. Так учётные данные не попадают в дерево исходного кода, а передаются при сборке или прошивке:

```shell
SSID="your-ssid" PASSWORD="your-password" cargo run --release
```

Cargo передаёт переменные окружения процесса компилятору, поэтому эти значения встраиваются в прошивку. Чтобы сменить сеть позже, нужно пересобрать прошивку.

HTTP-помощник использует необязательные переменные для получателя на вашем компьютере:

```rust,ignore
{{#include ../../project/part2/src/http.rs:http_env}}
```

[`option_env!`][option-env-macro] возвращает `None`, если переменная не задана, поэтому прошивка всё равно соберётся. Во время работы `send_sensor_data` записывает ошибку в лог и завершается, если `HOST_IP` не задан. `HTTP_PORT` по умолчанию равен `8080`.

## Ожидание адреса и отправка данных

После запуска задач `main` ждёт, пока связь не установится и DHCP не назначит адрес:

```rust,ignore
{{#shiftinclude auto:../../project/part2/src/main.rs:wait_ip}}
```

`wait_link_up()` завершается, когда станция ассоциирована с точкой доступа. Следующий цикл ждёт, пока `config_v4()` не вернёт `Some`, — это значит, что аренда адреса DHCP получена. Только после этого мы начинаем отправку.

Цикл приложения теперь состоит из двух вызовов: считать датчик и, если чтение успешно, отправить значения:

```rust,ignore
{{#shiftinclude auto:../../project/part2/src/main.rs:main_loop}}
```

`send_sensor_data` формирует небольшое тело JSON, открывает TCP-сокет и отправляет вручную сформированный HTTP/1.0-запрос `POST` на `/sensor`:

```rust,ignore
{{#shiftinclude auto:../../project/part2/src/http.rs:http_payload}}
```

Эта глава использует HTTP лишь как простой способ убедиться, что устройство может связаться с другим хостом. В следующей главе HTTP заменяется на MQTT.

## Запуск приложения

Прошивка отправляет данные на компьютер в той же сети. Из корня репозитория сначала запустите приёмник:

```shell
cargo xtask http-server
```

Задача выводит IP-адрес хоста, если может его определить. Используйте это значение как `HOST_IP`. Затем из `project/part2/` прошейте приложение:

```shell
SSID="your-ssid" PASSWORD="your-password" HOST_IP="192.168.1.10" cargo run --release
```

Замените SSID, пароль и IP-адрес на свои: на название вашей сети, пароль и адрес, который вывела `xtask`. Если сервер работает не на порту `8080`, можно задать `HTTP_PORT`.

Как только устройство подключится, в мониторе последовательного порта появится сообщение о подключении:

```text
INFO - Wifi connected!
INFO -   24.31 °C | 42.18 %RH
INFO - HTTP request sent
```

Терминал `http-server` должен выводить каждую отправленную полезную нагрузку:

```text
sensor payload={"temperature":24.31,"humidity":42.18}
```

На этом этапе плата может подключиться к сети Wi-Fi, получить IP-адрес и отправлять измерения за пределы устройства. В следующей главе мы опубликуем эти показания в брокер MQTT.

<!-- TODO: Use /latest/ when new docs are redeployed -->
[esp-radio]: https://docs.espressif.com/projects/rust/esp-radio/0.18.0/esp32c3/esp_radio/index.html
[esp-alloc]: https://docs.espressif.com/projects/rust/esp-alloc/0.10.0/esp_alloc/index.html
[heap-allocator]: https://docs.espressif.com/projects/rust/esp-alloc/0.10.0/esp_alloc/macro.heap_allocator.html
[alloc-book]: https://docs.espressif.com/projects/rust/book/application-development/alloc.html
[alloc]: https://doc.rust-lang.org/alloc/
[wifi-new]: https://docs.espressif.com/projects/rust/esp-radio/0.18.0/esp32c3/esp_radio/wifi/fn.new.html
[wifi-controller]: https://docs.espressif.com/projects/rust/esp-radio/0.18.0/esp32c3/esp_radio/wifi/struct.WifiController.html
[embassy-net]: https://docs.rs/embassy-net/0.9.1/embassy_net/
[embassy-net-new]: https://docs.rs/embassy-net/0.9.1/embassy_net/fn.new.html
[embassy-net-config]: https://docs.rs/embassy-net/0.9.1/embassy_net/struct.Config.html#method.dhcpv4
[embassy-net-stack]: https://docs.rs/embassy-net/0.9.1/embassy_net/struct.Stack.html
[embassy-net-runner]: https://docs.rs/embassy-net/0.9.1/embassy_net/struct.Runner.html
[stack-resources]: https://docs.rs/embassy-net/0.9.1/embassy_net/struct.StackResources.html
[static-cell]: https://docs.rs/static_cell/2.1.1/static_cell/struct.StaticCell.html
[embassy-task]: https://docs.rs/embassy-executor/0.10.0/embassy_executor/attr.task.html
[env-macro]: https://doc.rust-lang.org/core/macro.env.html
[option-env-macro]: https://doc.rust-lang.org/core/macro.option_env.html
