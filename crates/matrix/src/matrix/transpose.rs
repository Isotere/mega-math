use super::Matrix;

impl<T: Default + Copy> Matrix<T> {
    /// Возвращает транспонированную матрицу.
    ///
    /// Для квадратных матриц выполняет swap на месте (0 аллокаций).
    /// Для неквадратных — аллоцирует новую матрицу `n×m`.
    pub fn transpose(mut self) -> Self {
        if self.is_square() {
            for i in 0..self.rows {
                for j in (i + 1)..self.cols {
                    let tmp = self.data[i][j];
                    self.data[i][j] = self.data[j][i];
                    self.data[j][i] = tmp;
                }
            }
            return self;
        }

        let mut result = Self::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Транспонирование квадратной матрицы
    #[test]
    fn transpose_square() {
        let a = Matrix::from_data(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);

        let result = a.transpose();

        assert_eq!(
            result,
            Matrix::from_data(vec![
                vec![1, 4, 7],
                vec![2, 5, 8],
                vec![3, 6, 9],
            ])
        );
    }

    /// Транспонирование неквадратной матрицы (2×3 → 3×2)
    #[test]
    fn transpose_rectangular() {
        let a = Matrix::from_data(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ]);

        let result = a.transpose();

        assert_eq!(result.rows(), 3);
        assert_eq!(result.cols(), 2);
        assert_eq!(
            result,
            Matrix::from_data(vec![
                vec![1, 4],
                vec![2, 5],
                vec![3, 6],
            ])
        );
    }

    /// Двойное транспонирование возвращает исходную матрицу
    #[test]
    fn transpose_double_is_identity() {
        let a = Matrix::from_data(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ]);
        let original = a.clone();

        let result = a.transpose().transpose();

        assert_eq!(result, original);
    }

    /// Транспонирование матрицы-строки даёт матрицу-столбец
    #[test]
    fn transpose_row_to_column() {
        let row = Matrix::from_data(vec![vec![1, 2, 3]]);

        let col = row.transpose();

        assert_eq!(col.rows(), 3);
        assert_eq!(col.cols(), 1);
        assert_eq!(
            col,
            Matrix::from_data(vec![vec![1], vec![2], vec![3]])
        );
    }

    /// Транспонирование матрицы 1×1
    #[test]
    fn transpose_single_element() {
        let a = Matrix::from_data(vec![vec![42]]);

        let result = a.clone().transpose();

        assert_eq!(result, a);
    }
}
