# Контейнерное развёртывание

Необязательный способ развернуть сборник самостоятельно, без GitHub Pages. Подходит, например, для сети за файрволом или внутренней сети.

**Это не путь для локальной разработки.** Для написания и просмотра книг используйте `cargo xtask serve`: он пересобирает книги и запускает сервер на <http://localhost:3000> без контейнера.

## Использование

Из корня репозитория:

```bash
docker compose -f docker/compose.yaml up --build
```

Затем откройте <http://localhost:3000>. Порт хоста можно переопределить через `PORT`:

```bash
PORT=8080 docker compose -f docker/compose.yaml up --build
```

Без Compose:

```bash
docker build -f docker/Dockerfile -t embedded-rust-ru .
docker run --rm -p 3000:8080 embedded-rust-ru
```

В обоих случаях контекст сборки — корень репозитория: сборке нужны исходники книг и крейт `xtask`.

## Как это работает

Сборка идёт в две стадии:

1. **builder** (`rust:1-slim-bookworm`) устанавливает mdBook и препроцессоры, собирает крейты `mdbook-course` и `mdbook-exerciser`, затем запускает `cargo xtask build`. Эта команда собирает все книги и страницу-каталог в `site/` и завершается ошибкой, если хотя бы одна книга не собралась.
2. **runtime** (`nginxinc/nginx-unprivileged:alpine`) раздаёт `site/` на порту 8080. В итоговом образе нет тулчейна Rust, mdBook и исходников книг.

## Фиксированные версии

Версии инструментов заданы аргументами сборки в [Dockerfile](Dockerfile): `MDBOOK_VERSION`, `MDBOOK_MERMAID_VERSION`, `MDBOOK_EPUB_VERSION`, `MDBOOK_I18N_VERSION`, `MDBOOK_SVGBOB_VERSION`, `MDBOOK_TABS_VERSION` и `MDBOOK_SHIFTINCLUDE_VERSION`. CI (`pages.yml` и `ci.yml`) использует те же версии, поэтому контейнер и опубликованный сайт собираются одинаково.

## Примечания

- Контейнер работает от пользователя с uid 101 и слушает непривилегированный порт. Поэтому ему не нужны root-права и дополнительные capabilities.
- Содержимое встраивается при сборке образа. Чтобы подхватить изменения в книгах, пересоберите образ.
- Сборка занимает долго: `cargo install` компилирует инструменты из исходников. Повторные сборки используют кэш слоёв Docker.
