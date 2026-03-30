mod add;
mod mul;
mod neg;
mod sub;
mod transpose;

#[derive(Debug, Clone, PartialEq)]
#[allow(
    clippy::derive_partial_eq_without_eq,
    reason = "Matrix<f64> не может реализовать Eq"
)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<Vec<T>>,
}

impl<T: Default + Clone> Matrix<T> {
    /// Создаёт матрицу заданного размера, заполненную значениями `T::default()`.
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![vec![T::default(); cols]; rows],
        }
    }
}

impl<T> Matrix<T> {
    /// Создаёт матрицу из готовых данных (move, без копирования).
    pub fn from_data(data: Vec<Vec<T>>) -> Self {
        let rows = data.len();
        let cols = data.first().map_or(0, Vec::len);
        Self { rows, cols, data }
    }

    pub const fn rows(&self) -> usize {
        self.rows
    }

    pub const fn cols(&self) -> usize {
        self.cols
    }

    pub const fn is_square(&self) -> bool {
        self.rows == self.cols
    }
}
