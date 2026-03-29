# Стиль кода

## Именование

| Сущность | Стиль | Пример |
|---|---|---|
| Крейты | `snake_case` (в `Cargo.toml` через `-`) | `mega-math-matrix` |
| Модули, функции, методы, переменные | `snake_case` | `compute_rank`, `row_count` |
| Типы, трейты, перечисления | `UpperCamelCase` | `Matrix`, `IntoIterator` |
| Варианты enum | `UpperCamelCase` | `Some`, `InvalidIndex` |
| Константы, статические переменные | `SCREAMING_SNAKE_CASE` | `MAX_SIZE`, `DEFAULT_EPSILON` |
| Параметры типов | одна заглавная буква или короткое слово | `T`, `E`, `Item` |
| Lifetime-параметры | короткие, строчные | `'a`, `'de` |
| Макросы | `snake_case!` | `vec!`, `assert_eq!` |

## Форматирование

- Используется `rustfmt` (`make fmt-all`). Настройки — в `rustfmt.toml`.
- Максимальная длина строки — **120 символов**.
- Отступы — **4 пробела**, без табов.
- Запятая после последнего элемента в многострочных конструкциях (trailing comma).

## Структура файла

Рекомендуемый порядок элементов внутри файла:

1. `use` импорты (группировка: `std` → сторонние крейты → крейты workspace → `crate`/`super`)
2. Константы
3. Типы (struct, enum)
4. Реализации (`impl`)
5. Реализации трейтов (`impl Trait for Type`)
6. Приватные вспомогательные функции
7. Тесты (`#[cfg(test)]` в конце файла)

## Импорты

```rust
// std
use std::fmt;
use std::ops::{Add, Mul};

// Сторонние крейты
use serde::{Deserialize, Serialize};

// Крейты workspace
use mega_math_vector::Vector;

// Текущий крейт
use crate::storage::Storage;
use super::utils;
```

- Группы разделяются пустой строкой.
- Внутри группы — алфавитный порядок (`rustfmt` делает это автоматически).
- Избегайте glob-импортов (`use std::ops::*`), кроме `prelude` модулей и тестов.

## Документация

- Публичный API документируется через `///`.
- Модули документируются через `//!` в начале файла.
- Комментарии в коде (`//`) — только для неочевидной логики.

```rust
/// Создаёт единичную матрицу заданного размера.
///
/// # Examples
///
/// ```
/// use mega_math_matrix::Matrix;
///
/// let m = Matrix::identity(3);
/// assert_eq!(m[(0, 0)], 1.0);
/// ```
pub fn identity(size: usize) -> Self {
    // ...
}
```

## Обработка ошибок

- Для публичного API — собственные типы ошибок и `Result`.
- `unwrap()` / `expect()` — только в тестах и прототипах (Clippy предупредит через `unwrap_used`).
- `panic!` — только при нарушении инвариантов, которые невозможно выразить через типы.

```rust
// Плохо
let value = map.get("key").unwrap();

// Хорошо
let value = map.get("key").ok_or(Error::KeyNotFound("key"))?;
```

## Типы и трейты

- Предпочитайте **новые типы** (newtype pattern) вместо голых примитивов для доменных значений.
- Реализуйте стандартные трейты (`Display`, `Debug`, `Clone`, `PartialEq`) через `#[derive]` где возможно.
- `Default` реализуйте только если есть осмысленное значение по умолчанию.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}
```

## Тесты

- Юнит-тесты — в том же файле, в блоке `#[cfg(test)]`.
- Интеграционные тесты — в директории `tests/` крейта.
- Имена тестов описывают поведение: `addition_of_different_sizes_returns_error`.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_matrix_has_ones_on_diagonal() {
        let m = Matrix::<f64>::identity(3);
        for i in 0..3 {
            assert!((m[(i, i)] - 1.0).abs() < f64::EPSILON);
        }
    }
}
```

## unsafe

- Запрещён на уровне Clippy (`unsafe_code = "deny"`).
- Если потребуется — выносить в отдельный модуль с обоснованием в `// SAFETY:` комментарии и разрешать точечно через `#[allow(unsafe_code, reason = "...")]`.
