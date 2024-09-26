#[derive(Debug, PartialEq)]
pub enum MatrixError {
    InconsistentColumnSizes,
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
        let self_data_iterator = self.data.iter_mut();
        let matrix_data_iterator = matrix.data.iter();
        for (self_row, matrix_row) in self_data_iterator.zip(matrix_data_iterator) {
            for (self_el, matrix_el) in self_row.iter_mut().zip(matrix_row) {
                *self_el += matrix_el;
            }
        }
        Ok(())
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
