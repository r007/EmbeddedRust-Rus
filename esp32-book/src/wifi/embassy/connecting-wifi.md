{{#title Асинхронная работа с Wi-Fi на ESP32 с Embassy}}

# Подключение ESP32 к Wi-Fi с поддержкой Embassy

Итак, мы обсудили зависимости, которые нужны для этого упражнения, и функции, которые нужно включить. Теперь перейдём к коду. Сначала посмотрим код, который подключает Wi-Fi с помощью Embassy.

## Вспомогательный макрос для StaticCell

Во встраиваемой среде крейт StaticCell полезен, когда нужно инициализировать переменную во время выполнения, но при этом требуется, чтобы она имела статическое время жизни. Мы определим макрос, который создаёт глобально доступные статические переменные. Макрос принимает два аргумента: тип переменной и значение, которым её нужно инициализировать. Функция uninit возвращает изменяемую ссылку на неинициализированную память, и мы записываем в неё значение.

```rust
// Если устраивает nightly-компилятор, можно использовать макрос из крейта static_cell: https://docs.rs/static_cell/2.1.0/static_cell/macro.make_static.html

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}
```

## Инициализация контроллера Wi-Fi

Инициализируем Embassy обычным образом:

```rust
let timg0 = TimerGroup::new(peripherals.TIMG0);
esp_rtos::start(timg0.timer0);
```

Загрузите учётные данные Wi-Fi из переменных окружения:
```rust
const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");
```

Далее инициализируем контроллер радиомодуля со статическим временем жизни. Мы используем макрос `mk_static!`, чтобы сделать контроллер `'static`, потому что стек Wi-Fi работает как асинхронная задача, которая непрерывно обрабатывает сетевые события на протяжении всей работы программы. Поэтому переменная `radio_init` должна оставаться действительной всё время работы программы.

```rust
// let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
let radio_init = &*mk_static!(
    esp_radio::Controller<'static>,
    esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller")
);
```

Далее создаём контроллер Wi-Fi вместе с его сетевыми интерфейсами, а затем выделяем интерфейс STA.

```rust
let (wifi_controller, interfaces) =
        esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");
let wifi_interface = interfaces.sta;
```

### Инициализация сетевого стека

Случайное число нужно и для конфигурации TLS, и для инициализации сетевого стека, и оба требуют u64. Однако rng генерирует только значения u32, поэтому мы получаем два случайных числа и с помощью побитовой операции помещаем одно в старшие биты (MSB), а другое — в младшие (LSB):

```rust
let rng = Rng::new();
let net_seed = rng.random() as u64 | ((rng.random() as u64) << 32);
let tls_seed = rng.random() as u64 | ((rng.random() as u64) << 32);
```

Инициализируем сетевой стек из крейта embassy_net, используя сетевой интерфейс, полученный от контроллера Wi-Fi, случайное число в качестве seed, конфигурацию DHCP и ресурсы стека размером 3.

```rust
 let dhcp_config = DhcpConfig::default();
// dhcp_config.hostname = Some(String::from_str("implRust").unwrap());

let config = embassy_net::Config::dhcpv4(dhcp_config);
// Инициализируем сетевой стек
let (stack, runner) = embassy_net::new(
    wifi_interface,
    config,
    mk_static!(StackResources<3>, StackResources::<3>::new()),
    net_seed,
);
```

Далее запустим две фоновые задачи: connection_task будет поддерживать подключение к Wi-Fi, а net_task будет запускать сетевой стек и обрабатывать сетевые события.

```rust
spawner.spawn(connection(controller)).ok();
spawner.spawn(net_task(runner)).ok();
```

Вскоре мы разберём, что происходит в этих двух задачах, и посмотрим на определения функций. Но сначала завершим общий ход работы.

## Ожидание подключения к Wi-Fi

Будем ждать, пока соединение Wi-Fi не поднимется, а затем получим IP-адрес.

```rust
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


## Задачи подключения к Wi-Fi

Функция connection_task управляет подключением к Wi-Fi: она непрерывно проверяет состояние, настраивает контроллер Wi-Fi и пытается переподключиться, если соединение потеряно или Wi-Fi не запущен.

1. Сначала проверяем состояние Wi-Fi. Если оно StaConnected, ждём, пока соединение не будет разорвано. Если соединение разорвано, переходим к остальным шагам цикла.
2. Проверяем, запущен ли контроллер Wi-Fi. Если нет, инициализируем клиентскую конфигурацию Wi-Fi с SSID (именем сети) и паролем и запускаем его.
3. Наконец, пытаемся подключиться к Wi-Fi.

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

### Запуск сетевого стека 

```rust
#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}
```
