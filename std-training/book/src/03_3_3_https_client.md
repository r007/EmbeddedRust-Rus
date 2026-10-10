# HTTPS-клиент

Теперь внесите изменения в файлы HTTP-клиента, чтобы он работал и с зашифрованными соединениями.

`intro/http-client/examples/https_client.rs` содержит решение. Запустить его можно командой:

```console
cargo run --example https_client
```

Создайте собственную конфигурацию клиента для `esp_idf_svc::http::client::EspHttpConnection`, которая включает использование этих сертификатов и задаёт значения по умолчанию для всего остального:

```rust
{{#include ../../intro/http-client/examples/https_client.rs:connection}}
```

✅ Инициализируйте HTTP-клиент с новой конфигурацией и проверьте работу HTTPS, загрузив ресурс по `https`, например `https://espressif.com/`. Загрузка выведется в терминал как исходный HTML.

## Устранение неполадок (повтор из предыдущего раздела)

- `missing WiFi name/password`: убедитесь, что `cfg.toml` настроен по образцу `cfg.toml.example`. Частая проблема — имя пакета и имя секции конфигурации не совпадают.

```toml
# Cargo.toml
#...
[package]
name = "http-client"
#...

# cfg.toml
[http-client]
wifi_ssid = "..."
wifi_psk = "..."
```
