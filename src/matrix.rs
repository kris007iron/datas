#[derive(Debug, PartialEq)]
pub enum MatrixError {
    InconsistentColumnSizes,
    MultiplicationDimensionMismatch,
    DimensionMismatch,
    RowOutOfBound,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
    rows: u64,
    cols: u64,
}

impl Matrix<i64> {
    pub fn new(data: Vec<Vec<i64>>) -> Result<Self, MatrixError> {
        let rows = data.len() as u64;
        if rows == 0 {
            return Ok(Self {
                data,
                rows: 0,
                cols: 0,
            });
        }
        let cols = data[0].len() as u64;
        if !data.iter().all(|row| row.len() as u64 == cols) {
            return Err(MatrixError::InconsistentColumnSizes);
        }

        Ok(Self { data, rows, cols })
    }

    pub fn swap_row(&mut self, f_row: u64, s_row: u64) -> Result<(), MatrixError> {
        if f_row >= self.rows || s_row >= self.rows {
            return Err(MatrixError::RowOutOfBound);
        }
        let temp: Vec<i64> = self.data[f_row as usize].clone();
        self.data[f_row as usize] = self.data[s_row as usize].clone();
        self.data[s_row as usize] = temp;
        Ok(())
    }

    pub fn add(&mut self, matrix: &Matrix<i64>) -> Result<(), MatrixError> {
        if self.cols != matrix.cols || self.rows != matrix.rows {
            return Err(MatrixError::DimensionMismatch);
        }

        self.data
            .iter_mut()
            .zip(&matrix.data)
            .for_each(|(self_row, matrix_row)| {
                self_row
                    .iter_mut()
                    .zip(matrix_row)
                    .for_each(|(a, b)| *a += b)
            });
        Ok(())
    }
    pub fn scalar_multiplication(&mut self, scalar: i64) {
        self.data
            .iter_mut()
            .for_each(|col| col.iter_mut().for_each(|number| *number *= scalar));
    }

    pub fn matrix_multiplication(self, matrix: &Matrix<i64>) -> Result<Matrix<i64>, MatrixError> {
        if self.cols != matrix.rows {
            return Err(MatrixError::MultiplicationDimensionMismatch);
        }

        let mut data = Vec::with_capacity(self.rows as usize);

        for row in &self.data {
            let mut new_row = Vec::with_capacity(matrix.cols as usize);

            for col_idx in 0..matrix.cols {
                let mut sum = 0;
                for (i, val) in row.iter().enumerate() {
                    sum += val * matrix.data[i][col_idx as usize];
                }
                new_row.push(sum);
            }

            data.push(new_row);
        }
        return Ok(match Matrix::<i64>::new(data) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn imatrix_addition() {
        let mut matrix = match Matrix::<i64>::new(vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]])
        {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        let matrix2 = match Matrix::<i64>::new(vec![vec![1, 2, 3], vec![2, 3, 1], vec![3, 1, 2]]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };
        let res = matrix.add(&matrix2).err();
        println!("{:?}", matrix);
        assert_eq!(res, None);
    }

    #[test]
    fn imatrix_cloning() {
        let matrix = match Matrix::<i64>::new(vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        let matrix2 = matrix.clone();

        assert_eq!(matrix, matrix2)
    }

    #[test]
    fn imatrix_swap() {
        let mut matrix = match Matrix::<i64>::new(vec![vec![1, 2, 3], vec![3, 2, 1], vec![1, 2, 3]])
        {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };
        let matrix2 = match Matrix::<i64>::new(vec![vec![1, 2, 3], vec![1, 2, 3], vec![3, 2, 1]]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        // 0 based indexing
        match matrix.swap_row(1, 2) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };
        println!("{:?}", matrix);
        assert_eq!(matrix, matrix2)
    }

    #[test]
    fn imatrix_multiplication() {
        let matrix = match Matrix::<i64>::new(vec![vec![1, 2], vec![3, 4]]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        let matrix2 = match Matrix::<i64>::new(vec![vec![2, 0], vec![1, 2]]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        let matrix3 = match Matrix::<i64>::new(vec![vec![4, 4], vec![10, 8]]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(matrix.matrix_multiplication(&matrix2).unwrap(), matrix3)
    }
    // Test case for identity matrix
    #[test]
    fn imatrix_multiplication_identity() {
        let matrix = match Matrix::<i64>::new(vec![vec![1, 2], vec![3, 4]]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        let identity_matrix = match Matrix::<i64>::new(vec![vec![1, 0], vec![0, 1]]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        let expected_result = matrix.clone(); // Multiplying by identity matrix should return the same matrix

        assert_eq!(
            matrix.matrix_multiplication(&identity_matrix).unwrap(),
            expected_result
        );
    }

    // Test case for multiplication by zero matrix
    #[test]
    fn imatrix_multiplication_zero_matrix() {
        let matrix = match Matrix::<i64>::new(vec![vec![1, 2], vec![3, 4]]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        let zero_matrix = match Matrix::<i64>::new(vec![vec![0, 0], vec![0, 0]]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        let expected_result = match Matrix::<i64>::new(vec![vec![0, 0], vec![0, 0]]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(
            matrix.matrix_multiplication(&zero_matrix).unwrap(),
            expected_result
        );
    }

    // Test case for non-square matrices
    #[test]
    fn imatrix_multiplication_non_square() {
        let matrix = match Matrix::<i64>::new(vec![vec![1, 2, 3], vec![4, 5, 6]]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        let matrix2 = match Matrix::<i64>::new(vec![vec![7, 8], vec![9, 10], vec![11, 12]]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        let expected_result = match Matrix::<i64>::new(vec![vec![58, 64], vec![139, 154]]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(
            matrix.matrix_multiplication(&matrix2).unwrap(),
            expected_result
        );
    }

    #[test]
    fn imatrix_multiplication_large_matrix() {
        let matrix = match Matrix::<i64>::new(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        let matrix2 = match Matrix::<i64>::new(vec![
            vec![16, 15, 14, 13],
            vec![12, 11, 10, 9],
            vec![8, 7, 6, 5],
            vec![4, 3, 2, 1],
        ]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        let expected_result = match Matrix::<i64>::new(vec![
            vec![80, 70, 60, 50],
            vec![240, 214, 188, 162],
            vec![400, 358, 316, 274],
            vec![560, 502, 444, 386],
        ]) {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(
            matrix.matrix_multiplication(&matrix2).unwrap(),
            expected_result
        );
    }
}
