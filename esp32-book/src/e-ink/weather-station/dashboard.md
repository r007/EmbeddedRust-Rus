{{#title Погодная панель на ESP32: вывод данных о погоде на e-Paper}}

# Панель (Dashboard)

В этой главе мы напишем основную логику. Идея в том, чтобы обновлять панель через определённые промежутки времени, скажем каждые 10 минут. Для этого мы будем получать свежие данные о погоде, вызывая функцию `access_website` из модуля weather, и затем обновлять каждую часть панели новой информацией.

## Псевдонимы типов

Создадим псевдонимы типов для SPI-устройства и e-paper дисплея, чтобы код было легче читать и использовать.

```rust
type SpiDevice = ExclusiveDevice<Spi<'static, esp_hal::Blocking>, Output<'static>, Delay>;
type EPD = Epd1in54<SpiDevice, Input<'static>, Output<'static>, Output<'static>, Delay>;
```

## Структура Dashboard

Определим структуру `Dashboard`, которая принимает Wi-Fi-стек, драйвер e-paper и SPI-устройство. Функция `new` инициализирует структуру и задаёт дисплей e-paper по умолчанию.

```rust
pub struct Dashboard {
    display: Display1in54,
    wifi: Stack<'static>,
    epd: EPD,
    spi_dev: SpiDevice,
}

impl Dashboard {
    pub fn new(wifi: Stack<'static>, epd: EPD, spi_dev: SpiDevice) -> Self {
        Self {
            display: Display1in54::default(),
            wifi,
            epd,
            spi_dev,
        }
    }
}
```

Примечание: следующие функции, которые принимают `self` первым параметром, тоже должны находиться внутри блока `impl`.

## Запуск панели

Это стартовая функция, которую мы вызовем далее в файле `main.rs`. Здесь же создаём экземпляр `WeatherApi`.

Я повернул дисплей на 90 градусов. Технически в этом нет необходимости, так как наш дисплей имеет размер 200x200 пикселей. Но если вы используете прямоугольный дисплей, поворот имеет больше смысла.

В цикле каждые 10 минут мы вызываем функцию `refresh`, которая получает свежие данные и обновляет дисплей.

```rust
pub async fn start(&mut self, tls_seed: u64) {
    self.display.set_rotation(DisplayRotation::Rotate90);

    let api = WeatherApi::new(self.wifi, tls_seed);
    loop {
        self.refresh(&api).await;

        Timer::after(Duration::from_secs(60 * 10)).await;
    }
}
```

## Обновление дисплея свежими данными о погоде

Сначала получаем свежие данные о погоде, вызывая `access_website` у `WeatherApi`. Затем выводим e-paper дисплей из режима сна, так как в конце мы снова его усыпим. После этого очищаем предыдущий кадр и заливаем дисплей белым цветом.

Далее выводим обновлённую информацию о погоде: сначала дату, затем иконку погоды и температуру. После этого добавляем влажность, скорость ветра и, наконец, подпись внизу (небольшой текст «implRust»). Когда всё нарисовано, обновляем дисплей новым кадром, ждём 5 секунд и переводим дисплей в спящий режим.

```rust
pub async fn refresh(&mut self, api: &WeatherApi) {
    info!("Getting weather data");
    let weather_data = api.access_website().await;
    info!("Got weather data");

    self.epd.wake_up(&mut self.spi_dev, &mut Delay).unwrap();
    Timer::after(Duration::from_secs(5)).await;

    // Очищаем имеющееся изображение
    self.epd.clear_frame(&mut self.spi_dev, &mut Delay).unwrap();
    self.display.clear(Color::White).unwrap();
    self.epd
        .update_and_display_frame(&mut self.spi_dev, self.display.buffer(), &mut Delay)
        .unwrap();
    Timer::after(Duration::from_secs(5)).await;

    self.draw_date(weather_data.dt);

    self.draw_icon(weather_data.weather[0].id.icon(), Point::new(20, 50));
    self.draw_temperature(weather_data.main.temp, Point::new(20 + 70, 60));

    self.draw_humidity(weather_data.main.humidity);
    self.draw_wind(weather_data.wind.speed);

    self.draw_signature();

    self.epd
        .update_and_display_frame(&mut self.spi_dev, self.display.buffer(), &mut Delay)
        .unwrap();
    Timer::after(Duration::from_secs(5)).await;

    self.epd.sleep(&mut self.spi_dev, &mut Delay).unwrap();
}
```
## Вспомогательная функция получения иконки
Эта простая функция принимает имя иконки и позицию, в которой её нужно нарисовать. Она сопоставляет имя иконки с соответствующими байтами изображения, затем с помощью tinybmp и embedded_graphics превращает байты в изображение и выводит его на дисплей.

```rust
fn draw_icon(&mut self, icon_name: &'static str, pos: Point) {
    let img_bytes = self.get_icon(icon_name).unwrap();

    let bmp = Bmp::from_slice(img_bytes).unwrap();
    let image = Image::new(&bmp, pos);
    image.draw(&mut self.display).unwrap();
}

pub fn get_icon(&self, icon_name: &'static str) -> Option<&'static [u8]> {
    ICONS
        .iter()
        .find(|(name, _)| *name == icon_name)
        .map(|(_, img_bytes)| *img_bytes)
}
```

## Вывод температуры
Выведем температуру на экран, добавим к ней суффикс «°C» и нарисуем под ней горизонтальную линию.

```rust
fn draw_temperature(&mut self, temperature: f64, pos: Point) {
    let text_style = MonoTextStyle::new(&PROFONT_24_POINT, Color::Black);

    info!("Drawing temperature");
    let mut text: String<20> = String::new();
    write!(&mut text, "{}°C", temperature).unwrap();

    Text::with_baseline(&text, pos, text_style, Baseline::Top)
        .draw(&mut self.display)
        .unwrap();

    Line::new(Point::new(0, 105), Point::new(200, 105))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 5))
        .draw(&mut self.display)
        .unwrap();
}
```

## Вывод влажности

Покажем на экране иконку влажности, затем выведем рядом значение влажности и вертикальную линию для разделения.

```rust
fn draw_humidity(&mut self, humidity: i32) {
    self.draw_icon("humidity_percentage.bmp", Point::new(5, 110));

    let text_style = MonoTextStyle::new(&PROFONT_18_POINT, Color::Black);

    let mut text: String<10> = String::new();
    write!(&mut text, "{}", humidity).unwrap();

    Text::with_baseline(&text, Point::new(5 + 50, 120), text_style, Baseline::Top)
        .draw(&mut self.display)
        .unwrap();

    Line::new(Point::new(5 + 85, 120), Point::new(5 + 85, 120 + 30))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 5))
        .draw(&mut self.display)
        .unwrap();
}
```

## Вывод ветра

Выведем скорость ветра на экран: сначала нарисуем иконку ветра, затем значение скорости и единицу измерения `m/s` (м/с).

```rust
fn draw_wind(&mut self, wind_speed: f64) {
    self.draw_icon("air.bmp", Point::new(100, 110));

    let text_style = MonoTextStyle::new(&PROFONT_18_POINT, Color::Black);

    let mut text: String<10> = String::new();
    write!(&mut text, "{}", wind_speed).unwrap();

    Text::with_baseline(&text, Point::new(100 + 50, 120), text_style, Baseline::Top)
        .draw(&mut self.display)
        .unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(Color::Black)
        .build();

    Text::with_baseline("m/s", Point::new(100 + 50, 140), text_style, Baseline::Top)
        .draw(&mut self.display)
        .unwrap();
}
```
## Вывод даты

Выведем текущую дату на экран, отформатировав её с днём, месяцем и годом. Затем выведем текст в заданной позиции и нарисуем под ним горизонтальную линию для разделения.

```rust
fn draw_date(&mut self, dt: DateTime<Utc>) {
    let text_style = MonoTextStyle::new(&PROFONT_24_POINT, Color::Black);

    let mut text: String<24> = String::new();
    write!(
        &mut text,
        "{} {} {}",
        dt.day(),
        month_name(dt.month()),
        dt.year()
    )
    .unwrap();

    Text::with_baseline(&text, Point::new(20, 10), text_style, Baseline::Top)
        .draw(&mut self.display)
        .unwrap();

    Line::new(Point::new(0, 45), Point::new(200, 45))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 5))
        .draw(&mut self.display)
        .unwrap();
}
```

## Вывод подписи

Это не связано с погодой: мы просто выводим «implRust» для удовольствия. Вычисляем центральную позицию, рисуем в центре чёрный прямоугольник и помещаем текст ровно посередине.

```rust
fn draw_signature(&mut self) {
    let display_width = epd1in54_v2::WIDTH as i32;
    let rect_padding = 20;

    let rect_width = display_width - 2 * rect_padding;
    let rect_height = 40;
    let rect_x = rect_padding;
    let rect_y = 170;

    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Color::Black)
        .stroke_width(3)
        .fill_color(Color::Black)
        .build();

    Rectangle::new(
        Point::new(rect_x, rect_y),
        Size::new(rect_width as u32, rect_height as u32),
    )
    .into_styled(style)
    .draw(&mut self.display)
    .unwrap();

    let text = "implRust";
    let text_style = MonoTextStyle::new(&PROFONT_24_POINT, Color::White);

    let char_width = PROFONT_24_POINT.character_size.width as i32;
    let text_width = text.len() as i32 * char_width;
    let text_x = rect_x + (rect_width - text_width) / 2;

    Text::with_baseline(
        text,
        Point::new(text_x as i32, rect_y),
        text_style,
        Baseline::Top,
    )
    .draw(&mut self.display)
    .unwrap();
}
```

## Вспомогательная функция для названия месяца
Мы создали вспомогательную функцию, которая возвращает сокращённое название месяца по его номеру.

```rust
fn month_name(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "Err",
    }
}
```
