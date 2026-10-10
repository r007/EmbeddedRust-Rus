{{#title Как подключить ESP32 к Wi-Fi и открыть сайт на Rust}}

# Как подключить ESP32 к Wi-Fi и открыть сайт

В этом упражнении мы настроим ESP32 в режиме STA для подключения к вашей Wi-Fi-сети. Затем отправим запрос с ESP32 на веб-сервер и выведем ответ в системную консоль. Этот код основан на примерах esp-hal.

### Создание проекта с помощью esp-generate

Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 wifi-webfetch
```

Откроется экран с вопросом о выборе опций. Чтобы включить Wi-Fi, сначала нужно включить функции «unstable» и «alloc». Как вы могли заметить, пока не выбраны эти две опции, пункт Wi-Fi недоступен. Поэтому выбирайте их по одной:

- Сначала выберите опцию «Enable unstable HAL features».
- Выберите опцию «Enable allocations via the esp-alloc crate».
- Теперь можно включить «Enable Wi-Fi via the esp-radio crate».


Также включите логирование.

- Прокрутите до пункта «Flashing, logging and debugging (espflash)» и нажмите Enter.
- Затем выберите «Use defmt to print messages».

Сохраните настройки, нажав на клавиатуре «s».

### Обновляем зависимости

Добавьте в файл Cargo.toml следующие зависимости:
```toml
blocking-network-stack = { git = "https://github.com/bjoernQ/blocking-network-stack.git", rev = "b3ecefc222d8806edd221f266999ca339c52d34e", default-features = false, features = [
  "dhcpv4",
  "tcp",
] }

heapless = { version = "0.9.1" }
```

blocking-network-stack — это крейт, который предоставляет синхронные (не асинхронные) сетевые примитивы для TCP/UDP-обмена.

Из-за конфликта версий я понизил embedded-io до версии 0.6.1 (вместо 0.7.1, которую сгенерировал esp-generate).

```toml
embedded-io = { version = "0.6.1" }
```

### Крейт esp-alloc

«Простой аллокатор кучи no_std для процессоров RISC-V и Xtensa от Espressif. Поддерживает все доступные на данный момент устройства ESP32». Крейт esp-alloc предоставляет глобальный аллокатор для SoC ESP.

## Инициализация периферии

Создадим вспомогательную функцию, которая инициализирует ESP HAL и возвращает экземпляр периферии. Также настроим кучу размером 72 КиБ с помощью макроса esp_alloc::heap_allocator!.

```rust
fn init_hardware() -> Peripherals {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(size: 72 * 1024);
    peripherals
}
```

## Инициализация контроллера Wi-Fi

Программе нужно передать имя Wi-Fi и пароль. Вместо того чтобы жёстко прописывать эти учётные данные в коде, загрузим их из переменных окружения. При запуске программы передадим переменные окружения вместе с ней.

```rust
const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");
```

Сначала инициализируем TimerGroup и генератор случайных чисел (RNG), которые нужны для настройки контроллера Wi-Fi. RNG также используется для инициализации синхронного стека TCP/IP, поэтому используем clone, чтобы можно было переиспользовать его. 

```rust
let timg0 = TimerGroup::new(peripherals.TIMG0);
let rng = Rng::new();
```

Инициализируем контроллер Wi-Fi, используя аппаратный таймер, RNG и периферию тактирования. Затем создаём экземпляр драйвера Wi-Fi, который управляет сетевыми подключениями и различными интерфейсами. Наконец, настраиваем устройство на работу в режиме станции (STA), чтобы оно могло подключаться к сетям Wi-Fi как клиент.

```rust
esp_rtos::start(timg0.timer0);
let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");

let (mut wifi_controller, interfaces) =
    esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
        .expect("Failed to initialize Wi-Fi controller");

let mut device = interfaces.sta;
```


### Инициализация SocketSet

Создадим SocketSet с местом для трёх сокетов, чтобы управлять несколькими сокетами внутри стека, например DHCP и TCP.

```rust
let mut socket_set_entries: [SocketStorage; 3] = Default::default();
let mut socket_set = SocketSet::new(&mut socket_set_entries[..]);
```

### DHCP-сокет

Создадим DHCP-сокет, чтобы запросить IP-адрес у DHCP-сервера. Можно задать исходящие опции, в том числе имя хоста, через DHCP Client Option 12 (подробнее о DHCP-опциях — [здесь](https://efficientip.com/glossary/dhcp-option/)). Добавим DHCP-сокет в SocketSet, который создали ранее. 

```rust
let mut dhcp_socket = smoltcp::socket::dhcpv4::Socket::new();
// здесь можно задать имя хоста (или добавить другие DHCP-опции)
dhcp_socket.set_outgoing_options(&[DhcpOption {
    kind: 12,
    data: b"implRust",
}]);
socket_set.add(dhcp_socket);
// socket_set.add(smoltcp::socket::dhcpv4::Socket::new());
```

### Инициализация сетевого стека

Инициализируем Stack из крейта smoltcp, используя сетевой интерфейс и устройство (которые мы получили из Wi-Fi-интерфейса, созданного ранее), сокеты, замыкание для получения времени и случайное число.


```rust
let now = || Instant::now().duration_since_epoch().as_millis();
let mut stack = Stack::new(
    create_interface(&mut device),
    device,
    socket_set,
    now,
    rng.random(),
);
```

Шаблонные функции для настройки сетевого интерфейса стека TCP/IP smoltcp с Wi-Fi-устройством (взяты из официальных примеров esp-hal)

```rust
pub fn create_interface(device: &mut esp_wifi::wifi::WifiDevice) -> smoltcp::iface::Interface {
    // можно создать несколько экземпляров, но так как WifiDevice один, сделать с ним что-то плохое вряд ли получится
    // скорее всего, с ним нельзя сделать ничего плохого
    smoltcp::iface::Interface::new(
        smoltcp::iface::Config::new(smoltcp::wire::HardwareAddress::Ethernet(
            smoltcp::wire::EthernetAddress::from_bytes(&device.mac_address()),
        )),
        device,
        timestamp(),
    )
}

// шаблонный код smoltcp
fn timestamp() -> smoltcp::time::Instant {
    smoltcp::time::Instant::from_micros(
        esp_hal::time::Instant::now()
            .duration_since_epoch()
            .as_micros() as i64,
    )
}
```


**Режим работы Wi-Fi**:

Далее настраиваем режим работы Wi-Fi с помощью перечисления ModeConfig. Поскольку ESP32 должен работать как Wi-Fi-клиент, используем вариант Client. Нужно указать SSID (идентификатор сети, то есть имя вашей сети Wi-Fi) и пароль Wi-Fi.

```rust
fn configure_wifi(controller: &mut WifiController<'_>) {
    controller
        .set_power_saving(esp_radio::wifi::PowerSaveMode::None)
        .unwrap();

    let client_config = ModeConfig::Client(
        ClientConfig::default()
            .with_ssid(SSID.into())
            .with_password(PASSWORD.into()),
    );
    let res = controller.set_config(&client_config);
    println!("wifi_set_configuration returned {:?}", res);

    controller.start().unwrap();
    println!("is wifi started: {:?}", controller.is_started());
}
```

## Сканирование Wi-Fi и подключение к точке доступа

Далее выполняем блокирующее сканирование Wi-Fi-сетей с параметрами по умолчанию. Это значит, что программа остановится и будет ждать, пока сканирование не завершится. Когда сканирование закончится, будет возвращён список доступных точек доступа Wi-Fi. Затем мы проходим по результатам и выводим информацию о каждой точке доступа в консоль.

```rust
fn scan_wifi(controller: &mut WifiController<'_>) {
    println!("Start Wifi Scan");
    let scan_config = ScanConfig::default().with_max(10);
    let res = controller.scan_with_config(scan_config).unwrap();
    for ap in res {
        println!("{:?}", ap);
    }
}
```

## Подключение к Wi-Fi

Затем выводим поддерживаемые возможности контроллера: будет показан только «Client», так как именно его мы и настроили.

После этого вызываем метод connect у контроллера Wi-Fi. Это подключит ESP32 к сети Wi-Fi, которую мы указали ранее.

В цикле будем постоянно проверять, подключён ли Wi-Fi. Как только подключение установится, перейдём дальше.


```rust
fn connect_wifi(controller: &mut WifiController<'_>) {
    println!("{:?}", controller.capabilities());
    println!("wifi_connect {:?}", controller.connect());

    println!("Wait to get connected");
    loop {
        match controller.is_connected() {
            Ok(true) => break,
            Ok(false) => {}
            Err(err) => panic!("{:?}", err),
        }
    }
    println!("Connected: {:?}", controller.is_connected());
}
```
 

### Получение IP-адреса

Нужно получить IP-адрес, поэтому сначала ждём завершения процесса DHCP. Будем в цикле проверять, поднялся ли интерфейс и получили ли мы IP-адрес.

```rust
fn obtain_ip(stack: &mut Stack<'_, esp_radio::wifi::WifiDevice<'_>>) {
    println!("Wait for IP address");
    loop {
        stack.work();
        if stack.is_iface_up() {
            println!("IP acquired: {:?}", stack.get_ip_info());
            break;
        }
    }
}
```

На этом этапе ESP32 должен быть успешно подключён к сети Wi-Fi. Далее настроим код, который отправит HTTP-запрос к сайту. После отправки запроса ESP32 получит HTML-ответ от сайта, и мы выведем HTML-содержимое в консоль.


## HTTP-клиент

Чтобы отправить веб-запрос на сайт, нужен HTTP-клиент. Для этого мы используем [крейт smoltcp](https://crates.io/crates/smoltcp), чтобы отправить «сырой» HTTP-запрос через TCP-сокет. 

«smoltcp — это самостоятельный управляемый событиями стек TCP/IP, предназначенный для bare-metal и систем реального времени. smoltcp вообще не требует выделения кучи, обширно документирован и компилируется на стабильном Rust 1.80 и новее».


### TCP-сокет

Инициализируем TCP-сокет с буферами чтения и записи для входящих и исходящих данных сокета.

```rust
let mut rx_buffer = [0u8; 1536];
let mut tx_buffer = [0u8; 1536];
let socket = stack.get_socket(&mut rx_buffer, &mut tx_buffer);

http_loop(socket)
```

## Цикл HTTP: отправка GET-запроса

Отправим HTTP-запрос методом GET на сайт «www.mobile-j.de». Этот сайт используется в примерах esp-hal: он возвращает «Hello fellow Rustaceans!» с ASCII-арт Ferris. Откроем соединение на порту 80 по IP-адресу сайта и затем отправим HTTP-запрос.

```rust
println!("Making HTTP request");
socket.work();

let remote_addr = IpAddress::v4(142, 250, 185, 115);
socket.open(remote_addr, 80).unwrap();
socket
    .write(b"GET / HTTP/1.0\r\nHost: www.mobile-j.de\r\n\r\n")
    .unwrap();
socket.flush().unwrap();
```

### Чтение ответа

Далее читаем ответ сервера с таймаутом 20 секунд. 

```rust
let deadline = Instant::now() + Duration::from_secs(20);
let mut buffer = [0u8; 512];
while let Ok(len) = socket.read(&mut buffer) {
    // let text = unsafe { core::str::from_utf8_unchecked(&buffer[..len]) };
    let Ok(text) = core::str::from_utf8(&buffer[..len]) else {
        panic!("Invalid UTF-8 sequence encountered");
    };

    println!("{}", text);

    if Instant::now() > deadline {
        println!("Timeout");
        break;
    }
}
```

### Закрытие сокета
После завершения HTTP-запроса закрываем сокет и ждём 5 секунд.

```rust
socket.disconnect();
let deadline = Instant::now() + Duration::from_secs(5);
while Instant::now() < deadline {
    socket.work();
}
```


## Клонирование существующего проекта

Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `wifi-webfetch`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/wifi-webfetch
```

### Как запустить?

Обычно мы просто запускаем `cargo run --release`, но на этот раз нужно также передать переменные окружения для подключения к Wi-Fi.

```sh
SSID=YOUR_WIFI_NAME PASSWORD=YOUR_WIFI_PASSWORD  cargo run --release
```

## Полный код
```rust
#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use blocking_network_stack::Stack;
use embedded_io::{Read, Write};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::peripherals::Peripherals;
use esp_hal::rng::Rng;
use esp_hal::time::{Duration, Instant};
use esp_hal::timer::timg::TimerGroup;
use esp_println::{self as _, println};
use esp_radio::wifi::{ClientConfig, ModeConfig, ScanConfig, WifiController};
use smoltcp::iface::{SocketSet, SocketStorage};
use smoltcp::wire::{DhcpOption, IpAddress};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// Создаёт дескриптор приложения по умолчанию, который требуется загрузчику esp-idf.
// Подробнее см.: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

#[main]
fn main() -> ! {
    // generator version: 1.0.0

    let peripherals = init_hardware();

    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 98767);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let rng = Rng::new();

    esp_rtos::start(timg0.timer0);
    let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");

    let (mut wifi_controller, interfaces) =
        esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");

    let mut device = interfaces.sta;

    // let mut stack = setup_network_stack(device, &mut rng);
    let mut socket_set_entries: [SocketStorage; 3] = Default::default();
    let mut socket_set = SocketSet::new(&mut socket_set_entries[..]);
    let mut dhcp_socket = smoltcp::socket::dhcpv4::Socket::new();

    // здесь можно задать имя хоста (или добавить другие DHCP-опции)
    dhcp_socket.set_outgoing_options(&[DhcpOption {
        kind: 12,
        data: b"implRust",
    }]);
    socket_set.add(dhcp_socket);
    // sta_socket_set.add(smoltcp::socket::dhcpv4::Socket::new());

    let now = || Instant::now().duration_since_epoch().as_millis();
    let mut stack = Stack::new(
        create_interface(&mut device),
        device,
        socket_set,
        now,
        rng.random(),
    );

    configure_wifi(&mut wifi_controller);
    scan_wifi(&mut wifi_controller);
    connect_wifi(&mut wifi_controller);
    obtain_ip(&mut stack);

    let mut rx_buffer = [0u8; 1536];
    let mut tx_buffer = [0u8; 1536];
    let socket = stack.get_socket(&mut rx_buffer, &mut tx_buffer);

    http_loop(socket)
}

pub fn create_interface(device: &mut esp_radio::wifi::WifiDevice) -> smoltcp::iface::Interface {
    // можно создать несколько экземпляров, но так как WifiDevice один, сделать с ним что-то плохое вряд ли получится
    // скорее всего, с ним нельзя сделать ничего плохого
    smoltcp::iface::Interface::new(
        smoltcp::iface::Config::new(smoltcp::wire::HardwareAddress::Ethernet(
            smoltcp::wire::EthernetAddress::from_bytes(&device.mac_address()),
        )),
        device,
        timestamp(),
    )
}

// шаблонный код smoltcp
fn timestamp() -> smoltcp::time::Instant {
    smoltcp::time::Instant::from_micros(
        esp_hal::time::Instant::now()
            .duration_since_epoch()
            .as_micros() as i64,
    )
}

fn init_hardware() -> Peripherals {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(size: 72 * 1024);
    peripherals
}

fn configure_wifi(controller: &mut WifiController<'_>) {
    controller
        .set_power_saving(esp_radio::wifi::PowerSaveMode::None)
        .unwrap();

    let client_config = ModeConfig::Client(
        ClientConfig::default()
            .with_ssid(SSID.into())
            .with_password(PASSWORD.into()),
    );
    let res = controller.set_config(&client_config);
    println!("wifi_set_configuration returned {:?}", res);

    controller.start().unwrap();
    println!("is wifi started: {:?}", controller.is_started());
}

fn scan_wifi(controller: &mut WifiController<'_>) {
    println!("Start Wifi Scan");
    let scan_config = ScanConfig::default().with_max(10);
    let res = controller.scan_with_config(scan_config).unwrap();
    for ap in res {
        println!("{:?}", ap);
    }
}

fn connect_wifi(controller: &mut WifiController<'_>) {
    println!("{:?}", controller.capabilities());
    println!("wifi_connect {:?}", controller.connect());

    println!("Wait to get connected");
    loop {
        match controller.is_connected() {
            Ok(true) => break,
            Ok(false) => {}
            Err(err) => panic!("{:?}", err),
        }
    }
    println!("Connected: {:?}", controller.is_connected());
}

fn obtain_ip(stack: &mut Stack<'_, esp_radio::wifi::WifiDevice<'_>>) {
    println!("Wait for IP address");
    loop {
        stack.work();
        if stack.is_iface_up() {
            println!("IP acquired: {:?}", stack.get_ip_info());
            break;
        }
    }
}

fn http_loop(
    mut socket: blocking_network_stack::Socket<'_, '_, esp_radio::wifi::WifiDevice<'_>>,
) -> ! {
    println!("Starting HTTP client loop");
    let delay = Delay::new();
    loop {
        println!("Making HTTP request");
        socket.work();

        let remote_addr = IpAddress::v4(142, 250, 185, 115);
        socket.open(remote_addr, 80).unwrap();
        socket
            .write(b"GET / HTTP/1.0\r\nHost: www.mobile-j.de\r\n\r\n")
            .unwrap();
        socket.flush().unwrap();

        let deadline = Instant::now() + Duration::from_secs(20);
        let mut buffer = [0u8; 512];
        while let Ok(len) = socket.read(&mut buffer) {
            // let text = unsafe { core::str::from_utf8_unchecked(&buffer[..len]) };
            let Ok(text) = core::str::from_utf8(&buffer[..len]) else {
                panic!("Invalid UTF-8 sequence encountered");
            };

            println!("{}", text);

            if Instant::now() > deadline {
                println!("Timeout");
                break;
            }
        }

        socket.disconnect();
        let deadline = Instant::now() + Duration::from_secs(5);
        while Instant::now() < deadline {
            socket.work();
        }

        delay.delay_millis(1000);
    }
}
```
