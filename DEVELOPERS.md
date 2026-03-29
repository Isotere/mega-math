# Руководство разработчика

## Модули внутри крейта

### Структура файлов = структура модулей

В современном Rust (начиная с edition 2018) структура файлов напрямую отражает дерево модулей. Файл `mod.rs` **не используется** — вместо этого модуль-директория объявляется одноимённым файлом рядом.

```
src/
├── lib.rs            # корень крейта
├── storage.rs        # модуль storage
├── storage/          # подмодули storage
│   ├── memory.rs     # storage::memory
│   └── disk.rs       # storage::disk
├── ops.rs            # модуль ops
└── ops/              # подмодули ops
    ├── add.rs        # ops::add
    └── mul.rs        # ops::mul
```

### Объявление модулей

Модуль нужно **объявить** через `mod` в родительском файле — просто создать файл недостаточно.

`lib.rs`:
```rust
mod storage;
mod ops;
```

`storage.rs`:
```rust
mod memory;
mod disk;
```

### Видимость

По умолчанию всё приватное. `pub` открывает доступ родительскому модулю, `pub(crate)` — всему крейту.

```rust
// storage.rs
mod memory;
mod disk;

// Реэкспорт публичного API модуля
pub use memory::MemoryStorage;
pub use disk::DiskStorage;
```

```rust
// storage/memory.rs
pub struct MemoryStorage { /* ... */ }

// Доступно только внутри крейта
pub(crate) fn internal_helper() { /* ... */ }
```

### Реэкспорт через `lib.rs`

Публичный API крейта формируется реэкспортом из `lib.rs`:

```rust
// lib.rs
mod storage;
mod ops;

pub use storage::MemoryStorage;
pub use storage::DiskStorage;
```

Пользователи крейта будут писать:
```rust
use mega_math_matrix::MemoryStorage;
```

а не:
```rust
use mega_math_matrix::storage::memory::MemoryStorage;
```

### Правила

- **Один файл — один модуль.** Не складывайте несвязанные типы в один файл.
- **`mod.rs` не используется.** Это устаревший стиль (до edition 2018).
- **`mod` объявляет, `use` импортирует.** `mod` создаёт модуль в дереве, `use` подтягивает пути для удобства.
- **Реэкспортируйте публичный API.** Внутренняя структура модулей — деталь реализации, пользователь крейта не должен знать о ней.

## Организация кода в крейте

### Структура по доменным понятиям

Модули организуются по доменным понятиям, а не по техническим слоям. Тип, его трейты, ошибки и реализации живут рядом.

Плохо (слои):
```
src/
├── types/
│   ├── matrix.rs
│   └── vector.rs
├── traits/
│   ├── matrix.rs
│   └── vector.rs
└── errors/
    ├── matrix.rs
    └── vector.rs
```

Хорошо (домен):
```
src/
├── lib.rs
├── matrix.rs
├── matrix/
│   ├── ops.rs
│   ├── decomposition.rs
│   └── error.rs
├── vector.rs
└── vector/
    ├── ops.rs
    └── error.rs
```

### Общие типы и трейты

Типы и трейты, специфичные для одного модуля, живут в нём. Общие для нескольких модулей — выносятся отдельно:

```
src/
├── lib.rs
├── traits.rs     # общие трейты: Norm, Transpose
├── error.rs      # общий тип ошибок крейта
├── matrix.rs
└── vector.rs
```

Если тип используется только в одном месте — он **не** общий, не выносите его заранее.

## Организация workspace

### Крейт `core` как общее ядро

Когда нескольким крейтам нужны общие трейты и типы, они выносятся в `mega-math-core`:

```
crates/
├── core/         # общие трейты, типы
├── matrix/       # зависит от core
├── vector/       # зависит от core
└── calculus/     # зависит от core, matrix, vector
```

Пример `crates/core/src/lib.rs`:
```rust
mod traits;
mod scalar;
mod error;

pub use traits::{Norm, Transpose, DotProduct};
pub use scalar::Scalar;
pub use error::Error;
```

Крейты подключают его как зависимость:
```toml
[dependencies]
mega-math-core = { path = "../core" }
```

### Принципы

| Принцип | Реализация в Rust |
|---|---|
| Доменная область | Отдельный крейт в workspace |
| Сущность / значение | Struct с методами |
| Доменная логика | Функции или трейты |
| Общее ядро | Крейт `core` с общими трейтами и типами |
| Единый язык | Именование типов и методов в терминах домена |
