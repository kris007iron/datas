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
}
