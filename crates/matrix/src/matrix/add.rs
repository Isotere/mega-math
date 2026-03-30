use super::Matrix;
use crate::MatrixError;
use std::ops::{Add, AddAssign};

impl<T: Copy> Matrix<T> {
    /// Складывает две матрицы, возвращая `Err` при несовпадении размерностей.
    ///
    /// В отличие от оператора `+`, не паникует, а возвращает [`MatrixError::DimensionMismatch`].
    /// Исходные матрицы остаются неизменными — результат аллоцируется отдельно.
    pub fn try_add(&self, rhs: &Self) -> Result<Self, MatrixError>
    where
        T: Add<Output = T>,
    {
        if self.cols != rhs.cols || self.rows != rhs.rows {
            return Err(MatrixError::DimensionMismatch);
        }

        let data: Vec<Vec<T>> = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(self_row, rhs_row)| {
                self_row
                    .iter()
                    .zip(rhs_row.iter())
                    .map(|(s, r)| *s + *r)
                    .collect()
            })
            .collect();
        Ok(Self::from_data(data))
    }
}

impl<T: AddAssign + Copy> AddAssign for Matrix<T> {
    /// Прибавляет `rhs` к текущей матрице на месте (`+=`).
    ///
    /// # Panics
    ///
    /// Паникует при несовпадении размерностей матриц.
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.rows, rhs.rows, "Matrix dimensions mismatch");
        assert_eq!(self.cols, rhs.cols, "Matrix dimensions mismatch");

        self.data
            .iter_mut()
            .zip(rhs.data.iter())
            .for_each(|(self_row, rhs_row)| {
                self_row
                    .iter_mut()
                    .zip(rhs_row.iter())
                    .for_each(|(s, r)| *s += *r);
            });
    }
}

impl<T: AddAssign + Copy> Add for Matrix<T> {
    type Output = Self;

    /// Складывает две матрицы (`+`). Переиспользует память `self`, избегая лишних аллокаций.
    ///
    /// # Panics
    ///
    /// Паникует при несовпадении размерностей матриц.
    /// Для безопасного варианта используйте [`Matrix::try_add`].
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- try_add ---

    /// Поэлементное сложение двух матриц одинакового размера
    #[test]
    fn try_add_same_dimensions() {
        let a = Matrix::from_data(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let b = Matrix::from_data(vec![vec![10, 20, 30], vec![40, 50, 60]]);

        let result = a.try_add(&b).expect("dimensions match");

        assert_eq!(result.rows(), 2);
        assert_eq!(result.cols(), 3);
        assert_eq!(
            result,
            Matrix::from_data(vec![vec![11, 22, 33], vec![44, 55, 66],])
        );
    }

    /// Несовпадение размерностей возвращает ошибку, а не панику
    #[test]
    fn try_add_dimension_mismatch() {
        let a = Matrix::from_data(vec![vec![1, 2]]);
        let b = Matrix::from_data(vec![vec![1, 2, 3]]);

        assert!(a.try_add(&b).is_err());
    }

    /// Исходные матрицы не изменяются после ``try_add``
    #[test]
    fn try_add_does_not_consume_operands() {
        let a = Matrix::from_data(vec![vec![1, 2]]);
        let b = Matrix::from_data(vec![vec![3, 4]]);

        let _ = a.try_add(&b);

        // a и b по-прежнему доступны
        assert_eq!(a.rows(), 1);
        assert_eq!(b.rows(), 1);
    }

    /// Сложение с нулевой матрицей не меняет значения
    #[test]
    fn try_add_with_zero_matrix() {
        let a = Matrix::from_data(vec![vec![5, 10], vec![15, 20]]);
        let zero = Matrix::new(2, 2);

        let result = a.try_add(&zero).expect("dimensions match");

        assert_eq!(result, a);
    }

    // --- Add (оператор +) ---

    /// Оператор + складывает матрицы поэлементно
    #[test]
    fn add_operator() {
        let a = Matrix::from_data(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::from_data(vec![vec![5, 6], vec![7, 8]]);

        let result = a + b;

        assert_eq!(result, Matrix::from_data(vec![vec![6, 8], vec![10, 12]]));
    }

    /// Оператор + паникует при несовпадении размерностей
    #[test]
    #[should_panic(expected = "Matrix dimensions mismatch")]
    fn add_operator_panics_on_mismatch() {
        let a = Matrix::from_data(vec![vec![1, 2]]);
        let b = Matrix::from_data(vec![vec![1], vec![2]]);

        let _ = a + b;
    }

    // --- AddAssign (оператор +=) ---

    /// Оператор += модифицирует матрицу на месте
    #[test]
    fn add_assign_operator() {
        let mut a = Matrix::from_data(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::from_data(vec![vec![10, 20], vec![30, 40]]);

        a += b;

        assert_eq!(a, Matrix::from_data(vec![vec![11, 22], vec![33, 44]]));
    }

    /// Оператор += паникует при несовпадении размерностей
    #[test]
    #[should_panic(expected = "Matrix dimensions mismatch")]
    fn add_assign_panics_on_mismatch() {
        let mut a = Matrix::from_data(vec![vec![1, 2, 3]]);
        let b = Matrix::from_data(vec![vec![1, 2]]);

        a += b;
    }
}
