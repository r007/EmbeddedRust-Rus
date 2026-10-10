# Запись данных на SD-карту с Raspberry Pi Pico

Когда вы освоили чтение SD-карты, этот шаг простой. Нужно открыть файл в режиме ReadWriteCreateOrTruncate. Если файла нет, он будет создан; если он существует, его содержимое будет обрезано. После этого можно писать в файл.

Мы создадим файл `FERRIS.TXT` и запишем в него emoji Ferris (🦀).

```rust
let Ok(mut my_file) = root_dir.open_file_in_dir(
    "FERRIS.TXT",
    embedded_sdmmc::Mode::ReadWriteCreateOrTruncate,
) else {
    serial.write("err in open_file_in_dir".as_bytes()).unwrap();
    continue;
};
my_file.write("🦀".as_bytes()).unwrap();
serial.write("Written".as_bytes()).unwrap();
```

Чтобы проверить результат, можно подключить SD-карту напрямую к компьютеру или изменить программу чтения SD-карты так, чтобы она читала и выводила содержимое этого файла.
