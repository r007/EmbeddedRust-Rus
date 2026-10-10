{{#title Управление светодиодом ESP32 по Wi-Fi через браузер на Rust}}

# Управление светодиодом ESP32 через веб-страницу

Мы создадим простую конечную точку API, которая принимает логическое значение и управляет состоянием светодиода. Кроме того, будет отдаваться страница «index.html» с двумя кнопками: одна включает светодиод, другая выключает его.

При нажатии одной из кнопок на конечную точку «/led» будет отправлен запрос с таким JSON-телом:

- `{ "is_on": true }` — включить светодиод
- `{ "is_on": false }` — выключить светодиод

В зависимости от значения поля «is_on» обновляется переменная «LED_STATE» модуля led. Затем «led_task» соответственно включает или выключает светодиод.

## Маршрутизация

В функции «build_app» настраиваются веб-маршруты приложения. Корневой путь («/») отдаёт содержимое «index.html», поэтому этот файл нужно поместить в папку «src/». Путь «/led» принимает POST-запросы, которые обрабатывает «led_handler».

```rust
pub struct Application;

impl AppBuilder for Application {
    type PathRouter = impl routing::PathRouter;

    fn build_app(self) -> picoserve::Router<Self::PathRouter> {
        picoserve::Router::new()
            .route(
                "/",
                routing::get_service(File::html(include_str!("index.html"))),
            )
            .route("/led", routing::post(led_handler))
    }
}
```

## Обработчик светодиода

Мы определим две структуры: одну для входящих данных и одну для ответа. Структура LedRequest получит derive Deserialize, чтобы разбирать входящий JSON и представлять его в виде экземпляра структуры. Структура LedResponse получит derive Serialize, чтобы преобразовывать экземпляр структуры и отправлять его как JSON-ответ.

```rust
#[derive(serde::Deserialize)]
struct LedRequest {
    is_on: bool,
}

#[derive(serde::Serialize)]
struct LedResponse {
    success: bool,
}
```

В функции led_handler структура LedRequest извлекается как параметр. Значение «is_on» можно сразу сохранить в LED_STATE, так как оба значения логические. Наконец, обработчик возвращает JSON-ответ с LedResponse, в котором указывается успешное выполнение.

```rust
async fn led_handler(input: picoserve::extract::Json<LedRequest>) -> impl IntoResponse {
    crate::led::LED_STATE.store(input.0.is_on, Ordering::Relaxed);

    picoserve::response::Json(LedResponse { success: true })
}
```


## Содержимое веб-страницы

Файл index.html можно скачать [здесь](https://github.com/ImplFerris/esp32-projects/blob/main/wifi-led/src/index.html) и поместить в папку «src/», либо создать собственное содержимое, которое отправляет JSON-запросы.

**Примечание:**

Нужно заменить URL «http://192.168.0.50/led» на IP-адрес вашего ESP32. Для простоты я прописал адрес прямо в коде; иначе пришлось бы использовать заглушку, которую заменяли бы динамически, или подход на основе шаблонов.

```html
<div class="button-container">
        <button class="btn-on" onclick="sendRequest(true)">Turn on LED</button>
        <button class="btn-off" onclick="sendRequest(false)">Turn off LED</button>
    </div>

    <script>
        function sendRequest(is_on) {
            const url = 'http://192.168.0.50/led'; // Замените на статический IP-адрес ESP32

            fetch(url, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ is_on })
            })
            .then(response => {
                if (response.ok) {
                    return response.json();
                }
                throw new Error('Network response was not ok');
            })
            .then(data => {
                console.log('Success:', data);
                //alert(LED turned ${action});
            })
            .catch(error => {
                console.error('Error:', error);
                alert('Failed to send the request');
            });
        }
    </script>
```



## Клонирование существующего проекта
Вы также можете клонировать (или посмотреть) созданный мной проект и перейти в папку `wifi-led`.

```sh
git clone https://github.com/ImplFerris/esp32-projects
cd esp32-projects/wifi-led
```

### Как запустить?

Передайте имя Wi-Fi, пароль, статический IP-адрес и IP-адрес шлюза как переменные окружения, а затем прошейте ESP32

```sh
SSID=YOUR_WIFI_NAME PASSWORD=YOUR_WIFI_PASSWORD STATIC_IP=ASSIGN_ESP32_IP/24 GATEWAY_IP=WIFI_GATEWAY_IP  cargo run --release
```
