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
