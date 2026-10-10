{{#title Статический IP-адрес на ESP32 на Rust}}

# Статический IP-адрес

В предыдущих упражнениях мы полагались на DHCP-сервер, который назначал ESP32 IP-адрес. Однако это может быть ненадёжно, так как IP-адрес со временем может измениться. Во многих случаях нужно назначить статический IP-адрес, чтобы у ESP32 всегда был фиксированный, предсказуемый адрес. Так к устройству проще обращаться постоянно, не проверяя и не обновляя его IP-адрес при каждом переподключении к сети.

Теперь удалим код, связанный с DHCP, и изменим переменную net_config так, чтобы она инициализировалась статическим IP-адресом.

Сначала определим константы, которые будут загружать статический IP-адрес и IP-адрес шлюза из переменных окружения. IP-адрес должен быть в формате CIDR, который включает и сам IP-адрес, и маску подсети. Нужно указать IP-адрес, затем косую черту и маску подсети. Например, если хотите назначить ESP32 адрес 192.168.0.50, запишите его как 192.168.0.50/24.

> [!Tip]
> Назначить можно не любой IP-адрес. Нужно узнать диапазон адресов, который использует ваш Wi-Fi-роутер. Для этого введите в терминале `ip a` и найдите IP-адрес рядом с Wi-Fi-интерфейсом (обычно он начинается с `wl`). Например, если IP-адрес вашей системы 192.168.0.103, можно назначить адрес, начиная с 192.168.0.2

## Базовый проект
Скопируйте тот же проект, который мы создали в предыдущей главе.

```sh
cp -r webserver-html webserver-base
```


```rust
// IP-адрес/маска подсети, например: STATIC_IP=192.168.0.50/24
const STATIC_IP: &str = env!("STATIC_IP");
const GATEWAY_IP: &str = env!("GATEWAY_IP");
```

Шлюз тоже нужно настроить. Для этого упражнения он не обязателен, так как мы не будем отправлять запросы в интернет, но для будущих упражнений полезно настроить его заранее. 

Адрес шлюза часто является первым адресом в диапазоне IP-адресов вашей Wi-Fi-сети. Например, если адреса находятся в диапазоне от 192.168.0.1 до 192.168.0.255, шлюз, скорее всего, будет 192.168.0.1. Узнать адрес шлюза в Linux можно также командой `ip route | grep default`.

В модуле wifi.rs, который мы создали в предыдущей главе, удалите конфигурацию DHCP и замените её в функции start_wifi следующим кодом:

```rust
// Дополнительные импорты
use core::str::FromStr;
use core::net::Ipv4Addr;
use embassy_net::Ipv4Cidr;
```

```rust
//найдите фрагмент `let net_config` и замените его
let Ok(ip_addr) = Ipv4Cidr::from_str(STATIC_IP) else {
    println!("Invalid STATIC_IP");
    loop {}
};

let Ok(gateway) = Ipv4Addr::from_str(GATEWAY_IP) else {
    println!("Invalid GATEWAY_IP");
    loop {}
};

let net_config = embassy_net::Config::ipv4_static(StaticConfigV4 {
    address: ip_addr,
    gateway: Some(gateway),
    dns_servers: Vec::new(),
});
// В вызове `embassy_net::new` ничего менять не нужно.
```

## Обновлённая функция start_wifi

```rust
pub async fn start_wifi(
    radio_init: &'static esp_radio::Controller<'static>,
    wifi: esp_hal::peripherals::WIFI<'static>,
    rng: Rng,
    spawner: &Spawner,
) -> Stack<'static> {
    let (wifi_controller, interfaces) = esp_radio::wifi::new(radio_init, wifi, Default::default())
        .expect("Failed to initialize Wi-Fi controller");

    let wifi_interface = interfaces.sta;
    let net_seed = rng.random() as u64 | ((rng.random() as u64) << 32);

    let Ok(ip_addr) = Ipv4Cidr::from_str(STATIC_IP) else {
        println!("Invalid STATIC_IP");
        loop {}
    };

    let Ok(gateway) = Ipv4Addr::from_str(GATEWAY_IP) else {
        println!("Invalid GATEWAY_IP");
        loop {}
    };

    let net_config = embassy_net::Config::ipv4_static(StaticConfigV4 {
        address: ip_addr,
        gateway: Some(gateway),
        dns_servers: Default::default(),
    });
    // Инициализируем сетевой стек
    let (stack, runner) = embassy_net::new(
        wifi_interface,
        net_config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        net_seed,
    );

    spawner.spawn(connection(wifi_controller)).ok();
    spawner.spawn(net_task(runner)).ok();

    wait_for_connection(stack).await;

    stack
}
```

## Базовый проект

Я реорганизовал проект, разбив его на модули, например wifi и web, чтобы основной файл оставался аккуратным. Этот проект станет основой для следующего упражнения, поэтому советую посмотреть, как он устроен.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/webserver-base
```

**Структура проекта**:

```
├── build.rs
├── Cargo.toml
├── rust-toolchain.toml
├── src
│   ├── bin
│   │   └── main.rs
│   ├── index.html
│   ├── lib.rs
│   ├── web.rs
│   └── wifi.rs
```

### Как запустить?

Обычно мы просто запускаем `cargo run --release`, но на этот раз нужно также передать переменные окружения для подключения к Wi-Fi.

```sh
SSID=YOUR_WIFI_NAME PASSWORD=YOUR_WIFI_PASSWORD STATIC_IP=ASSIGN_ESP32_IP/24 GATEWAY_IP=WIFI_GATEWAY_IP  cargo run --release
```
