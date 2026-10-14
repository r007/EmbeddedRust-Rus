{{#title Асинхронные HTTP-запросы по Wi-Fi на ESP32 с Embassy и Rust}}

# Подключение ESP32 к Wi-Fi и асинхронный доступ к сайтам с Embassy

В этом упражнении мы снова используем режим STA для доступа к сайту, но на этот раз сделаем это асинхронно. В качестве HTTP-клиента будем использовать крейт reqwless, а для поддержки асинхронности во встраиваемых системах — фреймворк Embassy. Кроме них, подключим и другие вспомогательные крейты.


## Создание проекта с помощью esp-generate

Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 wifi-async-http
```

Откроется экран с вопросом о выборе опций. Чтобы включить Wi-Fi, сначала нужно включить функции «unstable» и «alloc». Как вы могли заметить, пока не выбраны эти две опции, пункт Wi-Fi недоступен. Поэтому выбирайте их по одной:

- Сначала выберите опцию «Enable unstable HAL features».
- Выберите опцию «Enable allocations via the esp-alloc crate».
- Теперь можно включить «Enable Wi-Fi via esp-radio crate».
- Выберите опцию «Adds embassy framework support».

Также включите логирование.

- Прокрутите до пункта «Flashing, logging and debugging (espflash)» и нажмите Enter.
- Затем выберите «Use defmt to print messages».

Сохраните настройки, нажав на клавиатуре «s».

## Обновляем зависимости

### Embassy net

embassy-net, построенный на крейте smoltcp, предоставляет асинхронный сетевой стек для встраиваемых систем. Он предлагает API более высокого уровня и поддерживает IPv4, IPv6, среды Ethernet и bare-IP, а также TCP, UDP, DNS, DHCPv4 и multicast. 

Этот крейт добавляется автоматически, если при генерации проекта с помощью esp-generate вы выбрали опции Wi-Fi и Embassy. Однако нам нужна ещё одна функция: «dns». Она необходима для HTTP-запросов, потому что имя сайта (например, google.com) нужно преобразовать в IP-адрес. Напомню, что в предыдущем упражнении мы указывали IP-адрес сайта вручную.

```toml
# Обновлённый embassy-net в Cargo.toml
embassy-net = { version = "0.7.1", features = [
  "defmt",
  "dhcpv4",
  "medium-ethernet",
  "tcp",
  "udp",
  #добавлено:
  "dns",
] }
```

### smoltcp

Крейт smoltcp тоже добавляется в Cargo.toml. Чтобы использовать DNS-серверы, нужно включить feature «dns-max-server-count-4».

```toml
# Обновлённый smoltcp в Cargo.toml
smoltcp = { version = "0.12.0", default-features = false, features = [
    "defmt",
    "medium-ethernet",
    "multicast",
    "proto-dhcpv4",
    "proto-dns",
    "proto-ipv4",
    "socket-dns",
    "socket-icmp",
    "socket-raw",
    "socket-tcp",
    "socket-udp",
    # добавлено:
    "dns-max-server-count-4", 
] }
```

### Reqwless
Крейт reqwless — это HTTP-клиент для встраиваемых систем, который работает с любым транспортом, реализующим трейты из крейта embedded-io.

```toml
reqwless = { version = "0.13.0", default-features = false, features = [
    "embedded-tls",
] }
```
