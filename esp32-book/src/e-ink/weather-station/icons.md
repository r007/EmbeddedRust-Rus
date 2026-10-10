{{#title Иконки погоды для e-Ink на ESP32: tinybmp и build.rs}}

# Иконки

Нам понадобится набор иконок, чтобы показывать погоду, влажность, качество воздуха и другую информацию. Я преобразовал эти иконки из Google Fonts в файлы BMP, которые можно использовать с крейтом tinybmp.

Мы будем хранить байты изображений в статическом массиве, чтобы перебирать их и находить иконку, которая соответствует текущей погоде. Для этого я добавил в `build.rs` новую функцию: она загружает файлы иконок из указанной папки и генерирует статический массив в `icons.rs`. Так нам не нужно вручную обновлять массив каждый раз, когда добавляется новая иконка.

## Изменяем файл build.rs
В `build.rs` мы добавим функцию, которая сканирует папку `src/icons/`. Иконки можно взять из этого репозитория: https://github.com/ImplFerris/esp32-epaper-weather/tree/main/src/icons и поместить в свою папку `src/icons`.

```rust
fn generate_icon_array() {
    let dest_path = Path::new("src/icons.rs");
    let icon_dir = Path::new("src/icons/");

    let mut generated_code = String::new();

    generated_code.push_str("pub static ICONS: &[(&str, &[u8])] = &[\n");

    if let Ok(entries) = fs::read_dir(icon_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                generated_code.push_str(&format!(
                    "    (\"{}\", include_bytes!(\"{}/{}\")),\n",
                    file_name, "icons", file_name
                ));
            }
        }
    }

    generated_code.push_str("];\n");

    fs::write(dest_path, generated_code).expect("Failed to write icons.rs");

    println!("cargo:rerun-if-changed={}", icon_dir.to_str().unwrap());
}
```

Обновите функцию `main` в `build.rs`, чтобы она вызывала нашу новую функцию:
```rust
fn main() {
    generate_icon_array();

    linker_be_nice();
    println!("cargo:rustc-link-arg=-Tdefmt.x");
    // убедитесь, что linkall.x — последний скрипт компоновщика (иначе возможны проблемы с flip-link)
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}
```

## icons.rs — не меняйте вручную!
Если всё прошло гладко, в папке `src` появится файл `icons.rs` с содержимым ниже. Если файл не создан, попробуйте запустить скрипт сборки вручную, выполнив `cargo build`.

```rust
pub static ICONS: &[(&str, &[u8])] = &[
    ("partly_cloudy_day.bmp", include_bytes!("icons/partly_cloudy_day.bmp")),
    ("nights_stay.bmp", include_bytes!("icons/nights_stay.bmp")),
    ("humidity_percentage.bmp", include_bytes!("icons/humidity_percentage.bmp")),
    ("cyclone.bmp", include_bytes!("icons/cyclone.bmp")),
    ("foggy.bmp", include_bytes!("icons/foggy.bmp")),
    ("snowing_heavy.bmp", include_bytes!("icons/snowing_heavy.bmp")),
    ("sunny.bmp", include_bytes!("icons/sunny.bmp")),
    ("cloud.bmp", include_bytes!("icons/cloud.bmp")),
    ("partly_cloudy_night.bmp", include_bytes!("icons/partly_cloudy_night.bmp")),
    ("rainy.bmp", include_bytes!("icons/rainy.bmp")),
    ("heat.bmp", include_bytes!("icons/heat.bmp")),
    ("clear_day.bmp", include_bytes!("icons/clear_day.bmp")),
    ("weather_mix.bmp", include_bytes!("icons/weather_mix.bmp")),
    ("weather_hail.bmp", include_bytes!("icons/weather_hail.bmp")),
    ("rainy_snow.bmp", include_bytes!("icons/rainy_snow.bmp")),
    ("mist.bmp", include_bytes!("icons/mist.bmp")),
    ("snowing.bmp", include_bytes!("icons/snowing.bmp")),
    ("rainy_heavy.bmp", include_bytes!("icons/rainy_heavy.bmp")),
    ("flood.bmp", include_bytes!("icons/flood.bmp")),
    ("sunny_snowing.bmp", include_bytes!("icons/sunny_snowing.bmp")),
    ("air.bmp", include_bytes!("icons/air.bmp")),
    ("storm.bmp", include_bytes!("icons/storm.bmp")),
    ("weather_snowy.bmp", include_bytes!("icons/weather_snowy.bmp")),
    ("thunderstorm.bmp", include_bytes!("icons/thunderstorm.bmp")),
    ("thermostat.bmp", include_bytes!("icons/thermostat.bmp")),
    ("rainy_light.bmp", include_bytes!("icons/rainy_light.bmp")),
];
```
