{{#title Запуск веб-сайта на ESP32 на Rust и Embassy}}

# Пишем код на Rust для запуска веб-сайта на ESP32

В предыдущих упражнениях мы обращались к сайтам и выводили ответ в консоль. В этом упражнении сделаем наоборот: запустим веб-сервер на ESP32. Этот сервер будет доступен в локальной сети Wi-Fi. Чтобы открыть его из интернета, потребуется дополнительная настройка. Сначала сосредоточимся на доступе к сайту в локальной сети Wi-Fi. По-прежнему будем работать только в режиме STA (то есть подключаться к существующей сети Wi-Fi).

## Что мы будем делать
Настроим простой веб-сервер, который отдаёт одну страницу index.html. В этом примере будем считать, что ESP32 назначен IP-адрес «192.168.0.101» (он выводится в консоль во время работы). Когда сервер запущен, страницу можно открыть в браузере компьютера по адресу «http://192.168.0.101/». Можно использовать свою HTML-страницу или index.html, который я создал для этого упражнения и который можно найти [здесь](https://github.com/ImplFerris/esp32-projects/blob/main/webserver-html/src/bin/index.html).

> [!Note]
> Можно задать статический IP-адрес вместо того, чтобы его назначал DHCP-сервер. Так адрес будет постоянным, но добавятся дополнительные шаги. Чтобы не усложнять, в этом упражнении мы этого делать не будем. Настройку статического IP покажем в одном из следующих упражнений.

Хватит ждать, начнём прямо сейчас. 

## Создание проекта с помощью esp-generate

Чтобы создать проект, используйте команду `esp-generate`. Выполните:

```sh
esp-generate --chip esp32 webserver-html
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

### Крейт picoserve
[picoserve](https://docs.rs/picoserve/latest/picoserve/) — это крейт, который предоставляет асинхронный HTTP-сервер для bare-metal окружений и сильно вдохновлён Axum. Как можно догадаться по названию, изначально он создавался с расчётом на «Raspberry Pi Pico W» и Embassy. Но он прекрасно работает и с другими встраиваемыми рантаймами и железом, включая ESP32. Этот крейт сильно упрощает работу. Без него пришлось бы писать ядро веб-сервера с нуля, а это трудоёмкая задача, выходящая за рамки этой книги.

```toml
picoserve = { version = "0.17.1", features = ["embassy"] }
```

## Структура проекта

Разделим логику на модули. В lib создадим два подмодуля: web.rs и wifi.rs.

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

## Модуль lib

Перенесём макрос mk_static, который позволяет создавать статические переменные, инициализируемые во время выполнения, в модуль lib. Кроме того, включим функцию impl_trait_in_assoc_type, которую требует крейт picoserve.

```rust
#![no_std]
#![feature(impl_trait_in_assoc_type)]

pub mod web;
pub mod wifi;

#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}
```

## Функция main (файл main.rs)

Чтобы использовать модуль lib, обычно пришлось бы обращаться к нему по полному имени проекта (например, webserver::web). Однако чтобы ссылки оставались одинаковыми в разных упражнениях, мы дадим импорту псевдоним «lib». Так к нему можно обращаться как к «lib::web».

В функции main начинаем с типового кода: настраиваем глобальный аллокатор кучи и инициализируем Embassy.

Далее создаём контроллер Wi-Fi, который передаём функции start_wifi; её мы вскоре определим в модуле wifi. Эта функция вернёт экземпляр сетевого стека.

Создадим экземпляр веб-приложения и настроим маршрутизацию и параметры с помощью крейта picoserve. Затем запустим несколько задач для обработки входящих запросов в соответствии с заданным размером пула. Каждая задача получает идентификатор, экземпляр приложения, сетевой стек и настройки сервера.

```rust
use webserver_html as lib;

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.0.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 98767);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    info!("Embassy initialized!");

    let radio_init = &*lib::mk_static!(
        esp_radio::Controller<'static>,
        esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller")
    );
    let rng = Rng::new();

    let stack = lib::wifi::start_wifi(radio_init, peripherals.WIFI, rng, &spawner).await;

    let web_app = lib::web::WebApp::default();
    for id in 0..lib::web::WEB_TASK_POOL_SIZE {
        spawner.must_spawn(lib::web::web_task(
            id,
            stack,
            web_app.router,
            web_app.config,
        ));
    }

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
```
