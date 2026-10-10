# syntax=docker/dockerfile:1
#
# Многоэтапная сборка сборника книг EmbeddedRust-Rus.
#
#   Этап 1 (builder) — тулчейн Rust и инструменты mdBook; запускает
#                      `cargo xtask build`, который создаёт site/.
#   Этап 2 (runtime) — непривилегированный nginx, раздающий готовые файлы.
#                      В итоговом образе нет Rust, mdBook и исходников.
#
# Собирайте из КОРНЯ РЕПОЗИТОРИЯ, а не из docker/:
#   docker build -f docker/Dockerfile -t embedded-rust-ru .

ARG RUST_VERSION=1
ARG MDBOOK_VERSION=0.5.4
ARG MDBOOK_MERMAID_VERSION=0.17.1
ARG MDBOOK_EPUB_VERSION=0.5.4
ARG MDBOOK_I18N_VERSION=0.4.0
ARG MDBOOK_SVGBOB_VERSION=0.3.1
ARG MDBOOK_TABS_VERSION=1.0.4
ARG MDBOOK_SHIFTINCLUDE_VERSION=0.2.0
ARG NGINX_VERSION=1.27

# ──────────────────────────────────────────────────────────────────────
# Этап 1: сборка книг
# ──────────────────────────────────────────────────────────────────────
FROM rust:${RUST_VERSION}-slim-bookworm AS builder

ARG MDBOOK_VERSION
ARG MDBOOK_MERMAID_VERSION
ARG MDBOOK_EPUB_VERSION
ARG MDBOOK_I18N_VERSION
ARG MDBOOK_SVGBOB_VERSION
ARG MDBOOK_TABS_VERSION
ARG MDBOOK_SHIFTINCLUDE_VERSION

WORKDIR /build

# zip нужен xtask для архива с упражнениями Comprehensive Rust.
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl zip \
    && rm -rf /var/lib/apt/lists/*

# Версии зафиксированы: сборка воспроизводима, а CI и контейнер используют одни и те же инструменты.
# Все инструменты ставятся через `cargo install --locked`, поэтому они совместимы с mdBook ${MDBOOK_VERSION}.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo install mdbook --version "${MDBOOK_VERSION}" --locked --root /usr/local; \
    cargo install mdbook-mermaid --version "${MDBOOK_MERMAID_VERSION}" --locked --root /usr/local; \
    cargo install mdbook-epub --version "${MDBOOK_EPUB_VERSION}" --locked --root /usr/local; \
    cargo install mdbook-i18n-helpers --version "${MDBOOK_I18N_VERSION}" --locked --root /usr/local; \
    cargo install mdbook-svgbob --version "${MDBOOK_SVGBOB_VERSION}" --locked --root /usr/local; \
    cargo install mdbook-tabs --version "${MDBOOK_TABS_VERSION}" --locked --root /usr/local; \
    cargo install mdbook-shiftinclude --version "${MDBOOK_SHIFTINCLUDE_VERSION}" --locked --root /usr/local; \
    mdbook --version; mdbook-mermaid --version

# Копируем весь репозиторий: книги, covers/, скрипты и крейт xtask.
COPY . .

# Вспомогательные крейты Comprehensive Rust (препроцессор course и бэкенд exerciser) лежат в самом репозитории.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/comprehensive-rust/mdbook-course/target \
    --mount=type=cache,target=/build/comprehensive-rust/mdbook-exerciser/target \
    cargo install --path comprehensive-rust/mdbook-course --root /usr/local \
    && cargo install --path comprehensive-rust/mdbook-exerciser --root /usr/local

# `cargo xtask build` пишет в site/ и падает, если хотя бы одна книга не собралась.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo run --release --package xtask -- build \
    && test -f site/index.html \
    && echo "==> собрано книг: $(find site -mindepth 1 -maxdepth 1 -type d ! -name covers | wc -l)"

# ──────────────────────────────────────────────────────────────────────
# Этап 2: раздача
# ──────────────────────────────────────────────────────────────────────
# nginx-unprivileged — стандартный образ nginx, переконфигурированный на
# работу от uid 101 и порт 8080. Поэтому контейнеру не нужны root-права
# и NET_BIND_SERVICE.
FROM nginxinc/nginx-unprivileged:${NGINX_VERSION}-alpine AS runtime

COPY docker/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder --chown=nginx:nginx /build/site /usr/share/nginx/html

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD wget -q -O /dev/null http://127.0.0.1:8080/ || exit 1
