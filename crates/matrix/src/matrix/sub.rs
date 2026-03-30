use super::Matrix;
use crate::MatrixError;
use std::ops::{Sub, SubAssign};

impl<T: Copy> Matrix<T> {
    /// Вычитает одну матрицу из другой, возвращая `Err` при несовпадении размерностей.
    ///
    /// В отличие от оператора `-`, не паникует, а возвращает [`MatrixError::DimensionMismatch`].
    /// Исходные матрицы остаются неизменными — результат аллоцируется отдельно.
    pub fn try_sub(&self, rhs: &Self) -> Result<Self, MatrixError>
    where
        T: Sub<Output = T>,
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
                    .map(|(s, r)| *s - *r)
                    .collect()
            })
            .collect();
        Ok(Self::from_data(data))
    }
}

impl<T: SubAssign + Copy> SubAssign for Matrix<T> {
    /// Вычитает `rhs` из текущей матрицы на месте (`-=`).
    ///
    /// # Panics
    ///
    /// Паникует при несовпадении размерностей матриц.
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(self.rows, rhs.rows, "Matrix dimensions mismatch");
        assert_eq!(self.cols, rhs.cols, "Matrix dimensions mismatch");

        self.data
            .iter_mut()
            .zip(rhs.data.iter())
            .for_each(|(self_row, rhs_row)| {
                self_row
                    .iter_mut()
                    .zip(rhs_row.iter())
                    .for_each(|(s, r)| *s -= *r);
            });
    }
}

impl<T: SubAssign + Copy> Sub for Matrix<T> {
    type Output = Self;

    /// Вычитает одну матрицу из другой (`-`). Переиспользует память `self`, избегая лишних аллокаций.
    ///
    /// # Panics
    ///
    /// Паникует при несовпадении размерностей матриц.
    /// Для безопасного варианта используйте [`Matrix::try_sub`].
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- try_sub ---

    /// Поэлементное вычитание двух матриц одинакового размера
    #[test]
    fn try_sub_same_dimensions() {
        let a = Matrix::from_data(vec![vec![10, 20, 30], vec![40, 50, 60]]);
        let b = Matrix::from_data(vec![vec![1, 2, 3], vec![4, 5, 6]]);

        let result = a.try_sub(&b).expect("dimensions match");

        assert_eq!(result.rows(), 2);
        assert_eq!(result.cols(), 3);
        assert_eq!(
            result,
            Matrix::from_data(vec![vec![9, 18, 27], vec![36, 45, 54]])
        );
    }

    /// Несовпадение размерностей возвращает ошибку, а не панику
    #[test]
    fn try_sub_dimension_mismatch() {
        let a = Matrix::from_data(vec![vec![1, 2]]);
        let b = Matrix::from_data(vec![vec![1, 2, 3]]);

        assert!(a.try_sub(&b).is_err());
    }

    /// Исходные матрицы не изменяются после `try_sub`
    #[test]
    fn try_sub_does_not_consume_operands() {
        let a = Matrix::from_data(vec![vec![5, 10]]);
        let b = Matrix::from_data(vec![vec![1, 2]]);

        let _ = a.try_sub(&b);

        assert_eq!(a.rows(), 1);
        assert_eq!(b.rows(), 1);
    }

    /// Вычитание матрицы из самой себя даёт нулевую матрицу
    #[test]
    fn try_sub_self_gives_zero() {
        let a = Matrix::from_data(vec![vec![5, 10], vec![15, 20]]);
        let zero = Matrix::new(2, 2);

        let result = a.try_sub(&a).expect("dimensions match");

        assert_eq!(result, zero);
    }

    // --- Sub (оператор -) ---

    /// Оператор - вычитает матрицы поэлементно
    #[test]
    fn sub_operator() {
        let a = Matrix::from_data(vec![vec![10, 20], vec![30, 40]]);
        let b = Matrix::from_data(vec![vec![1, 2], vec![3, 4]]);

        let result = a - b;

        assert_eq!(result, Matrix::from_data(vec![vec![9, 18], vec![27, 36]]));
    }

    /// Оператор - паникует при несовпадении размерностей
    #[test]
    #[should_panic(expected = "Matrix dimensions mismatch")]
    fn sub_operator_panics_on_mismatch() {
        let a = Matrix::from_data(vec![vec![1, 2]]);
        let b = Matrix::from_data(vec![vec![1], vec![2]]);

        let _ = a - b;
    }

    // --- SubAssign (оператор -=) ---

    /// Оператор -= модифицирует матрицу на месте
    #[test]
    fn sub_assign_operator() {
        let mut a = Matrix::from_data(vec![vec![10, 20], vec![30, 40]]);
        let b = Matrix::from_data(vec![vec![1, 2], vec![3, 4]]);

        a -= b;

        assert_eq!(a, Matrix::from_data(vec![vec![9, 18], vec![27, 36]]));
    }

    /// Оператор -= паникует при несовпадении размерностей
    #[test]
    #[should_panic(expected = "Matrix dimensions mismatch")]
    fn sub_assign_panics_on_mismatch() {
        let mut a = Matrix::from_data(vec![vec![1, 2, 3]]);
        let b = Matrix::from_data(vec![vec![1, 2]]);

        a -= b;
    }
}
