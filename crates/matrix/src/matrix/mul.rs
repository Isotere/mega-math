use super::Matrix;
use std::ops::{AddAssign, Mul, MulAssign};

impl<T> Mul for Matrix<T>
where
    T: Mul<Output = T> + AddAssign + Default + Copy,
{
    type Output = Self;

    /// Матричное умножение (`A * B`). Количество столбцов `self` должно совпадать с количеством строк `rhs`.
    ///
    /// # Panics
    ///
    /// Паникует при несовпадении размерностей (`self.cols != rhs.rows`).
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols, rhs.rows, "Matrix dimensions mismatch");

        let (m, n, k) = (self.rows, self.cols, rhs.cols);

        let mut result = Self::new(m, k);

        for i in 0..m {
            for j in 0..k {
                for s in 0..n {
                    result.data[i][j] += self.data[i][s] * rhs.data[s][j];
                }
            }
        }

        result
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Matrix<T> {
    /// Умножает каждый элемент матрицы на скаляр на месте (`*=`).
    fn mul_assign(&mut self, rhs: T) {
        for row in &mut self.data {
            for elem in row {
                *elem *= rhs;
            }
        }
    }
}

impl<T: MulAssign + Copy> Mul<T> for Matrix<T> {
    type Output = Self;

    /// Умножает каждый элемент матрицы на скаляр (`*`). Переиспользует память `self`, избегая лишних аллокаций.
    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Mul (оператор *) ---

    /// Умножение матрицы на скаляр
    #[test]
    fn mul_scalar() {
        let a = Matrix::from_data(vec![vec![1, 2, 3], vec![4, 5, 6]]);

        let result = a * 10;

        assert_eq!(
            result,
            Matrix::from_data(vec![vec![10, 20, 30], vec![40, 50, 60]])
        );
    }

    /// Умножение на 1 не меняет матрицу
    #[test]
    fn mul_by_one() {
        let a = Matrix::from_data(vec![vec![5, 10], vec![15, 20]]);

        let result = a.clone() * 1;

        assert_eq!(result, a);
    }

    /// Умножение на 0 даёт нулевую матрицу
    #[test]
    #[allow(clippy::erasing_op, reason = "тест проверяет именно умножение на 0")]
    fn mul_by_zero() {
        let a = Matrix::from_data(vec![vec![5, 10], vec![15, 20]]);

        let result = a * 0;

        assert_eq!(result, Matrix::new(2, 2));
    }

    /// Умножение с отрицательным скаляром
    #[test]
    fn mul_negative_scalar() {
        let a = Matrix::from_data(vec![vec![1, -2], vec![3, -4]]);

        let result = a * -2;

        assert_eq!(result, Matrix::from_data(vec![vec![-2, 4], vec![-6, 8]]));
    }

    /// Умножение с f64
    #[test]
    fn mul_f64_scalar() {
        let a = Matrix::from_data(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

        let result = a * 0.5;

        assert_eq!(
            result,
            Matrix::from_data(vec![vec![0.5, 1.0], vec![1.5, 2.0]])
        );
    }

    // --- MulAssign (оператор *=) ---

    /// Оператор *= модифицирует матрицу на месте
    #[test]
    fn mul_assign_scalar() {
        let mut a = Matrix::from_data(vec![vec![1, 2], vec![3, 4]]);

        a *= 3;

        assert_eq!(a, Matrix::from_data(vec![vec![3, 6], vec![9, 12]]));
    }

    /// Последовательное *= применяется корректно
    #[test]
    fn mul_assign_twice() {
        let mut a = Matrix::from_data(vec![vec![1, 2], vec![3, 4]]);

        a *= 2;
        a *= 3;

        assert_eq!(a, Matrix::from_data(vec![vec![6, 12], vec![18, 24]]));
    }

    // --- Mul матричное (оператор * Matrix) ---

    /// Умножение квадратных матриц 2×2
    #[test]
    fn mul_matrix_square() {
        let a = Matrix::from_data(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::from_data(vec![vec![5, 6], vec![7, 8]]);

        let result = a * b;

        assert_eq!(
            result,
            Matrix::from_data(vec![vec![19, 22], vec![43, 50]])
        );
    }

    /// Умножение неквадратных матриц (2×3 * 3×2 → 2×2)
    #[test]
    fn mul_matrix_rectangular() {
        let a = Matrix::from_data(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let b = Matrix::from_data(vec![
            vec![7, 8],
            vec![9, 10],
            vec![11, 12],
        ]);

        let result = a * b;

        assert_eq!(result.rows(), 2);
        assert_eq!(result.cols(), 2);
        assert_eq!(
            result,
            Matrix::from_data(vec![vec![58, 64], vec![139, 154]])
        );
    }

    /// Умножение на единичную матрицу не меняет результат
    #[test]
    fn mul_matrix_identity() {
        let a = Matrix::from_data(vec![vec![1, 2], vec![3, 4]]);
        let identity = Matrix::from_data(vec![vec![1, 0], vec![0, 1]]);

        let result = a.clone() * identity;

        assert_eq!(result, a);
    }

    /// Умножение матрицы-строки на матрицу-столбец даёт 1×1
    #[test]
    fn mul_matrix_row_by_column() {
        let row = Matrix::from_data(vec![vec![1, 2, 3]]);
        let col = Matrix::from_data(vec![vec![4], vec![5], vec![6]]);

        let result = row * col;

        assert_eq!(result.rows(), 1);
        assert_eq!(result.cols(), 1);
        assert_eq!(result, Matrix::from_data(vec![vec![32]]));
    }

    /// Паника при несовместимых размерностях для матричного умножения
    #[test]
    #[should_panic(expected = "Matrix dimensions mismatch")]
    fn mul_matrix_panics_on_mismatch() {
        let a = Matrix::from_data(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::from_data(vec![vec![1, 2], vec![3, 4], vec![5, 6]]);

        let _ = a * b;
    }
}
