# `tether` — план разработки

> Персональный TUI-заметочник для связанных заметок во вселенной **Samos**.
> Узлы = Осколки, связи = натянутые кабели, кластеры = созвездия Осколков, поиск = навигация сквозь Пустоту.
> Фишка: связи тянешь руками ИЛИ через упоминания `[[...]]`; заметки живут в кластерах; всё переносимо (экспорт/импорт).

Этот документ — твой роадмап и одновременно замена issue-трекеру. Иди сверху вниз.
Каждая **Фаза** = отдельная git-ветка. Внутри фазы — пронумерованные шаги.
Отмечай сделанное галочкой `[x]`.

---

## Как пользоваться этим документом

- Читай шаг → делай → проверяй результат («**Проверка:**») → ставь галочку.
- Блоки «**Rust-концепт**» объясняют, что нового ты выучил. Не пропускай.
- Блоки «**⚠️ Грабли**» — типичные ошибки новичка. Сэкономят тебе часы.
- Блоки «**📖 Читать**» — что открыть в The Rust Book (`rustup doc --book` откроет офлайн).
- Команды, начинающиеся с `$`, — это то, что ты вводишь в терминале (сам `$` не вводи).

### Правила git-флоу (соблюдаем с самого начала — навык под корпорат)

- Ветка `main` **всегда** собирается (`cargo build`) и проходит тесты (`cargo test`).
- В `main` напрямую не коммитим. На каждую фазу: `git switch -c phase-N-slug`.
- Коммиты мелкие и осмысленные. Формат — **Conventional Commits**:
  `feat:` (функциональность), `fix:` (баг), `refactor:` (переработка без смены поведения),
  `test:`, `docs:`, `chore:` (рутина). Пример: `feat: add cluster path to Shard`.
- Закрытие фазы: влить с сохранением «пузыря»:
  ```
  $ git switch main
  $ git merge --no-ff phase-N-slug
  ```

### «Проверка качества» — гоняй в конце каждой фазы (потом это станет CI)

```
$ cargo fmt
$ cargo clippy -- -D warnings
$ cargo test
$ cargo build
```

### Обзор фаз

| # | Фаза | Ключевая тема Rust |
|---|------|--------------------|
| 0 | Установка и каркас | cargo, toolchain |
| 1 | Модель данных в памяти | struct/enum, ownership |
| 2 | Сохранение на диск + CLI | serde, Result/`?`, трейты |
| 3 | Кабели, backlinks, контекст, слаги | HashSet, время, транслит |
| 4 | Захват + коллекторы + fuzzy-поиск | процессы, трейты, итераторы |
| 5 | Упоминания `[[...]]` → авто-связь | парсинг, HashMap-индекс, рефакторинг |
| 6 | **Кластеры (папки/подпапки) + ссылки на кластеры** | newtype, дерево, enum-ключи |
| 7 | TUI + рендер Markdown + дерево кластеров | событийный цикл, состояние |
| 8 | Редактор в TUI + автодополнение (Tab) | вложенные состояния, overlay |
| 9 | Эстетика Samos | конфиги, стили |
| 10 | **Экспорт/импорт + лавинный экспорт** | обход графа (BFS), архивы, стратегии конфликтов |
| 11 | (опц.) Демон-индексатор | async, tokio, каналы |
| 12 | (опц.) «Круги» — магия как шаблоны | generics, трейты |

---

# Фаза 0 — Установка Rust и каркас проекта

**Ветка:** _(ещё нет git — создадим в этой фазе)_
**Цель:** рабочий toolchain, пустой проект, который собирается, первый коммит.
**Rust-концепты:** cargo, структура проекта, модули (введение).

### Шаг 0.1 — Установить Rust через rustup

`rustup` — официальный установщик и менеджер версий Rust (аналог `pyenv`/`nvm`). Ставит `rustc` (компилятор) и `cargo` (сборщик + пакетный менеджер, аналог `pip`+`venv`+`make`).

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Скрипт спросит вариант установки — жми `1` (default, stable).
- В конце выполни (или перелогинься):
  ```
  $ source "$HOME/.cargo/env"
  ```

**Проверка:**
```
$ rustc --version
$ cargo --version
$ rustup --version
```

> ⚠️ **Грабли:** не ставь Rust из `apt` — устаревшая версия, конфликтует с rustup. Только rustup.

### Шаг 0.2 — Добавить компоненты toolchain

```
$ rustup component add clippy         # линтер
$ rustup component add rustfmt        # автоформаттер
$ rustup component add rust-analyzer  # LSP для редактора
```

**Проверка:** `$ cargo clippy --version` и `$ cargo fmt --version`.

> **Rust-концепт — toolchain:** связка компилятор+cargo+компоненты определённой версии. Мы на `stable`. Посмотреть: `$ rustup show`.

### Шаг 0.3 — Настроить редактор

- **VS Code:** расширение `rust-analyzer` (НЕ старое `Rust`).
- **Neovim/Helix/JetBrains:** подключи `rust-analyzer` как LSP (в Helix подхватится сам, бинарь уже в PATH).

### Шаг 0.4 — Создать проект

```
$ cd ~/projects
$ cargo new tether
$ cd tether
```
Структура:
```
tether/
├── Cargo.toml   # манифест: имя, версия, зависимости
├── .git/        # cargo сам сделал git init!
├── .gitignore   # уже игнорит target/
└── src/main.rs  # точка входа
```

**Проверка:** `$ cargo run` → `Hello, world!`.

> **Rust-концепт — `cargo run`:** компилит в `target/debug/`, затем запускает. Первый раз дольше, дальше инкрементально.

> ⚠️ **Грабли:** `target/` НЕ коммить (генерируемая, огромная). Проверь `.gitignore`: `$ cat .gitignore` → есть `/target`.

### Шаг 0.5 — Разобрать `main.rs`

```rust
fn main() {
    println!("Hello, world!");
}
```
- `fn main()` — точка входа. `println!` — **макрос** (`!` = макрос). Поменяй текст на `println!("tether online. void is silent.");`, `cargo run`.

### Шаг 0.6 — Первый коммит и служебные файлы

Создай `ROADMAP.md` (этот план), `CHANGELOG.md` (`## [Unreleased]`), `README.md`.
```
$ git add -A
$ git status                 # target/ быть не должно
$ git commit -m "chore: scaffold cargo project with tooling"
```

> **Rust-концепт — модули:** большой код делится на **модули** (`mod`) — как namespace в C++ / пакеты в Python. Пока всё в `main.rs`; со Фазы 1 разнесём по файлам.

### Шаг 0.7 — База

> 📖 **Читать:** Book главы **1–3** (переменные и `mut`, типы, функции, `if`, циклы). Главное отличие: **всё immutable по умолчанию**. `let x=5;` менять нельзя; `let mut x=5;` — можно.

> 💡 **Параллельно:** Rustlings — мелкие упражнения:
> ```
> $ cargo install rustlings && rustlings init
> ```

**✅ Фаза 0 Done when:** `cargo run` работает, `clippy` чист, есть git-история, прочитаны главы 1–3.

---

# Фаза 1 — Модель данных и CRUD в памяти

**Ветка:** `phase-1-model`
**Цель:** описать «Осколок», научиться CRUD **в памяти** (без диска и CLI).
**Rust-концепты:** `struct`, `enum`, `Option`, `Result`, `Vec`, `HashMap`, владение/заимствование, методы (`impl`), unit-тесты.

### Шаг 1.1 — Ветка
```
$ git switch main && git switch -c phase-1-model
```

### Шаг 1.2 — Структура `Shard`

`src/shard.rs`:
```rust
pub type ShardId = String;   // пока String, позже uuid

pub struct Shard {
    pub id: ShardId,
    pub title: String,
    pub body: String,
}
```
- `pub` = публичный (без него — приватно, в отличие от Python).

В `main.rs` сверху:
```rust
mod shard;
use shard::Shard;
```

> **Rust-концепт — владение (ownership), главное в Rust:** у каждого значения один **владелец**; когда он выходит из области видимости — память освобождается (без GC и без `delete`). Передача «по значению» = **move** (старая переменная недоступна). Чтобы «взглянуть, не забирая» — **ссылка** `&x`. Заменяет и `malloc/free` из C++, и GC из Python.

> 📖 **Читать:** Book глава **4** целиком (Ownership) — самая важная. Медленно.

### Шаг 1.3 — Методы (`impl`)

```rust
impl Shard {
    pub fn new(id: ShardId, title: String) -> Self {
        Shard { id, title, body: String::new() }
    }
    pub fn summary(&self) -> String {
        format!("[{}] {}", self.id, self.title)
    }
}
```
- `Self` = тип из `impl`. `&self` = объект по ссылке. `format!` = как `println!`, но возвращает String.

**Проверка:** создай Shard в `main`, напечатай `summary()`.
Коммит: `feat: add Shard struct with constructor and summary`.

### Шаг 1.4 — Хранилище `Net`

`src/net.rs`:
```rust
use std::collections::HashMap;
use crate::shard::{Shard, ShardId};

pub struct Net { shards: HashMap<ShardId, Shard> }

impl Net {
    pub fn new() -> Self { Net { shards: HashMap::new() } }
    pub fn add(&mut self, s: Shard) { self.shards.insert(s.id.clone(), s); }
    pub fn get(&self, id: &str) -> Option<&Shard> { self.shards.get(id) }
    pub fn remove(&mut self, id: &str) -> Option<Shard> { self.shards.remove(id) }
    pub fn count(&self) -> usize { self.shards.len() }
}
```
В `main.rs`: `mod net;`.

> **Rust-концепт — `Option<T>`:** в Rust нет `null`. «Может отсутствовать» = `Option<T>` (`Some(x)`/`None`). Компилятор ЗАСТАВЛЯЕТ обработать оба случая → нет `NullPointerException`.

> **Rust-концепт — `.clone()`:** явная копия. `insert` забирает владение, поэтому `s.id.clone()`. Клонирование в Rust всегда явное.

### Шаг 1.5 — `match` над `Option`
```rust
match net.get("s01") {
    Some(s) => println!("found: {}", s.summary()),
    None => println!("void: no such shard"),
}
```

> 📖 **Читать:** главы **5** (structs), **6** (enum/`match`), **8** (Vec/String/HashMap).

### Шаг 1.6 — Тесты

В конце `net.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::shard::Shard;
    #[test]
    fn add_and_get() {
        let mut net = Net::new();
        net.add(Shard::new("a".into(), "Alpha".into()));
        assert_eq!(net.count(), 1);
        assert!(net.get("a").is_some());
        assert!(net.get("zzz").is_none());
    }
}
```
**Проверка:** `$ cargo test` → `1 passed`.

> **Rust-концепт — тесты в языке:** не нужен pytest; `cargo test` находит `#[test]`. Тесты рядом с кодом.

### Шаг 1.7 — Закрыть
```
$ cargo fmt && cargo clippy -- -D warnings && cargo test
$ git add -A && git commit -m "feat: in-memory Net storage with tests"
$ git switch main && git merge --no-ff phase-1-model
```

**✅ Фаза 1 Done when:** CRUD в памяти, тесты зелёные, прочитаны главы 4–6, 8.

---

# Фаза 2 — Сохранение на диск + CLI

**Ветка:** `phase-2-persistence`
**Цель:** заметки сохраняются в файлы и грузятся при старте; команды `new`, `list`, `show`.
**Rust-концепты:** трейты, `serde`, файловый I/O, `?`/`Result`, крейты, CLI (`clap`), пути (`PathBuf`).

### Шаг 2.1 — Ветка и зависимости
```
$ git switch main && git switch -c phase-2-persistence
$ cargo add serde --features derive
$ cargo add serde_json
$ cargo add clap --features derive
$ cargo add directories        # папка данных
$ cargo add anyhow             # ошибки
```

> **Rust-концепт — крейты и crates.io:** «крейт» = пакет (как PyPI). `cargo add` пишет в `Cargo.toml`, `cargo build` качает. Версии фиксирует `Cargo.lock` (коммить его!).

### Шаг 2.2 — Сериализуемый `Shard`
```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shard {
    pub id: ShardId,
    pub title: String,
    pub body: String,
}
```

> **Rust-концепт — трейты и `derive`:** «трейт» = набор поведения (как интерфейсы/ABC, но мощнее). `#[derive(...)]` авто-генерит реализацию. `Serialize/Deserialize` (serde), `Debug` (`{:?}`), `Clone` (`.clone()`).

> 📖 **Читать:** глава **10.2** (Traits).

### Шаг 2.3 — Хранилище на диске

`src/store.rs` — каждый Осколок = отдельный `.json`:
```rust
use std::{path::PathBuf, fs};
use anyhow::{Result, Context};
use directories::ProjectDirs;
use crate::shard::Shard;

pub fn data_dir() -> Result<PathBuf> {
    let proj = ProjectDirs::from("dev", "samos", "tether")
        .context("cannot determine home directory")?;
    let dir = proj.data_dir().to_path_buf();
    fs::create_dir_all(&dir).context("cannot create data dir")?;
    Ok(dir)
}

pub fn save_shard(shard: &Shard) -> Result<()> {
    let path = data_dir()?.join(format!("{}.json", shard.id));
    fs::write(&path, serde_json::to_string_pretty(shard)?)
        .with_context(|| format!("writing {:?}", path))?;
    Ok(())
}

pub fn load_all() -> Result<Vec<Shard>> {
    let mut shards = Vec::new();
    for entry in fs::read_dir(data_dir()?)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let text = fs::read_to_string(&path)?;
            shards.push(serde_json::from_str(&text)?);
        }
    }
    Ok(shards)
}
```

> **Rust-концепт — `Result<T,E>` и `?`:** ошибки — не исключения. Функция возвращает `Result` (`Ok`/`Err`). Оператор `?` = «если `Err` — выйди с ней, иначе разверни `Ok`». `anyhow::Result` = универсальный тип ошибки, `.context()` добавляет описание.

> 📖 **Читать:** глава **9** (Error handling).

> ⚠️ **Грабли:** `?` работает только в функции, возвращающей `Result`/`Option`. Сделай `fn main() -> anyhow::Result<()>` и в конце `Ok(())`.

### Шаг 2.4 — CLI через `clap`
```rust
mod shard; mod net; mod store;
use clap::{Parser, Subcommand};
use anyhow::Result;
use shard::Shard;

#[derive(Parser)]
#[command(name = "tether", about = "Weave shards across the Void")]
struct Cli { #[command(subcommand)] command: Command }

#[derive(Subcommand)]
enum Command {
    /// Create a new shard
    New { title: String },
    /// List all shards
    List,
    /// Show a shard by id
    Show { id: String },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::New { title } => {
            let id = format!("s{}", store::load_all()?.len() + 1);
            store::save_shard(&Shard::new(id.clone(), title))?;
            println!("tethered new shard: {}", id);
        }
        Command::List => for s in store::load_all()? { println!("{}", s.summary()); },
        Command::Show { id } => match store::load_all()?.into_iter().find(|s| s.id == id) {
            Some(s) => println!("{}\n\n{}", s.summary(), s.body),
            None => println!("void: no shard '{}'", id),
        },
    }
    Ok(())
}
```

**Проверка:**
```
$ cargo run -- new "First anchor in the void"
$ cargo run -- list
$ cargo run -- show s1
$ cargo run -- --help
```

> **Rust-концепт — enum с данными:** `Command::New { title }` несёт поля. `match` разбирает варианты и извлекает поля — идиоматичный способ моделировать «одно из состояний».

### Шаг 2.5 — Тест и закрытие

Round-trip тест (сохранить → загрузить → сравнить) с временной папкой: `$ cargo add tempfile --dev`.
Закрой фазу: fmt/clippy/test → merge → CHANGELOG.

**✅ Фаза 2 Done when:** `new/list/show` работают, данные переживают перезапуск, есть тест, прочитаны 9, 10.2.

---

# Фаза 3 — Кабели, backlinks, контекст, слаги

**Ветка:** `phase-3-links-context`
**Цель:** связи между Осколками, контекст рождения (время+cwd), **слаг** (человекочитаемый уникальный id из названия, транслит кириллицы).
**Rust-концепты:** `HashSet`, `chrono`, графы через id, `uuid`, транслитерация.

### Шаг 3.1 — Ветка и зависимости
```
$ git switch main && git switch -c phase-3-links-context
$ cargo add chrono --features serde
$ cargo add uuid --features "v4 serde"
$ cargo add slug                        # слаг + транслит кириллицы
```

> **Rust-концепт — два идентификатора:** `id` (uuid) — **стабильный** внутренний ключ, по нему хранятся связи (как первичный ключ в БД). `slug` — **человекочитаемый** ярлык из названия («Станция №7» → `stanciya-no7`), для упоминаний/CLI, может меняться при переименовании. Связи храним по `id`, не по `slug`.

### Шаг 3.2 — Расширить `Shard`
```rust
use std::collections::HashSet;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub created_at: DateTime<Utc>,
    #[serde(default)] pub cwd: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shard {
    pub id: ShardId,             // uuid, стабильный
    pub slug: String,            // человекочитаемый, уникальный
    pub title: String,
    pub body: String,
    #[serde(default)] pub links: HashSet<ShardId>,  // по id
    pub context: Context,
}
```
`new` заполняет `created_at = Utc::now()`, `cwd` через `std::env::current_dir()`.

> ⚠️ **Грабли — миграция:** добавил поля → старые `.json` без них. Помечай новые поля `#[serde(default)]` — serde подставит дефолты. Реальный урок про версионирование форматов.

### Шаг 3.3 — Слаг (транслит + уникальность)

`src/slugify.rs`:
```rust
pub fn base_slug(title: &str) -> String {
    let s = slug::slugify(title);
    if s.is_empty() { "shard".to_string() } else { s }
}

pub fn unique_slug(title: &str, taken: &dyn Fn(&str) -> bool) -> String {
    let base = base_slug(title);
    if !taken(&base) { return base; }
    let mut n = 2;
    loop {
        let cand = format!("{}-{}", base, n);
        if !taken(&cand) { return cand; }
        n += 1;
    }
}
```

> **Rust-концепт — транслит:** `slug` под капотом использует `deunicode` (Unicode → ASCII, в т.ч. кириллица `Привет`→`privet`), нижний регистр, дефисы.

> **Rust-концепт — функция как аргумент (`&dyn Fn`):** `taken` — предикат «занят ли слаг?». Принимаем её аргументом, чтобы `unique_slug` не знал про хранилище (проще тестировать). Вызов: `unique_slug(t, &|s| net.slug_exists(s))`.

**Проверка (тест):**
```rust
#[test]
fn slugs_unique() {
    let taken = |s: &str| ["void"].contains(&s);
    assert_eq!(unique_slug("Void", &taken), "void-2");
    assert_eq!(unique_slug("Совсем новое", &taken), "sovsem-novoe");
}
```
Встрой генерацию слага в создание Осколка (против существующих слагов в `Net`).

### Шаг 3.4 — Двусторонние связи

`net.rs`: метод `link(a_id, b_id)` кладёт `b` в `a.links` И `a` в `b.links`.

> **Rust-концепт — почему графы «неудобны»:** нельзя держать две `&mut` на элементы одного `HashMap` (защита от гонок). Меняем по очереди. Поэтому связи = **id**, а не указатели. В Rust графы делают через id/индексы.

### Шаг 3.5 — Команды и backlinks

- `tether link <a> <b>` (принимай id и slug).
- `tether show <id|slug>` — Осколок + связанные. Backlinks бесплатны (связи двусторонние).
- В CLI показывай `slug`, не длинный uuid.

Закрой фазу.

> 📖 **Читать:** глава **13** (итераторы/замыкания — начни).

**✅ Фаза 3 Done when:** ручная линковка, `show` показывает связи, у каждой заметки уникальный слаг и контекст.

---

# Фаза 4 — Захват (`capture`), коллекторы, fuzzy-поиск

**Ветка:** `phase-4-capture-search`
**Цель:** мгновенный захват мысли; выделенный текст и (опц.) скриншот; fuzzy-поиск.
**Rust-концепты:** `std::process::Command`, трейты (`ContextCollector`), итераторы/замыкания, fuzzy-крейт.

### Шаг 4.1 — Ветка и зависимости
```
$ git switch main && git switch -c phase-4-capture-search
$ cargo add nucleo-matcher
```
(Скриншот/буфер — системные утилиты `grim`, `slurp`, `wl-paste`, не cargo.)

### Шаг 4.2 — Трейт `ContextCollector`

`src/context/mod.rs`:
```rust
pub trait ContextCollector {
    fn collect(&self) -> Option<String>;
    fn name(&self) -> &'static str;
}
```

> **Rust-концепт — трейт как полиморфизм:** общий интерфейс, разные реализации. `Vec<Box<dyn ContextCollector>>` = «вектор любых коллекторов». `dyn` = динамическая диспетчеризация (как виртуальные функции), `Box` = данные в куче.

### Шаг 4.3 — Коллектор выделенного текста

`src/context/selection.rs`:
```rust
use std::process::Command;
use super::ContextCollector;

pub struct SelectionCollector;
impl ContextCollector for SelectionCollector {
    fn name(&self) -> &'static str { "selection" }
    fn collect(&self) -> Option<String> {
        let out = Command::new("wl-paste").arg("--primary").arg("--no-newline")
            .output().ok()?;
        if !out.status.success() { return None; }
        let text = String::from_utf8(out.stdout).ok()?;
        if text.trim().is_empty() { None } else { Some(text) }
    }
}
```

> **Rust-концепт — `std::process::Command`:** запуск внешних программ. `.output()` ждёт, возвращает stdout/stderr/статус. `.ok()?` → `Option`, выход `None` при ошибке.

**Проверка:** выдели текст мышкой → `cargo run -- capture "test"` → выделенное в заметке.

### Шаг 4.4 — Коллектор скриншота (по флагу)

`--shot full` → `grim <path>`; `--shot region` → `slurp` (геометрия) → `grim -g <geom> <path>`. Путь в `Context.screenshot_path: Option<String>`.

> ⚠️ **Грабли:** `grim -g "$(slurp)"` — шелловская подстановка, из `Command` так нельзя. Запусти `slurp`, возьми stdout, передай отдельным аргументом. Два `Command`.

### Шаг 4.5 — Команда `capture`

`tether capture [TITLE] [--shot <full|region>]`: создаёт Shard (со слагом), прогоняет коллекторы, сохраняет. Быстро, под хоткей:
```
# Hyprland
bind = $mod, N, exec, foot -e tether capture
bind = $mod SHIFT, N, exec, foot -e tether capture --shot region
# niri (config.kdl)
Mod+N       { spawn "foot" "-e" "tether" "capture"; }
Mod+Shift+N { spawn "foot" "-e" "tether" "capture" "--shot" "region"; }
```

### Шаг 4.6 — Fuzzy-поиск

`tether find <query>` через `nucleo-matcher`. Вынеси движок в `src/search.rs` — переиспользуем в TUI (Фаза 7) и в автодополнении (Фаза 8).

> **Rust-концепт — итераторы/замыкания:** `shards.iter().filter_map(|s| score(s,&q)).collect()`. `|s| ...` = замыкание. Цепочки итераторов — как генераторы в Python, но без лишних аллокаций.

> 📖 **Читать:** глава **13** целиком.

Закрой фазу.

**✅ Фаза 4 Done when:** `capture` ловит мысль+выделенное+(флаг)скрин; `find` ищет; движок в отдельном модуле.

---

# Фаза 5 — Упоминания `[[...]]` → автоматические связи

**Ветка:** `phase-5-mentions`
**Цель:** как в Obsidian — `[[Другая заметка]]` в теле создаёт связь. Тело — **источник правды** для исходящих связей; backlinks вычисляются.
**Rust-концепты:** парсинг (regex), индекс (`HashMap`), рефакторинг модели, множества.

### Шаг 5.1 — Ветка и зависимости
```
$ git switch main && git switch -c phase-5-mentions
$ cargo add regex
$ cargo add once_cell
```

### Шаг 5.2 — Парсер упоминаний

`src/mentions.rs`:
```rust
use once_cell::sync::Lazy;
use regex::Regex;

static MENTION_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[\[([^\]]+)\]\]").unwrap());

pub fn extract_mentions(body: &str) -> Vec<String> {
    MENTION_RE.captures_iter(body).map(|c| c[1].trim().to_string()).collect()
}
```

> **Rust-концепт — `Lazy`/`static` (regex один раз):** компиляция regex дорогая → держим в лениво-инициализируемой `static`. (В новых версиях есть `std::sync::LazyLock`.)

> **Rust-концепт — почему regex внешний:** в std Rust нет regex (осознанно — не тащить тяжёлое). Крейт `regex` — де-факто стандарт, очень быстрый.

### Шаг 5.3 — Резолвер: текст → id

Приводим текст упоминания к слагу, ищем Осколок. В `Net` — индекс `slug → id`:
```rust
pub fn resolve_mention(&self, text: &str) -> Option<ShardId> {
    let target = crate::slugify::base_slug(text);
    self.by_slug.get(&target).cloned()   // by_slug: HashMap<String, ShardId>
}
```
`by_slug` строй при загрузке `Net`.

> **Rust-концепт — вторичный индекс:** отдельный `HashMap<slug,id>` для поиска O(1) (как индекс в БД). Обновляй его в одном месте (методах `Net`), чтобы не рассинхронить.

### Шаг 5.4 — Рефакторинг: связи из тела

Меняем модель линковки (`refactor:`-коммит):
1. При **сохранении**: `extract_mentions(&body)` → резолвим → это **исходящие** связи (перезаписываем `shard.links`).
2. **Backlinks** — вычисляем на лету (кто ссылается на мой id).
3. `tether link a b` теперь **дописывает** `[[<title-of-b>]]` в тело A.

> **Rust-концепт — единый источник правды:** держать связи и в теле, и полем — путь к багам рассинхрона. Решение: тело = источник исходящих, `links` = кэш, backlinks = вычисляемы.

### Шаг 5.5 — Висячие упоминания (dangling)

`[[Ещё не созданная]]` не резолвится → «висячий кабель» (как серые ссылки в Obsidian). Не падай: показывай списком «dangling» + действие «создать из упоминания».

> **Rust-концепт — `Option` в дизайне:** висячее упоминание = `None`. Тип сам говорит «может не найтись», без исключений.

### Шаг 5.6 — Тесты

`extract_mentions`, авто-связь при сохранении, висячее не ломает. Закрой фазу.

**✅ Фаза 5 Done when:** `[[...]]` создаёт связи, backlinks считаются, dangling подсвечены, тело — источник правды.

---

# Фаза 6 — Кластеры (папки/подпапки) + ссылки на кластеры

**Ветка:** `phase-6-clusters`
**Цель:** заметки объединяются в кластеры и подкластеры (иерархия папок). Кластер отображается **как кластер** (дерево), а не как связи. Заметки из одного кластера могут ссылаться на заметки из других. Заметка может ссылаться и на **целый кластер**.
**Rust-концепты:** newtype-паттерн (`ClusterPath`), рекурсивное дерево (`BTreeMap`), enum как ключ `HashSet`, рефакторинг связей.

### Шаг 6.1 — Ветка

```
$ git switch main && git switch -c phase-6-clusters
```
Новых крейтов не нужно — дерево строим на `std::collections::BTreeMap` (даёт сортировку по ключу).

> **Решение по хранению (важно):** мы **не** раскладываем заметки по подпапкам на диске. Файлы остаются плоско `<id>.json`, а принадлежность к кластеру — это **поле** заметки. Почему: `id` стабилен, перемещение между кластерами = смена поля (а не move файла), связи не ломаются. Дерево кластеров строим в памяти из путей. (Раскладка по папкам на диске красивее для git, но ломает стабильность id при перемещениях — сознательно выбираем поле.)

### Шаг 6.2 — Тип пути кластера (newtype)

`src/cluster.rs`:
```rust
use serde::{Serialize, Deserialize};

// Путь кластера: ["samos", "stations"] = кластер samos/stations. Корень = пустой Vec.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct ClusterPath(pub Vec<String>);

impl ClusterPath {
    pub fn parse(s: &str) -> Self {
        ClusterPath(
            s.split('/').filter(|seg| !seg.is_empty())
             .map(|seg| crate::slugify::base_slug(seg))  // сегменты тоже слагируем
             .collect()
        )
    }
    pub fn to_string(&self) -> String { self.0.join("/") }
    // Является ли self потомком (или равным) other — для экспорта подкластеров.
    pub fn is_under(&self, other: &ClusterPath) -> bool {
        self.0.starts_with(&other.0)
    }
}
```
Добавь в `Shard`: `#[serde(default)] pub cluster: ClusterPath` (пустой = корень).

> **Rust-концепт — newtype-паттерн:** оборачиваем `Vec<String>` в свой тип `ClusterPath` вместо «голого» вектора. Плюсы: нельзя случайно перепутать с другим `Vec<String>`, вешаем методы (`is_under`, `parse`), derive нужных трейтов. Это идиоматичный Rust — придавать смысл примитивам через обёртку.

> **Rust-концепт — зачем столько derive:** `Ord`/`PartialOrd` → сортировка кластеров; `Hash`/`Eq` → использовать как ключ; `Default` → корневой пустой путь. Каждый трейт разблокирует конкретную возможность.

### Шаг 6.3 — Связь на заметку ИЛИ на кластер (рефакторинг)

Теперь цель связи — не только Осколок. Вводим enum и рефакторим `links`:
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LinkTarget {
    Shard(ShardId),
    Cluster(ClusterPath),
}
// было: links: HashSet<ShardId>
// стало:
#[serde(default)] pub links: HashSet<LinkTarget>,
```

> **Rust-концепт — enum как ключ множества:** чтобы класть `LinkTarget` в `HashSet`, он должен реализовать `Eq`+`Hash` — добавляем в `derive`. Enum с данными идеально моделирует «цель одного из двух типов» без наследования (в отличие от C++/Python, где ты бы делал базовый класс).

> ⚠️ **Грабли — снова миграция:** тип `links` сменился (`HashSet<ShardId>` → `HashSet<LinkTarget>`). Старый JSON не десериализуется в новый формат напрямую. Либо `#[serde(default)]` + пересборка связей из тела (у нас тело — источник правды из Фазы 5, так что связи всё равно перевычисляются!), либо разовый скрипт-миграция. Здесь Фаза 5 нас спасает: перечитали тела — связи восстановились.

### Шаг 6.4 — Синтаксис ссылки на кластер + резолвер

Различаем в `[[...]]`: **завершающий `/`** = кластер, иначе = заметка.
- `[[Натяжная станция №7]]` → заметка (по слагу).
- `[[samos/stations/]]` → кластер `samos/stations`.

Расширь резолвер из Фазы 5:
```rust
pub fn resolve_target(&self, text: &str) -> Option<LinkTarget> {
    if let Some(path) = text.strip_suffix('/') {
        Some(LinkTarget::Cluster(ClusterPath::parse(path)))  // кластер всегда «валиден»
    } else {
        self.resolve_mention(text).map(LinkTarget::Shard)     // заметка может не найтись
    }
}
```

> **Rust-концепт — `strip_suffix` и `map` на `Option`:** `strip_suffix('/')` возвращает `Option<&str>` (`Some` без слэша, если он был). `.map(LinkTarget::Shard)` оборачивает найденный id в вариант enum. Это цепочечный, «без if-лапши» стиль работы с `Option`.

### Шаг 6.5 — Дерево кластеров для отображения

`src/cluster.rs` — строим дерево из плоских путей всех заметок:
```rust
use std::collections::BTreeMap;
use crate::shard::ShardId;

#[derive(Default)]
pub struct ClusterNode {
    pub children: BTreeMap<String, ClusterNode>, // подкластеры (отсортированы)
    pub shards: Vec<ShardId>,                    // заметки прямо в этом кластере
}

pub fn build_tree(shards: &[(ShardId, ClusterPath)]) -> ClusterNode {
    let mut root = ClusterNode::default();
    for (id, path) in shards {
        let mut node = &mut root;
        for seg in &path.0 {
            node = node.children.entry(seg.clone()).or_default();
        }
        node.shards.push(id.clone());
    }
    root
}
```

> **Rust-концепт — рекурсивная структура через `BTreeMap`:** `ClusterNode` содержит `BTreeMap<String, ClusterNode>` — рекурсия! `BTreeMap` уже кладёт значения в кучу, поэтому `Box` тут не нужен (в отличие от `struct Node { child: Node }`, что не скомпилится без `Box`). `.entry(key).or_default()` = «возьми ветку или создай пустую» — классический паттерн построения дерева. `BTreeMap` держит подкластеры отсортированными по имени.

### Шаг 6.6 — CLI

- `tether new --cluster samos/stations "Title"` — создать в кластере.
- `tether mv <id|slug> <cluster/path>` — переместить заметку.
- `tether tree` — напечатать дерево кластеров с числом заметок (рекурсивный обход `ClusterNode`).
- `tether ls <cluster/path>` — заметки кластера (и подкластеров с флагом `--recursive`).
- Ссылка на кластер — через `[[cluster/path/]]` в теле или `tether link <a> "samos/stations/"`.

### Шаг 6.7 — Тесты

- `ClusterPath::parse("samos//Stations/")` → `["samos","stations"]` (пустые сегменты и регистр обработаны).
- `is_under`: `samos/stations/tension` под `samos` — true, под `void` — false.
- `build_tree` из нескольких путей даёт правильную вложенность.
- `resolve_target("x/")` → `Cluster`, `resolve_target("Заметка")` → `Shard`.

Закрой фазу.

**✅ Фаза 6 Done when:** заметки лежат в кластерах/подкластерах, `tree`/`ls`/`mv` работают, связь можно навести на кластер (`[[.../]]`), связи между кластерами работают.

---

# Фаза 7 — TUI на `ratatui` + Markdown + дерево кластеров

**Ветка:** `phase-7-tui`
**Цель:** интерактивный интерфейс: слева **дерево кластеров**, в нём — Осколки; просмотр с Markdown-рендером; навигация по кабелям и упоминаниям; поиск.
**Rust-концепты:** событийный цикл, состояние (`struct App` + enum `Mode`), модульная организация.

### Шаг 7.1 — Ветка и зависимости
```
$ git switch main && git switch -c phase-7-tui
$ cargo add ratatui
$ cargo add crossterm
$ cargo add tui-markdown
```

### Шаг 7.2 — Скелет: alt-screen + цикл

`ratatui::init()` / `ratatui::restore()`. Цикл: `terminal.draw(|f| ui(f,&app))?;` → чтение клавиши. `q`/`Esc` — выход с восстановлением.

> ⚠️ **Грабли:** паника в raw mode ломает терминал. Восстановление должно вызываться даже при панике (`ratatui::restore()` + хук паники). Иначе после краша набирай `reset` вслепую.

### Шаг 7.3 — Состояние
```rust
struct App {
    net: Net,
    tree: ClusterNode,           // дерево кластеров (Фаза 6)
    selected_cluster: ClusterPath,
    selected: usize,
    mode: Mode,
    query: String,
    should_quit: bool,
}
enum Mode { Browsing, Searching, Viewing(ShardId) }
```

> **Rust-концепт — состояние через enum:** вместо флагов `is_searching`/`is_viewing` — один `Mode`. `match app.mode` решает поведение; несогласованные состояния невозможны.

### Шаг 7.4 — Отрисовка + Markdown + дерево

`Layout`: слева дерево кластеров (рекурсивный обход `ClusterNode` с отступами/раскрытием), в центре — заметки выбранного кластера, справа — тело (Markdown) + кабели/backlinks/ссылки-на-кластеры.
```rust
let rendered = tui_markdown::from_str(&shard.body);
frame.render_widget(Paragraph::new(rendered), area);
```

> ⚠️ **Грабли:** держи `ui(frame,&app)` только читающей `&app`; изменение состояния — в обработчике клавиш. Не мешай ввод и отрисовку.

### Шаг 7.5 — Навигация

`j/k` — по элементам; `h/l` или `Enter` — заходить в кластер/разворачивать; `Enter` на заметке — просмотр; прыжки по `[[упоминаниям]]` и по ссылкам-на-кластеры (переход к кластеру); `/` — поиск (переиспользуй `search.rs`, Фаза 4); `n` — захват; `q` — выход.

### Шаг 7.6 — Причесать модули
```
src/
├── main.rs
├── model/   (shard, net, context)
├── cluster.rs
├── store.rs
├── context/
├── mentions.rs
├── slugify.rs
├── search.rs
└── tui/     (app.rs, ui.rs, events.rs)
```

> 📖 **Читать:** глава **7** (модули) — на своём коде.

Закрой фазу.

**✅ Фаза 7 Done when:** TUI показывает дерево кластеров, заходишь в кластеры, смотришь заметки с Markdown, прыгаешь по упоминаниям и ссылкам-на-кластеры.

---

# Фаза 8 — Редактор в TUI + автодополнение упоминаний (Tab)

**Ветка:** `phase-8-editor-autocomplete`
**Цель:** редактировать тело в TUI; при вводе `[[` — popup с кандидатами (заметки **и** кластеры), fuzzy-фильтр, `Tab`/`Enter` подставляет. Связи обновляются вживую.
**Rust-концепты:** вложенные машины состояний, overlay-виджеты, переиспользование модулей, UTF-8.

### Шаг 8.1 — Ветка и зависимости
```
$ git switch main && git switch -c phase-8-editor-autocomplete
$ cargo add tui-textarea
```

> **Rust-концепт — не изобретай редактор:** `tui-textarea` даёт готовый виджет (курсор, многострочность, Unicode). Мы лишь перехватываем `[[`. Брать правильный крейт вместо велосипеда — инженерный навык.

### Шаг 8.2 — Режим редактирования

`Mode::Editing`. Открытие по `e`. По `Ctrl-S`/`Esc` — сохранить тело и **переспарсить упоминания** (Фаза 5+6) → связи обновятся.

### Шаг 8.3 — Триггер `[[`

Набрал `[[` → под-режим `PickingMention`: символы копятся в `query`.

> **Rust-концепт — вложенная машина состояний:** состояние редактора имеет под-состояние. Смоделируй enum `EditorState { Typing, PickingMention { query: String, selected: usize } }`. Компилятор гарантирует обработку клавиш по текущему под-состоянию.

### Шаг 8.4 — Popup кандидатов (заметки + кластеры)

Кандидаты: заголовки заметок **и** пути кластеров (кластеры показывай с `/` на конце — так же, как в синтаксисе). Fuzzy-фильтр по `query` (переиспользуй `search.rs`). Overlay:
```rust
frame.render_widget(Clear, popup_area);
frame.render_widget(candidates_list, popup_area);
```

> **Rust-концепт — overlay:** `Clear` затирает область, поверх рисуем popup. Позиционируй у курсора.

### Шаг 8.5 — Подстановка

`Tab`/`Enter`: заменить `[[частичный` на `[[Полное Название]]` (или `[[cluster/path/]]`) в буфере `tui-textarea`. `Esc` — отмена.

> ⚠️ **Грабли — UTF-8:** строки в Rust — UTF-8, `s[3]` НЕ скомпилится, кириллица = 2 байта/символ. Правь буфер через API `tui-textarea` (корректная позиция курсора), не режь строки по числовым индексам. Если режешь — по `char_indices()`.

### Шаг 8.6 — Живое обновление

После закрытия редактора перезапусти линковку (Фаза 5): разобрать `[[...]]`, обновить `links`, пересчитать backlinks. Новые кабели видны сразу.

Закрой фазу.

**✅ Фаза 8 Done when:** редактируешь тело; `[[` даёт fuzzy-подсказки по заметкам и кластерам; `Tab` подставляет; связи обновляются вживую.

---

# Фаза 9 — Эстетика Samos

**Ветка:** `phase-9-aesthetic`
**Цель:** атмосферный инструмент Set'а.
**Rust-концепты:** конфиги (`toml`+`serde`), стили.

1. **Тема:** хром/сталь/пустота; акцентный «сигнальный» цвет для кабелей и `[[упоминаний]]`; кластеры — как «созвездия» (свой оттенок). `ratatui::style`.
2. **Тексты в лоре:** «tethered new shard», «void: no such shard», «cable strung», статус-бар: «anchored shards», «cables under tension», «dangling mentions», «clusters».
3. **ASCII-лого** при старте.
4. **Конфиг** `~/.config/tether/config.toml` (`$ cargo add toml`): цвета, коллекторы, путь данных, скрин по умолчанию, символ упоминаний/кластеров.
5. **(Опц.) «резонанс»:** пустой экран Пустоты, тонкая «искажение»-анимация в статус-баре.

Закрой фазу. Тег MVP: `$ git tag v0.1.0`.

**✅ Фаза 9 Done when:** выглядит и звучит как Samos; настраивается через конфиг.

---

# Фаза 10 — Экспорт / импорт + лавинный экспорт

**Ветка:** `phase-10-export-import`
**Цель:** переносить отдельные заметки, целые кластеры/подкластеры на другую машину или в репозиторий. **Лавинный экспорт** — тянуть за собой все заметки, на которые ссылаются выбранные (транзитивное замыкание связей).
**Rust-концепты:** обход графа (BFS/DFS, `VecDeque`+`HashSet`), сериализация «бандла», стратегии разрешения конфликтов (enum), опц. архивы (`tar`+`flate2`).

### Шаг 10.1 — Ветка

```
$ git switch main && git switch -c phase-10-export-import
```
Базовый формат бандла — один JSON-файл (без новых крейтов). Опционально позже — `.tar.gz`: `$ cargo add tar && cargo add flate2`.

### Шаг 10.2 — Формат бандла

`src/bundle.rs`:
```rust
#[derive(Serialize, Deserialize)]
pub struct Bundle {
    pub version: u32,
    pub exported_at: chrono::DateTime<chrono::Utc>,
    pub shards: Vec<crate::shard::Shard>,   // полные Осколки со связями и кластерами
}
```
Экспорт = сериализовать `Bundle` в `.tether.json`. Связи внутри едут по `id`, поэтому если цель включена в бандл — связь переживёт перенос.

> **Rust-концепт — версионируй форматы обмена:** поле `version` в бандле — чтобы будущий импорт понимал старые файлы. Дешёвая привычка, спасающая от боли совместимости.

### Шаг 10.3 — Выбор набора для экспорта

Три режима:
```rust
pub enum Selection {
    Shards(Vec<ShardId>),          // конкретные заметки
    Cluster(ClusterPath),          // кластер + все подкластеры (is_under)
}
```
- **Кластер:** отбери все заметки, чей `cluster.is_under(&target)` (из Фазы 6).
- **Лавина (`--avalanche`):** от набора-семени обойди граф связей и добавь всех достижимых:
```rust
use std::collections::{VecDeque, HashSet};

pub fn avalanche(net: &Net, seeds: &[ShardId]) -> HashSet<ShardId> {
    let mut visited = HashSet::new();
    let mut queue: VecDeque<ShardId> = seeds.iter().cloned().collect();
    while let Some(id) = queue.pop_front() {
        if !visited.insert(id.clone()) { continue; } // уже были — пропустить
        if let Some(shard) = net.get(&id) {
            for target in &shard.links {
                if let LinkTarget::Shard(next) = target {  // ссылки на кластеры не тянем заметки
                    if !visited.contains(next) { queue.push_back(next.clone()); }
                }
            }
        }
    }
    visited
}
```

> **Rust-концепт — обход графа (BFS):** очередь `VecDeque` (двусторонняя очередь, `pop_front`/`push_back`) + множество `visited` (защита от циклов). `visited.insert(x)` возвращает `false`, если уже был — компактная проверка «первый ли раз вижу». Это фундаментальный алгоритм; ты применяешь его к реальному графу заметок.

> **Rust-концепт — `if let` внутри цикла:** `if let LinkTarget::Shard(next) = target` разбирает enum и берёт только связи-на-заметки (ссылки-на-кластеры лавину заметок не расширяют — это осознанное решение; при желании добавь флаг «тянуть и содержимое кластеров»).

### Шаг 10.4 — Команда экспорта

```
tether export <id|slug>            --out bundle.tether.json
tether export --cluster samos/stations  [--avalanche]  --out ...
tether export <id> --avalanche     --out ...     # заметка + всё, на что ссылается
```
Собери множество id → достань Осколки → `Bundle` → записать файл. Печатай сводку: сколько заметок, из скольких кластеров, сколько висячих связей осталось за бортом.

### Шаг 10.5 — Импорт + конфликты

```rust
pub enum OnConflict { Skip, Overwrite, Duplicate }  // что делать, если id уже есть
```
`tether import bundle.tether.json [--into <cluster/path>] [--on-conflict skip|overwrite|duplicate]`:
1. Прочитать `Bundle`.
2. Для каждой заметки:
   - `Skip` — если id уже есть, пропустить;
   - `Overwrite` — заменить;
   - `Duplicate` — выдать **новый** `id` (и пере-слагировать при коллизии слага).
3. `--into` — подставить путь-префикс кластера ко всем импортируемым (`ClusterPath` = префикс + исходный путь).
4. Пере-слагирование: слаги должны остаться уникальны в целевой сети (переиспользуй `unique_slug` из Фазы 3).

> **Rust-концепт — стратегия как enum:** поведение при конфликте — не булев флаг, а `enum OnConflict` с явными вариантами. `match` заставит обработать все — не забудешь случай. Так моделируют «политики/режимы» в Rust.

### Шаг 10.6 — Висячие связи после импорта

Если импортировали заметку, чья связь ведёт на невключённый id — связь становится **висячей** (как dangling из Фазы 5). Не падай, покажи. Лавинный экспорт как раз для того, чтобы таких дыр не было.

### Шаг 10.7 — TUI-действия и тесты

- TUI: на кластере/заметке — `x` экспорт (с переключателем лавины), команда импорта.
- Тесты: `avalanche` даёт правильное замыкание (включая защиту от циклов A→B→A); round-trip export→import сохраняет заметки/связи/кластеры; `Duplicate` не ломает уникальность слагов; `--into` сдвигает пути.

Закрой фазу. Тег: `$ git tag v0.2.0`.

**✅ Фаза 10 Done when:** экспорт заметки/кластера/подкластеров; `--avalanche` тянет транзитивные связи; импорт с выбором стратегии конфликтов и префиксом кластера; всё переносится между машинами/репозиторием.

---

# Фаза 11 (опционально) — Фоновый индексатор-демон

**Ветка:** `phase-11-daemon`
**Цель:** фоновый процесс держит поисковый индекс горячим.
**Rust-концепты:** async/`tokio`, каналы, `Arc<Mutex<T>>`, слежение за файлами (`notify`), IPC (unix-сокет).

> Делай, ТОЛЬКО когда база уверенная — много новой сложности сразу.

1. `$ cargo add tokio --features full` и `$ cargo add notify`.
2. `tether daemon`: через `notify` следит за папкой данных, перестраивает индекс.
3. Индекс за `Arc<Mutex<Index>>`. **Rust-концепт:** `Arc` = потокобезопасный счётчик ссылок, `Mutex` = блокировка. Так Rust даёт «fearless concurrency».
4. IPC: `tether find` шлёт запрос демону через unix-сокет; демон выключен → падаем на локальный поиск (graceful).
5. Systemd user-сервис `~/.config/systemd/user/tether.service`.

> 📖 **Читать:** глава **16** (concurrency) + async-book (`https://rust-lang.github.io/async-book/`).

**✅ Фаза 11 Done when:** демон держит индекс, `find` мгновенный, без демона всё работает.

---

# Фаза 12 (опционально) — «Круги» (Circles): магия как декларативные схемы

**Ветка:** `phase-12-circles`
**Цель:** магия Samos как фича — декларативные шаблоны заметок (алхимические круги).
**Rust-концепты:** парсинг схем, generics, трейты с ассоциированными типами, паттерн «команда».

Идея: «Круг» — файл-схема (`.toml`): шаблон Осколка (поля, кластер по умолчанию, авто-упоминания, коллекторы контекста, теги). Пример: круг `journal` создаёт заметку с датой в заголовке, в кластере `journal/`, с `[[Дневник дня]]` и захватом медиа.

1. Формат схемы (`toml`).
2. Загрузка из `~/.config/tether/circles/`.
3. `tether cast <circle> [args]` — «начертить круг».
4. Обобщить создание через трейт `ShardTemplate`.

**✅ Фаза 12 Done when:** свой «круг» в конфиге, создание Осколка по нему одной командой.

---

# Приложение A — Шпаргалка команд

```
# качество (в конце каждой фазы)
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo build --release

# зависимости
cargo add <crate>
cargo add <crate> --features x,y
cargo add <crate> --dev
cargo remove <crate>
cargo update

# git-флоу фазы
git switch -c phase-N-slug
git add -A && git commit -m "feat: ..."
git switch main && git merge --no-ff phase-N-slug
git tag v0.1.0

# запуск
cargo run -- <args>
cargo run -- --help
```

# Приложение B — Системные зависимости (пакетный менеджер, НЕ cargo)

- `wl-clipboard` (`wl-paste`) — выделенный текст/буфер.
- `grim` — скриншот (Wayland/wlroots: niri, hyprland).
- `slurp` — выбор области мышью.
- терминал для биндов (`foot`, `alacritty`, `kitty`).

# Приложение C — Порядок чтения The Rust Book (по фазам)

| Фаза | Главы / материал |
|------|------------------|
| 0 | 1–3 (+ Rustlings) |
| 1 | 4 (Ownership!), 5, 6, 8 |
| 2 | 9 (ошибки), 10.2 (трейты) |
| 3 | 10 (generics/lifetimes), 13 (начать) |
| 4 | 13 (итераторы/замыкания) |
| 5 | — (парсинг; доки `regex`) |
| 6 | 8 (коллекции), newtype-паттерн; рекурсивные структуры |
| 7 | 7 (модули); доки `ratatui` |
| 8 | машины состояний; доки `tui-textarea` |
| 10 | графы/BFS (`VecDeque`, `HashSet`) |
| 11 | 16 (concurrency), async-book |

# Приложение D — Ключевые крейты по фазам

| Крейт | Фаза | Зачем |
|-------|------|-------|
| serde, serde_json | 2 | сериализация заметок |
| clap | 2 | CLI-аргументы |
| directories | 2 | папка данных |
| anyhow | 2 | ошибки |
| chrono | 3 | время создания |
| uuid | 3 | стабильные id |
| slug | 3 | слаг + транслит кириллицы |
| nucleo-matcher | 4 | fuzzy-поиск |
| regex, once_cell | 5 | парсинг `[[упоминаний]]` |
| — (BTreeMap из std) | 6 | дерево кластеров |
| ratatui, crossterm | 7 | TUI |
| tui-markdown | 7 | рендер Markdown |
| tui-textarea | 8 | редактор + автодополнение |
| toml | 9 | конфиг |
| tar, flate2 (опц.) | 10 | архив бандла `.tar.gz` |
| tokio, notify | 11 | демон (опц.) |

---

_Двигайся по одной фазе. Не прыгай вперёд — каждая опирается на предыдущую. Когда упрёшься — спрашивай по конкретному шагу._
