#[derive(Debug, PartialEq)]
pub enum MatrixError {
    InconsistentColumnSizes,
    MultiplicationDimensionMismatch,
    DimensionMismatch,
}

#[derive(Debug, PartialEq)]
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
        let data: Vec<Vec<i64>> = Vec::with_capacity(self.rows as usize);
        //TODO: matrix multiplication logic
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
}
