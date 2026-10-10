{{#title Режим точки доступа ESP32 и веб-сервер на embedded Rust}}

# Точка доступа: создание Wi-Fi-сети на ESP32

До сих пор мы использовали существующую Wi-Fi-сеть. Однако с помощью ESP32 можно создать собственную Wi-Fi-сеть (только не ждите, что она даст доступ к интернету). В этом упражнении мы настроим ESP32 как точку доступа и запустим веб-сервер.


## Базовый проект

Снова скопируем проект webserver-base и будем работать на его основе. 

Рекомендую сначала прочитать следующий раздел, прежде чем двигаться дальше: так мы избежим ненужных повторений кода и объяснений.
- [Создание веб-сервера](../web-server/index.md) 
 

```sh
git clone https://github.com/ImplFerris/esp32-projects
cp -r esp32-projects/webserver-base ~/YOUR_PROJECT_FOLDER/wifi-led
```

## Настройка Wi-Fi ESP32 (в файле wifi.rs)
 
Чтобы создать собственную Wi-Fi-сеть на ESP32, нужно задать статический IP-адрес в нотации CIDR (например, 192.168.2.1/24) и указать IP-адрес шлюза (например, 192.168.2.1). Также нужно задать имя Wi-Fi (SSID), которое может быть любым, но не длиннее 32 символов. Пароль для сети необязателен, но мы его настроим. SSID и пароль будут загружаться из переменных окружения, так же как мы делали в режиме станции.

```rust
// В отличие от режима станции, можно задать любой частный диапазон IP-адресов
// IP-адрес/маска подсети, например: STATIC_IP=192.168.13.37/24
const STATIC_IP: &str = "192.168.13.37/24";
// IP-адрес шлюза, например: GATEWAY_IP="192.168.13.37"
const GATEWAY_IP: &str = "192.168.13.37";

const PASSWORD: &str = env!("PASSWORD");
const SSID: &str = env!("SSID");
```

## Обновляем задачу подключения

Главная задача этой функции — убедиться, что Wi-Fi-сеть работает. Если она не работает, её нужно перезапустить. Функция в цикле проверяет состояние Wi-Fi. Если состояние `WifiApState::Started`, она ждёт, пока Wi-Fi не остановится (то есть пока не произойдёт событие `ApStop`). Если это случилось, выполнение переходит ко второй части цикла.

Если Wi-Fi не запущен, мы настроим его с SSID, необязательным паролем и режимом аутентификации WPA2 Personal. Затем запустим Wi-Fi-сеть.

**Примечание:** 

Чтобы запустить Wi-Fi без пароля, закомментируйте вызовы `password` и `auth_method` в `AccessPointConfig`. Тогда сеть будет открытой и будет использовать конфигурацию по умолчанию.


```rust
#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    println!("start connection task");
    println!("Device capabilities: {:?}", controller.capabilities());
    loop {
        match esp_radio::wifi::ap_state() {
            WifiApState::Started => {
                // ждём, пока соединение не будет разорвано
                controller.wait_for_event(WifiEvent::ApStop).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            _ => {}
        }

        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = ModeConfig::AccessPoint(
                AccessPointConfig::default()
                    .with_ssid(SSID.into())
                    .with_password(PASSWORD.into())
                    .with_auth_method(esp_radio::wifi::AuthMethod::Wpa2Personal),
            );
            controller.set_config(&client_config).unwrap();
            println!("Starting wifi");
            controller.start_async().await.unwrap();
            println!("Wifi started!");
        }
    }
}
```

## Обновляем функцию start_wifi

Нужно изменить интерфейс так, чтобы использовать режим точки доступа вместо режима станции.

```rust
let wifi_interface = interfaces.ap;
```
