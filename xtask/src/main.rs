//! Сборка сборника книг: `cargo xtask build | serve | clean`.
//!
//! Каждая книга собирается отдельным `mdbook build` из своего каталога, затем
//! результаты копируются в `site/<книга>/`. Страница-каталог `site/index.html`
//! генерируется здесь же из таблиц BOOKS, CATEGORIES и TAGS.

use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

/// Категории каталога: (идентификатор, название, описание).
const CATEGORIES: &[(&str, &str, &str)] = &[
    ("basics", "Основы Rust", "Язык Rust перед работой с железом"),
    (
        "theory",
        "Теория embedded",
        "Понятия и архитектура embedded-разработки на Rust",
    ),
    (
        "boards",
        "Практика на платах",
        "Книги с проектами на конкретных микроконтроллерных платах",
    ),
    ("courses", "Курсы", "Структурированные курсы с упражнениями"),
];

/// Теги каталога: (идентификатор для фильтра, отображаемое название).
const TAGS: &[(&str, &str)] = &[
    ("rust", "Rust"),
    ("embedded", "Embedded"),
    ("bare-metal", "Bare-metal"),
    ("no_std", "no_std"),
    ("std", "std"),
    ("micro-bit", "micro:bit v2"),
    ("rp2350", "RP2350 / Pico 2"),
    ("esp32", "ESP32"),
    ("esp-hal", "esp-hal"),
    ("esp-idf", "ESP-IDF"),
    ("rp-hal", "rp-hal"),
    ("embassy", "Embassy"),
    ("async", "async"),
    ("gpio", "GPIO"),
    ("pwm", "PWM"),
    ("adc", "ADC"),
    ("i2c", "I2C"),
    ("spi", "SPI"),
    ("uart", "UART"),
    ("interrupts", "Прерывания"),
    ("wifi", "Wi-Fi"),
    ("ble", "Bluetooth LE"),
    ("beginners", "Для начинающих"),
    ("course", "Курс"),
];

/// Книга сборника.
struct Book {
    /// Каталог с book.toml относительно корня репозитория.
    source: &'static str,
    /// Имя каталога в site/ и часть адреса на сайте.
    id: &'static str,
    title: &'static str,
    description: &'static str,
    category: &'static str,
    tags: &'static [&'static str],
    /// Имя, под которым EPUB попадает на сайт. Файл берётся из epub/ результата сборки.
    epub: Option<&'static str>,
    /// Имя ZIP с упражнениями. Архив собирается из exerciser/<имя без .zip>/.
    zip: Option<&'static str>,
}

const BOOKS: &[Book] = &[
    Book {
        source: "rust-training",
        id: "rust-training",
        title: "Практический курс Rust",
        description: "Основы языка, многозадачность, веб, FFI, Python и встраиваемые системы",
        category: "basics",
        tags: &["rust", "embedded", "async", "course"],
        epub: None,
        zip: None,
    },
    Book {
        source: "comprehensive-rust",
        id: "comprehensive-rust",
        title: "Comprehensive Rust 🦀",
        description: "Курс Rust от Google: от основ до bare-metal, FFI и асинхронности",
        category: "basics",
        tags: &["rust", "bare-metal", "course"],
        epub: Some("comprehensive-rust.epub"),
        zip: Some("comprehensive-rust-exercises.zip"),
    },
    Book {
        source: "embedded-book",
        id: "embedded-book",
        title: "Встраиваемый Rust (The Embedded Rust Book)",
        description: "Концепции embedded-разработки на Rust: no_std, HAL, PAC, BSP и прерывания",
        category: "theory",
        tags: &["embedded", "bare-metal", "no_std"],
        epub: None,
        zip: None,
    },
    Book {
        source: "discovery-mb2",
        id: "discovery-mb2",
        title: "Учебник Discovery по Rust на BBC micro-bit v2",
        description:
            "Первые шаги с Rust на micro:bit v2: светодиоды, UART, I2C, прерывания и игра «Змейка»",
        category: "boards",
        tags: &[
            "micro-bit",
            "embedded",
            "beginners",
            "gpio",
            "uart",
            "i2c",
            "interrupts",
        ],
        epub: Some("Учебник Discovery по Rust на BBC micro-bit v2.epub"),
        zip: None,
    },
    Book {
        source: "pico-pico",
        id: "pico-pico",
        title: "impl Rust для RP2350",
        description: "Raspberry Pi Pico 2: GPIO, PWM, I2C, SPI, Embassy и практические проекты",
        category: "boards",
        tags: &[
            "rp2350", "rp-hal", "embassy", "async", "gpio", "pwm", "i2c", "spi",
        ],
        epub: None,
        zip: None,
    },
    Book {
        source: "esp32-book",
        id: "esp32-book",
        title: "impl Rust для ESP32",
        description: "ESP32: светодиоды, датчики, Wi-Fi, Bluetooth, дисплеи и практические проекты",
        category: "boards",
        tags: &[
            "esp32", "esp-hal", "embassy", "wifi", "ble", "adc", "pwm", "spi", "i2c",
        ],
        epub: None,
        zip: None,
    },
    Book {
        source: "no-std-training",
        id: "no-std-training",
        title: "Обучение Embedded Rust для ESP32",
        description: "Курс на no_std: от первой программы до подключения к Wi-Fi на ESP32",
        category: "courses",
        tags: &["esp32", "no_std", "esp-hal", "embassy", "wifi", "course"],
        epub: None,
        zip: None,
    },
    Book {
        source: "std-training/book",
        id: "std-training",
        title: "Embedded Rust на Espressif",
        description: "Курс на std с ESP-IDF: HTTP-клиент и сервер, датчики по I2C, MQTT на ESP32",
        category: "courses",
        tags: &["esp32", "std", "esp-idf", "wifi", "i2c", "course"],
        epub: None,
        zip: None,
    },
];

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask должен находиться во вложенном каталоге воркспейса")
        .to_path_buf()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("build") => cmd_build(),
        Some("serve") => cmd_serve(),
        Some("clean") => cmd_clean(),
        Some("--help" | "-h" | "help") | None => print_usage(0),
        Some(other) => {
            eprintln!("Неизвестная команда: {other}\n");
            print_usage(1);
        }
    }
}

fn print_usage(code: i32) {
    let text = "\
Использование: cargo xtask <КОМАНДА>

Команды:
  build    Собрать все книги и страницу-каталог в site/
  serve    Собрать и запустить сервер на http://localhost:3000
  clean    Удалить каталоги site/ и target/book-build/";
    if code == 0 {
        println!("{text}");
    } else {
        eprintln!("{text}");
    }
    process::exit(code);
}

// ── сборка ───────────────────────────────────────────────────────────

fn cmd_build() {
    if !mdbook_available() {
        eprintln!("Ошибка: 'mdbook' не найден в PATH. Установите его: https://rust-lang.github.io/mdBook/guide/installation.html");
        process::exit(1);
    }
    let root = project_root();
    let site = root.join("site");
    let tmp = root.join("target").join("book-build");
    reset_dir(&site);
    reset_dir(&tmp);

    println!("Сборка книг в site/\n");
    let mut failed = Vec::new();
    for book in BOOKS {
        match build_book(&root, &tmp, &site, book) {
            Ok(()) => println!("  ✓ {}", book.id),
            Err(msg) => {
                eprintln!("  ✗ {}: {msg}", book.id);
                failed.push(book.id);
            }
        }
    }
    write_landing(&root, &site);
    fs::write(site.join(".nojekyll"), "").expect("не удалось создать .nojekyll");

    if failed.is_empty() {
        println!(
            "\nГотово: собрано книг {}/{}. Результат в site/",
            BOOKS.len(),
            BOOKS.len()
        );
    } else {
        eprintln!("\nНе собрались книги: {}", failed.join(", "));
        process::exit(1);
    }
}

fn build_book(root: &Path, tmp: &Path, site: &Path, book: &Book) -> Result<(), String> {
    let src = root.join(book.source);
    if !src.join("book.toml").is_file() {
        return Err(format!("нет book.toml в {}", book.source));
    }
    let out = tmp.join(book.id);
    let status = Command::new("mdbook")
        .args(["build", "--dest-dir"])
        .arg(&out)
        .current_dir(&src)
        .status()
        .map_err(|e| format!("не удалось запустить mdbook: {e}"))?;
    if !status.success() {
        return Err("ОШИБКА СБОРКИ".into());
    }

    let dest = site.join(book.id);
    // Книги с несколькими бэкендами (например, comprehensive-rust) кладут HTML в html/.
    let html_dir = out.join("html");
    if html_dir.join("index.html").is_file() {
        copy_dir(&html_dir, &dest, &[]);
    } else {
        copy_dir(&out, &dest, &["epub"]);
    }
    if let Some(name) = book.epub {
        let epub = find_epub(&out).ok_or_else(|| format!("EPUB не создан для {name}"))?;
        fs::copy(&epub, dest.join(name))
            .map_err(|e| format!("не удалось скопировать EPUB: {e}"))?;
    }
    if let Some(zip_name) = book.zip {
        let stem = zip_name.trim_end_matches(".zip");
        let exercises = out.join("exerciser").join(stem);
        if !exercises.is_dir() {
            return Err(format!("нет каталога упражнений exerciser/{stem}"));
        }
        let status = Command::new("zip")
            .args(["-r", "-q"])
            .arg(dest.join(zip_name))
            .arg(stem)
            .current_dir(out.join("exerciser"))
            .status()
            .map_err(|e| format!("не удалось запустить zip: {e}"))?;
        if !status.success() {
            return Err("zip завершился с ошибкой".into());
        }
    }
    Ok(())
}

fn find_epub(out: &Path) -> Option<PathBuf> {
    fs::read_dir(out.join("epub"))
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| p.extension().is_some_and(|x| x == "epub"))
}

fn mdbook_available() -> bool {
    Command::new("mdbook")
        .arg("--version")
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn reset_dir(dir: &Path) {
    if dir.exists() {
        fs::remove_dir_all(dir).expect("не удалось очистить каталог");
    }
    fs::create_dir_all(dir).expect("не удалось создать каталог");
}

/// Копирует дерево каталогов; элементы верхнего уровня из `skip` пропускаются.
fn copy_dir(from: &Path, to: &Path, skip: &[&str]) {
    fs::create_dir_all(to).expect("не удалось создать каталог назначения");
    for entry in fs::read_dir(from).expect("не удалось прочитать каталог")
    {
        let entry = entry.expect("ошибка чтения каталога");
        let name = entry.file_name();
        if skip.iter().any(|s| name == *s) {
            continue;
        }
        let target = to.join(&name);
        if entry.file_type().expect("ошибка типа файла").is_dir() {
            copy_dir(&entry.path(), &target, &[]);
        } else {
            fs::copy(entry.path(), &target).expect("не удалось скопировать файл");
        }
    }
}

// ── страница-каталог ────────────────────────────────────────────────

fn write_landing(root: &Path, site: &Path) {
    copy_dir(&root.join("covers"), &site.join("covers"), &[]);

    let mut sections = String::new();
    for &(cat_id, cat_title, cat_desc) in CATEGORIES {
        let cards: String = BOOKS
            .iter()
            .filter(|b| b.category == cat_id)
            .map(card)
            .collect();
        sections.push_str(&format!(
            r#"    <section class="cat cat-{cat_id}" aria-labelledby="cat-{cat_id}">
      <header>
        <h2 id="cat-{cat_id}">{cat_title}</h2>
        <p>{cat_desc}</p>
      </header>
      <div class="grid">
{cards}      </div>
    </section>
"#
        ));
    }

    let chips: String = std::iter::once(r#"<button type="button" class="chip" data-filter="" aria-pressed="true">Все</button>"#.to_string())
        .chain(TAGS.iter().map(|(id, label)| {
            format!(r#"<button type="button" class="chip" data-filter="{id}" aria-pressed="false">{label}</button>"#)
        }))
        .collect::<Vec<_>>()
        .join("\n      ");

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="ru">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="description" content="Книги о Rust для микроконтроллеров на русском языке: переводы и адаптации с примерами для плат micro:bit, Raspberry Pi Pico 2 и ESP32.">
  <title>Rust для embedded: книги на русском</title>
  <link rel="icon" href="covers/embedded-book.svg">
  <style>{CSS}</style>
</head>
<body>
  <header class="hero">
    <p class="eyebrow">Rust · микроконтроллеры</p>
    <h1>Книги по embedded Rust на русском</h1>
    <p class="lead">Переводы и адаптации книг о программировании микроконтроллеров на Rust: от основ языка до проектов на конкретных платах.</p>
    <a class="repo-link" href="https://github.com/r007/EmbeddedRust-Rus">Исходный код и вклад в проект на GitHub →</a>
  </header>

  <main>
    <div class="toolbar">
      <label class="search">
        <span class="visually-hidden">Поиск по книгам</span>
        <input id="search" type="search" placeholder="Поиск по названию и описанию" autocomplete="off">
      </label>
      <div class="chips" role="group" aria-label="Фильтр по тегам">
      {chips}
      </div>
    </div>

{sections}  </main>

  <footer>
    <p>Тексты книг распространяются по лицензиям, которые указаны в каталоге каждой книги (CC BY, CC BY-SA, MIT и Apache-2.0). Скрипты сборки и страница каталога — по лицензии MIT.</p>
    <p>Собрано с помощью <a href="https://rust-lang.github.io/mdBook/">mdBook</a>.</p>
  </footer>
  <script>{JS}</script>
</body>
</html>
"#
    );
    fs::write(site.join("index.html"), html).expect("не удалось записать index.html");
    println!("  ✓ index.html");
}

fn card(book: &Book) -> String {
    let tags: String = book
        .tags
        .iter()
        .map(|t| {
            let label = TAGS
                .iter()
                .find(|(id, _)| id == t)
                .map(|(_, l)| *l)
                .unwrap_or(t);
            format!("<li>{label}</li>")
        })
        .collect();
    let data_tags = book.tags.join(" ");
    let search = format!("{} {}", book.title, book.description).to_lowercase();
    let mut downloads = String::new();
    if let Some(name) = book.epub {
        downloads.push_str(&format!(
            r#"<a class="btn" href="{id}/{file}" download>EPUB</a>"#,
            id = book.id,
            file = url_encode(name)
        ));
    }
    if let Some(name) = book.zip {
        downloads.push_str(&format!(
            r#"<a class="btn" href="{id}/{file}" download>Упражнения (ZIP)</a>"#,
            id = book.id,
            file = url_encode(name)
        ));
    }
    format!(
        r#"        <article class="card" data-tags="{data_tags}" data-search="{search}">
          <a class="cover" href="{id}/" tabindex="-1" aria-hidden="true"><img src="covers/{id}.svg" alt="" loading="lazy"></a>
          <div class="body">
            <h3><a href="{id}/">{title}</a></h3>
            <p>{desc}</p>
            <ul class="tags">{tags}</ul>
            <div class="actions">
              <a class="btn primary" href="{id}/">Читать онлайн</a>
              {downloads}
            </div>
          </div>
        </article>
"#,
        id = book.id,
        title = book.title,
        desc = book.description,
        downloads = downloads,
    )
}

/// Кодирует имя файла для URL: всё, кроме букв, цифр и `-_.~`, превращается в %XX.
fn url_encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

const CSS: &str = r#"
:root {
  --bg: #f7f7fb; --surface: #ffffff; --text: #1b1f2a; --muted: #5b6272;
  --border: rgba(20, 24, 40, .10); --shadow: 0 10px 30px rgba(20, 24, 40, .10);
  --accent: #b7410e; --chip: #eceef5;
  --c-basics: #2563eb; --c-theory: #7c3aed; --c-boards: #059669; --c-courses: #d97706;
}
@media (prefers-color-scheme: dark) {
  :root:not([data-theme="light"]) {
    --bg: #0e1118; --surface: #171b26; --text: #e8eaf2; --muted: #9aa3b8;
    --border: rgba(255, 255, 255, .08); --shadow: 0 10px 30px rgba(0, 0, 0, .35);
    --accent: #f08a5d; --chip: #222838;
  }
}
:root[data-theme="dark"] {
  --bg: #0e1118; --surface: #171b26; --text: #e8eaf2; --muted: #9aa3b8;
  --border: rgba(255, 255, 255, .08); --shadow: 0 10px 30px rgba(0, 0, 0, .35);
  --accent: #f08a5d; --chip: #222838;
}
* { box-sizing: border-box; }
[hidden] { display: none !important; }
.visually-hidden { position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0 0 0 0); white-space: nowrap; }
html { -webkit-text-size-adjust: 100%; }
body {
  margin: 0; background: var(--bg); color: var(--text);
  font: 16px/1.55 -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
}
a { color: var(--accent); }
.hero { padding: 56px 16px 32px; text-align: center; background: linear-gradient(180deg, color-mix(in srgb, var(--c-basics) 14%, transparent), transparent); }
.eyebrow { margin: 0 0 8px; font-size: 13px; letter-spacing: .12em; text-transform: uppercase; color: var(--muted); }
.hero h1 { margin: 0 auto 12px; font-size: clamp(28px, 5vw, 42px); line-height: 1.15; max-width: 22ch; }
.lead { margin: 0 auto 16px; max-width: 60ch; color: var(--muted); }
.repo-link { font-weight: 600; text-decoration: none; }
main { max-width: 1120px; margin: 0 auto; padding: 8px 16px 40px; }
.toolbar { position: sticky; top: 0; z-index: 2; padding: 12px 0; background: var(--bg); }
.search input {
  width: 100%; padding: 12px 14px; font: inherit; color: var(--text); background: var(--surface);
  border: 1px solid var(--border); border-radius: 12px; outline: none;
}
.search input:focus { border-color: var(--accent); box-shadow: 0 0 0 3px color-mix(in srgb, var(--accent) 25%, transparent); }
.chips { display: flex; flex-wrap: wrap; gap: 8px; margin-top: 12px; }
.chip {
  font: inherit; font-size: 14px; padding: 6px 12px; border-radius: 999px; cursor: pointer;
  color: var(--text); background: var(--chip); border: 1px solid var(--border);
}
.chip[aria-pressed="true"] { background: var(--accent); border-color: var(--accent); color: #fff; }
.cat { margin-top: 36px; }
.cat header { margin-bottom: 16px; border-left: 4px solid var(--stripe); padding-left: 14px; }
.cat h2 { margin: 0; font-size: 22px; }
.cat header p { margin: 4px 0 0; color: var(--muted); }
.cat-basics { --stripe: var(--c-basics); }
.cat-theory { --stripe: var(--c-theory); }
.cat-boards { --stripe: var(--c-boards); }
.cat-courses { --stripe: var(--c-courses); }
.grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(260px, 1fr)); gap: 20px; }
.card {
  display: flex; flex-direction: column; overflow: hidden; background: var(--surface);
  border: 1px solid var(--border); border-radius: 16px; box-shadow: var(--shadow);
  transition: transform .15s ease, box-shadow .15s ease;
}
.card:hover { transform: translateY(-3px); }
.cover { display: block; aspect-ratio: 16 / 9; background: var(--chip); }
.cover img { display: block; width: 100%; height: 100%; object-fit: cover; }
.body { display: flex; flex: 1; flex-direction: column; gap: 10px; padding: 16px 18px 18px; }
.body h3 { margin: 0; font-size: 18px; line-height: 1.3; }
.body h3 a { color: var(--text); text-decoration: none; }
.body h3 a:hover { text-decoration: underline; }
.body p { margin: 0; color: var(--muted); font-size: 15px; }
.tags { display: flex; flex-wrap: wrap; gap: 6px; margin: auto 0 0; padding: 0; list-style: none; }
.tags li { font-size: 12px; padding: 3px 8px; border-radius: 6px; background: var(--chip); color: var(--muted); }
.actions { display: flex; flex-wrap: wrap; gap: 8px; margin-top: 4px; }
.btn {
  display: inline-block; padding: 8px 14px; border-radius: 10px; font-size: 14px; font-weight: 600;
  text-decoration: none; color: var(--text); border: 1px solid var(--border); background: var(--surface);
}
.btn.primary { background: var(--accent); border-color: var(--accent); color: #fff; }
.btn:hover { filter: brightness(1.05); }
.empty { color: var(--muted); }
footer { max-width: 1120px; margin: 0 auto; padding: 24px 16px 48px; color: var(--muted); font-size: 14px; border-top: 1px solid var(--border); }
footer p { margin: 8px 0; }
@media (max-width: 480px) {
  .hero { padding-top: 36px; }
  .grid { grid-template-columns: 1fr; }
}
"#;

const JS: &str = r#"
(function () {
  var search = document.getElementById('search');
  var chips = document.querySelectorAll('.chip');
  var cards = document.querySelectorAll('.card');
  var sections = document.querySelectorAll('.cat');
  var tag = '';
  function apply() {
    var q = search.value.trim().toLowerCase();
    cards.forEach(function (card) {
      var byTag = !tag || card.dataset.tags.split(' ').indexOf(tag) !== -1;
      var byText = !q || card.dataset.search.indexOf(q) !== -1;
      card.hidden = !(byTag && byText);
    });
    sections.forEach(function (sec) {
      sec.hidden = !sec.querySelector('.card:not([hidden])');
    });
  }
  chips.forEach(function (chip) {
    chip.addEventListener('click', function () {
      tag = chip.dataset.filter;
      chips.forEach(function (c) { c.setAttribute('aria-pressed', String(c === chip)); });
      apply();
    });
  });
  search.addEventListener('input', apply);
})();
"#;

// ── сервер ───────────────────────────────────────────────────────────

enum Resolved {
    File(PathBuf),
    Redirect(String),
    NotFound,
}

fn cmd_serve() {
    cmd_build();
    let site = project_root().join("site");
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).expect("не удалось занять порт 3000");
    println!("\nСервер запущен: http://localhost:3000  (Ctrl+C — остановить)");

    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };
        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf).unwrap_or(0);
        let request = String::from_utf8_lossy(&buf[..n]);
        let target = request
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .unwrap_or("/");

        let response = match resolve(&site, target) {
            Resolved::File(path) => {
                let body = fs::read(&path).unwrap_or_default();
                let head = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    mime(&path),
                    body.len()
                );
                [head.into_bytes(), body].concat()
            }
            Resolved::Redirect(to) => format!(
                "HTTP/1.1 301 Moved Permanently\r\nLocation: {to}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            )
            .into_bytes(),
            Resolved::NotFound => {
                let body = "404 Not Found";
                format!(
                    "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                )
                .into_bytes()
            }
        };
        let _ = stream.write_all(&response);
    }
}

/// Переводит путь HTTP-запроса в файл внутри `site`. Отказывает в обходе каталогов (`..`).
/// Ссылки без расширения открывают `<имя>.html`, как на GitHub Pages.
fn resolve(site: &Path, target: &str) -> Resolved {
    let path_only = target.split(['?', '#']).next().unwrap_or("/");
    let decoded = percent_decode(path_only);
    if decoded.contains('\0') {
        return Resolved::NotFound;
    }
    let mut file = site.to_path_buf();
    for seg in decoded.split('/').filter(|s| !s.is_empty()) {
        if seg == ".." {
            return Resolved::NotFound;
        }
        file.push(seg);
    }
    if file.is_dir() {
        if !path_only.ends_with('/') {
            return Resolved::Redirect(format!("{path_only}/"));
        }
        file.push("index.html");
    }
    if file.is_file() {
        return Resolved::File(file);
    }
    let with_html = PathBuf::from(format!("{}.html", file.display()));
    if with_html.is_file() {
        return Resolved::File(with_html);
    }
    Resolved::NotFound
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(hi), Some(lo)) = (hex(bytes[i + 1]), hex(bytes[i + 2])) {
                out.push(hi << 4 | lo);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn mime(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("woff2") => "font/woff2",
        Some("woff") => "font/woff",
        Some("json") => "application/json",
        Some("epub") => "application/epub+zip",
        Some("pdf") => "application/pdf",
        _ => "application/octet-stream",
    }
}

// ── очистка ──────────────────────────────────────────────────────────

fn cmd_clean() {
    let root = project_root();
    for dir in [root.join("site"), root.join("target").join("book-build")] {
        if dir.exists() {
            fs::remove_dir_all(&dir).expect("не удалось удалить каталог");
            println!(
                "Удалено: {}",
                dir.strip_prefix(&root).unwrap_or(&dir).display()
            );
        }
    }
}
