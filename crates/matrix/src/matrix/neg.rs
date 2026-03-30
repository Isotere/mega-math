use super::Matrix;
use std::ops::Neg;

impl<T: Neg<Output = T> + Copy> Neg for Matrix<T> {
    type Output = Self;

    /// Возвращает матрицу с противоположными знаками всех элементов (`-A`).
    /// Переиспользует память `self`, избегая лишних аллокаций.
    fn neg(mut self) -> Self::Output {
        for row in &mut self.data {
            for elem in row {
                *elem = -*elem;
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Унарный минус меняет знак всех элементов
    #[test]
    fn neg_matrix() {
        let a = Matrix::from_data(vec![vec![1, -2, 3], vec![-4, 5, -6]]);

        let result = -a;

        assert_eq!(
            result,
            Matrix::from_data(vec![vec![-1, 2, -3], vec![4, -5, 6]])
        );
    }

    /// Двойное отрицание возвращает исходную матрицу
    #[test]
    fn neg_double_is_identity() {
        let a = Matrix::from_data(vec![vec![1, 2], vec![3, 4]]);
        let original = a.clone();

        let result = -(-a);

        assert_eq!(result, original);
    }

    /// Отрицание нулевой матрицы даёт нулевую матрицу
    #[test]
    fn neg_zero_matrix() {
        let zero: Matrix<i32> = Matrix::new(2, 3);

        let result = -zero;

        assert_eq!(result, Matrix::new(2, 3));
    }

    /// Отрицание работает с f64
    #[test]
    fn neg_f64() {
        let a = Matrix::from_data(vec![vec![1.5, -2.5], vec![0.0, 3.0]]);

        let result = -a;

        assert_eq!(
            result,
            Matrix::from_data(vec![vec![-1.5, 2.5], vec![-0.0, -3.0]])
        );
    }
}
