{{#title Клиент погодного API для e-Ink панели на ESP32 на Rust}}

# Погодное API

В этой главе мы научимся отправлять запрос к API и десериализовать JSON-ответ в структуру.

Надеюсь, вы получили API-ключ на сайте openweathermap. Его можно передать через переменную окружения, поэтому определим константу так:

```rust
const API_KEY: &str = env!("API_KEY");
```

## Из JSON в структуру

Советую изучить документацию API по адресу «https://openweathermap.org/current» и познакомиться со структурой JSON-ответа. На верхнем уровне я определил структуру `WeatherData` с полями `weather`, `main` и `wind`, каждое из которых использует собственные типы структур. Чтобы не загромождать руководство, определения остальных структур приведены в конце главы.


```rust
#[derive(Debug, Deserialize)]
pub struct WeatherData {
    pub weather: Vec<Weather, 4>,
    pub main: Main,
    pub wind: Wind,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub dt: DateTime<Utc>,
    pub name: String<20>,
}
```
Здесь мы десериализуем только нужные поля, а остальные данные JSON игнорируем.
 
## Погодный API

Создадим структуру-обёртку для обработки запросов к API. Хотя можно обойтись и простой функцией, тогда пришлось бы каждый раз передавать URL и Wi-Fi-стек. Используя структуру, мы создаём её один раз и вызываем метод `access_website` всякий раз, когда нужны свежие данные о погоде.

```rust
pub struct WeatherApi {
    wifi: Stack<'static>,
    url: String<120>,
    tls_seed: u64,
}
```

В функции `new` мы формируем URL API, добавляя к нему API-ключ, и сохраняем его в поле.

ПРИМЕЧАНИЕ: как вы могли заметить, я использовал «London» в качестве названия города. Вы можете заменить его на название своего города с кодом страны. Подробности смотрите в документации и при необходимости внесите изменения.

```rust
pub fn new(wifi: Stack<'static>, tls_seed: u64) -> Self {
    let mut url = String::new();
    url.push_str(
        "https://api.openweathermap.org/data/2.5/weather?q=London&units=metric&appid=",
    )
    .unwrap();
    url.push_str(API_KEY).unwrap();
    Self {
        wifi,
        url,
        tls_seed,
    }
}
```

## Отправка запроса к API

Далее создаём DNS-сокет и TCP-клиент. Когда базовая настройка готова, передаём TCP-клиент, DNS-сокет и конфигурацию TLS в `HttpClient` и с его помощью отправляем запрос по URL API. Получив ответ, десериализуем JSON-полезную нагрузку в структуру `WeatherData` с помощью `serde_json_core`.

```rust
pub async fn access_website(&self) -> WeatherData {
    let mut rx_buffer = [0; 4096 * 2];
    let mut tx_buffer = [0; 4096 * 2];

    let tls_config = TlsConfig::new(
        self.tls_seed,
        &mut rx_buffer,
        &mut tx_buffer,
        reqwless::client::TlsVerify::None,
    );

    let dns = DnsSocket::new(self.wifi);
    let tcp_state = TcpClientState::<1, 4096, 4096>::new();
    let tcp = TcpClient::new(self.wifi, &tcp_state);

    let mut client = HttpClient::new_with_tls(&tcp, &dns, tls_config);
    let mut buffer = [0u8; 4096];
    let mut http_req = client
        .request(reqwless::request::Method::GET, &self.url)
        .await
        .unwrap();
    let response = http_req.send(&mut buffer).await.unwrap();

    info!("Got response");
    let res = response.body().read_to_end().await.unwrap();

    let (data, _): (WeatherData, _) = serde_json_core::de::from_slice(res).unwrap();
    data
}
```

## Остальная часть структуры

Обратите внимание: API возвращает состояние погоды числом (он также даёт строку, но с числом работать проще). Мы сопоставим эти числа с перечислением `ConditionCode`. Кроме того, в это перечисление мы добавили функцию `icon`, которая сообщает, какую иконку показывать для каждого конкретного погодного условия.


```rust
#[derive(Debug, Deserialize_repr)]
#[repr(u16)]
pub enum ConditionCode {
    // Группа 2xx: гроза
    ThunderstormWithLightRain = 200,
    ThunderstormWithRain = 201,
    ThunderstormWithHeavyRain = 202,
    LightThunderstorm = 210,
    Thunderstorm = 211,
    HeavyThunderstorm = 212,
    RaggedThunderstorm = 221,
    ThunderstormWithLightDrizzle = 230,
    ThunderstormWithDrizzle = 231,
    ThunderstormWithHeavyDrizzle = 232,

    // Группа 3xx: морось
    LightIntensityDrizzle = 300,
    Drizzle = 301,
    HeavyIntensityDrizzle = 302,
    LightIntensityDrizzleRain = 310,
    DrizzleRain = 311,
    HeavyIntensityDrizzleRain = 312,
    ShowerRainAndDrizzle = 313,
    HeavyShowerRainAndDrizzle = 314,
    ShowerDrizzle = 321,

    // Группа 5xx: дождь
    LightRain = 500,
    ModerateRain = 501,
    HeavyIntensityRain = 502,
    VeryHeavyRain = 503,
    ExtremeRain = 504,
    FreezingRain = 511,
    LightIntensityShowerRain = 520,
    ShowerRain = 521,
    HeavyIntensityShowerRain = 522,
    RaggedShowerRain = 531,

    // Группа 6xx: снег
    LightSnow = 600,
    Snow = 601,
    HeavySnow = 602,
    Sleet = 611,
    LightShowerSleet = 612,
    ShowerSleet = 613,
    LightRainAndSnow = 615,
    RainAndSnow = 616,
    LightShowerSnow = 620,
    ShowerSnow = 621,
    HeavyShowerSnow = 622,

    // Группа 7xx: атмосфера
    Mist = 701,
    Smoke = 711,
    Haze = 721,
    SandDustWhirls = 731,
    Fog = 741,
    Sand = 751,
    Dust = 761,
    VolcanicAsh = 762,
    Squalls = 771,
    Tornado = 781,

    // Группа 800: ясно
    ClearSky = 800,

    // Группа 80x: облака
    FewClouds = 801,
    ScatteredClouds = 802,
    BrokenClouds = 803,
    OvercastClouds = 804,
}

impl ConditionCode {
    pub fn icon(&self) -> &'static str {
        match self {
            // Гроза
            ConditionCode::ThunderstormWithLightRain
            | ConditionCode::ThunderstormWithRain
            | ConditionCode::ThunderstormWithHeavyRain
            | ConditionCode::LightThunderstorm
            | ConditionCode::Thunderstorm
            | ConditionCode::HeavyThunderstorm
            | ConditionCode::RaggedThunderstorm
            | ConditionCode::ThunderstormWithLightDrizzle
            | ConditionCode::ThunderstormWithDrizzle
            | ConditionCode::ThunderstormWithHeavyDrizzle => "storm.bmp",

            // Морось
            ConditionCode::LightIntensityDrizzle
            | ConditionCode::Drizzle
            | ConditionCode::HeavyIntensityDrizzle
            | ConditionCode::LightIntensityDrizzleRain
            | ConditionCode::DrizzleRain
            | ConditionCode::HeavyIntensityDrizzleRain
            | ConditionCode::ShowerRainAndDrizzle
            | ConditionCode::HeavyShowerRainAndDrizzle
            | ConditionCode::ShowerDrizzle => "rainy.bmp",

            // Дождь
            ConditionCode::LightRain
            | ConditionCode::ModerateRain
            | ConditionCode::HeavyIntensityRain
            | ConditionCode::VeryHeavyRain
            | ConditionCode::ExtremeRain
            | ConditionCode::LightIntensityShowerRain
            | ConditionCode::ShowerRain
            | ConditionCode::HeavyIntensityShowerRain
            | ConditionCode::RaggedShowerRain => "rainy_heavy.bmp",
            ConditionCode::FreezingRain => "weather_mix.bmp",

            // Снег
            ConditionCode::LightSnow
            | ConditionCode::Snow
            | ConditionCode::HeavySnow
            | ConditionCode::Sleet
            | ConditionCode::LightShowerSleet
            | ConditionCode::ShowerSleet
            | ConditionCode::LightRainAndSnow
            | ConditionCode::RainAndSnow
            | ConditionCode::LightShowerSnow
            | ConditionCode::ShowerSnow
            | ConditionCode::HeavyShowerSnow => "snowing.bmp",

            // Атмосфера
            ConditionCode::Mist
            | ConditionCode::Smoke
            | ConditionCode::Haze
            | ConditionCode::SandDustWhirls
            | ConditionCode::Fog
            | ConditionCode::Sand
            | ConditionCode::Dust
            | ConditionCode::VolcanicAsh
            | ConditionCode::Squalls => "foggy.bmp",
            ConditionCode::Tornado => "cyclone.bmp",

            // Ясно
            ConditionCode::ClearSky => "sunny.bmp",

            // Облака
            ConditionCode::FewClouds
            | ConditionCode::ScatteredClouds
            | ConditionCode::BrokenClouds
            | ConditionCode::OvercastClouds => "partly_cloudy_day.bmp",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub id: ConditionCode,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i32,
    pub humidity: i32,
    pub sea_level: Option<i32>,
    pub grnd_level: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: f64,
    pub gust: Option<f64>,
}

```
