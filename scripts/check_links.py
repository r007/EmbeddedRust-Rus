#!/usr/bin/env python3
"""Проверяет внутренние ссылки в собранном сайте (site/).

Проверяются относительные и абсолютные (от корня сайта) ссылки href и src.
Внешние адреса не проверяются. Ссылка считается рабочей, если указывает на файл,
на файл с добавленным .html или на каталог с index.html.

Использование: python3 scripts/check_links.py site
"""
import os
import re
import sys
import urllib.parse
from html.parser import HTMLParser

# print.html собирается mdBook из исходного Markdown, поэтому относительные пути
# к медиа в нём указывают не туда (например, ../assets/). Это известное ограничение
# mdBook, поэтому страницу печати не проверяем.
SKIP_FILES = {"print.html"}
EXTERNAL = re.compile(r"^(https?:|mailto:|data:|javascript:|#)")


class RefCollector(HTMLParser):
    def __init__(self):
        super().__init__()
        self.refs = []

    def handle_starttag(self, tag, attrs):
        for name, value in attrs:
            if name in ("href", "src") and value:
                self.refs.append(value)


def exists(path):
    return (
        os.path.isfile(path)
        or os.path.isfile(path + ".html")
        or os.path.isfile(os.path.join(path, "index.html"))
    )


def main():
    site = sys.argv[1] if len(sys.argv) > 1 else "site"
    checked, broken = 0, []
    for root, _, files in os.walk(site):
        for name in files:
            if not name.endswith(".html") or name in SKIP_FILES:
                continue
            page = os.path.join(root, name)
            with open(page, encoding="utf-8", errors="replace") as fh:
                collector = RefCollector()
                collector.feed(fh.read())
            for ref in collector.refs:
                if EXTERNAL.match(ref):
                    continue
                target = urllib.parse.unquote(ref.split("#")[0].split("?")[0])
                if not target:
                    continue
                base = site if target.startswith("/") else root
                full = os.path.normpath(os.path.join(base, target.lstrip("/")))
                checked += 1
                if not exists(full):
                    broken.append((os.path.relpath(page, site), ref))

    print(f"Проверено ссылок: {checked}, битых: {len(broken)}")
    for page, ref in broken:
        print(f"  {page} -> {ref}")
    sys.exit(1 if broken else 0)


if __name__ == "__main__":
    main()
