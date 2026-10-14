{{#title Модуль подключения ESP32 к Wi-Fi на Embassy и Rust}}

# Модуль Wi-Fi


## Настройка подключения Wi-Fi

Код настройки Wi-Fi такой же, как описан в главе ["Подключение ESP32 к Wi-Fi с поддержкой Embassy"](../embassy/connecting-wifi.md) (в разделе про доступ к сайту). Чтобы не повторяться, я не буду объяснять его здесь снова. Если вы ещё не читали ту главу, загляните в неё.

```rust
const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");
```

## Запуск Wi-Fi

Функция `start_wifi` отвечает за настройку и запуск подключения Wi-Fi для ESP32. Вот пошаговое объяснение:

1. Создадим интерфейс и контроллер Wi-Fi в режиме STA (Station), чтобы ESP32 мог подключиться к существующей сети Wi-Fi.

2. Обычно, когда устройство подключается к сети Wi-Fi, роутер автоматически назначает ему IP-адрес через DHCP-сервер. В этом упражнении настроим ESP32 так, чтобы он запрашивал IP-адрес через DHCP.  
   Для этого создадим экземпляр `net_config`, настроенный на DHCP. Затем используем эту конфигурацию вместе с Wi-Fi-интерфейсом для инициализации сетевого стека и runner.

3. Запустим две задачи:
   - **Задача connection** следит за подключением Wi-Fi и переподключает его, если соединение разорвано.
   - **Задача network** управляет всем сетевым обменом данными.

4.  Подождём, пока соединение Wi-Fi не поднимется. Когда подключение будет готово, выведем IP-адрес, назначенный нашему ESP32.

5.  Наконец, вернём экземпляр сетевого стека. Далее его использует веб-задача для работы веб-сервера.

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

    let dhcp_config = DhcpConfig::default();
    let net_config = embassy_net::Config::dhcpv4(dhcp_config);

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

async fn wait_for_connection(stack: Stack<'_>) {
    println!("Waiting for link to be up");
    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    println!("Waiting to get IP address...");
    loop {
        if let Some(config) = stack.config_v4() {
            println!("Got IP: {}", config.address);
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}
```


### Задачи Wi-Fi и сети

В логике этих двух задач нет существенных изменений. Единственное отличие: теперь в net_task передаётся экземпляр runner, а раньше этого не было.

```rust
#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    println!("start connection task");
    println!("Device capabilities: {:?}", controller.capabilities());
    loop {
        match esp_radio::wifi::sta_state() {
            WifiStaState::Connected => {
                // ждём, пока соединение не будет разорвано
                controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = ModeConfig::Client(
                ClientConfig::default()
                    .with_ssid(SSID.into())
                    .with_password(PASSWORD.into()),
            );
            controller.set_config(&client_config).unwrap();
            println!("Starting wifi");
            controller.start_async().await.unwrap();
            println!("Wifi started!");

            println!("Scan");
            let scan_config = ScanConfig::default().with_max(10);
            let result = controller
                .scan_with_config_async(scan_config)
                .await
                .unwrap();
            for ap in result {
                println!("{:?}", ap);
            }
        }
        println!("About to connect...");

        match controller.connect_async().await {
            Ok(_) => println!("Wifi connected!"),
            Err(e) => {
                println!("Failed to connect to wifi: {:?}", e);
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}
```

```rust
#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}
```
