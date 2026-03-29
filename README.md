# mega-math

## Makefile

### Workspace

| Команда | Описание |
|---|---|
| `make build-all` | Сборка всех крейтов |
| `make build-all PROFILE=release` | Сборка всех крейтов в release |
| `make test-all` | Запуск тестов для всех крейтов |
| `make lint-all` | Запуск Clippy на весь workspace |
| `make fmt-all` | Форматирование всех крейтов |
| `make clean` | Удаление директории `target/` |

### Конкретный крейт

| Команда | Описание |
|---|---|
| `make build <crate>` | Сборка крейта |
| `make build <crate> PROFILE=release` | Сборка крейта в release |
| `make check <crate>` | Проверка крейта без генерации бинарников |
| `make test <crate>` | Запуск тестов крейта |
| `make new-crate <crate> DESCRIPTION="..."` | Создание нового крейта из шаблона |

## Создание нового крейта

Все крейты располагаются в `crates/`. Новый крейт подхватывается автоматически благодаря `members = ["crates/*"]` в корневом `Cargo.toml`.

### 1. Создать структуру

```bash
mkdir -p crates/<name>/src
```

### 2. Создать `crates/<name>/Cargo.toml`

```toml
[package]
name = "mega-math-<name>"
description = "..."
version.workspace = true
edition.workspace = true
license.workspace = true

[lints]
workspace = true

[dependencies]
```

- **Имя** крейта — `mega-math-<name>`, по конвенции workspace
- **version**, **edition**, **license** — наследуются из workspace
- **`[lints] workspace = true`** — обязательно, иначе настройки Clippy не применятся

### 3. Создать `crates/<name>/src/lib.rs`

После этого можно проверить:

```bash
cargo check --workspace
```

## Добавление зависимостей

Чтобы версии зависимостей были одинаковыми во всех крейтах, используйте зависимости на уровне workspace.

### 1. Добавить зависимость в корень workspace

```bash
cargo add serde --workspace
```

Это добавит зависимость в `[workspace.dependencies]` корневого `Cargo.toml`.

### 2. Подключить её в конкретном крейте

```bash
cargo add serde -p mega-math-matrix --features derive
```

Если зависимость уже объявлена в `[workspace.dependencies]`, команда автоматически установит `workspace = true` в `Cargo.toml` крейта.

### Результат

Корневой `Cargo.toml`:
```toml
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
```

`Cargo.toml` крейта:
```toml
[dependencies]
serde = { workspace = true }
```

Это гарантирует единую версию зависимости во всех крейтах и одно место для её обновления.