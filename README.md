

# Statistical, Vector, and Matrix Operations Library - Datas ![Build Status]
[Build Status]: https://img.shields.io/github/actions/workflow/status/kris007iron/datas/rust.yml?branch=master

This Rust library provides statistical, vector algebra, and matrix operations. It includes methods for calculating statistical measures (mean, median, variance, etc.), operations on vectors (addition, dot product, scalar multiplication), and matrix manipulation (addition, multiplication, row swapping, etc.).

## Modules

### 1. **Statistical Functions**

The `Mean` struct provides statistical functions for both `f64` and `i64` types and its functions are self-explanatory:
- **Mean**:
  - `mean_f`
  - `mean_i`
  
- **Weighted Average**:
  - `weighted_average_f`
  - `weighted_average_i`
  
- **Median**:
  - `median_f`
  - `median_i`
  
- **Mode**(here i tried some generics usage but i still need some clarification):
  - `mode`: Calculates the mode for any `Hash`, `Ord`, and `Clone` types.
  - `mode_f64`: Specialized mode calculation for `f64` numbers.

- **Variance and Standard Deviation**:
  - `variance_f`
  - `variance_i`
  - `standard_deviation_f`
  - `standard_deviation_i`

### 2. **CalculusError for Error Calculation**

The `CalculusError` struct provides error calculation methods for both `f64` and `i64` values:
- **Absolute Error**:
  - `absolute_error_f`
  - `absolute_error_i`
  
- **Relative Error**:
  - `relative_error_f`
  - `relative_error_i`

### 3. **Vector Operations**

The `Vector` struct supports basic vector algebra for both `i64` and `f64` types:
- **Creation**:
  - `new`: Initializes a new vector with `i64` or `f64` components.
  
- **Magnitude**:
  - `magnitude`: Computes the magnitude (Euclidean length) of the vector.

- **Addition**:
  - `add`: Adds two `i64` vectors (mutates the first vector).
  - `add_f`: Adds two `f64` vectors (mutates the first vector).
  - `add_i`: Adds an `i64` vector to an `f64` vector (mutates the `f64` vector).

- **Dot Product**:
  - `dot_product`: Computes the dot product of two `i64` vectors.
  - `dot_product_f`: Computes the dot product of two `f64` vectors.
  - `dot_product_i`: Computes the dot product between an `f64` and `i64` vector.

- **Scalar Multiplication**:
  - Overloaded multiplication (`Mul` trait) for both `i64` and `f64` scalar multiplication.

### 4. **Matrix Operations**

The `Matrix` struct provides support for matrix operations with `i64` data types:
- **Creation**:
  - `new`: Initializes a matrix from a `Vec<Vec<i64>>`. Returns an error if the rows have inconsistent column sizes.

- **Addition**:
  - `add`: Adds two matrices element-wise. Returns an error if the matrices have mismatched dimensions.

- **Row Swapping**:
  - `swap_row`: Swaps two rows of the matrix. Returns an error if the row indices are out of bounds.

- **Scalar Multiplication**:
  - `scalar_multiplication`: Multiplies each element of the matrix by a scalar.

- **Matrix Multiplication**:
  - `matrix_multiplication`: Multiplies two matrices. Returns an error if the number of columns in the first matrix does not match the number of rows in the second matrix.

### 5. **MatrixError for Error Handling**

The `MatrixError` enum handles errors specific to matrix operations:
- `InconsistentColumnSizes`: Raised when matrix rows have different column sizes.
- `MultiplicationDimensionMismatch`: Raised when matrix dimensions are incompatible for multiplication.
- `DimensionMismatch`: Raised when matrices have different dimensions during addition.
- `RowOutOfBound`: Raised when trying to swap rows that don't exist.

### 6. **VectorError for Error Handling**

The `VectorError` enum handles errors during vector operations:
- `DimensionMismatch`: Raised when vectors of different dimensions are used in an operation.

## Tests

The library includes comprehensive unit tests, covering:
- Statistical functions: mean, weighted average, median, mode, variance, and standard deviation for `f64` and `i64`.
- Vector operations: addition, scalar multiplication, dot product, and magnitude calculation.
- Matrix operations: addition, scalar multiplication, matrix multiplication, row swapping, and error handling for dimension mismatches.

## How to Use

1. **Add to your Cargo.toml**:
```toml
[dependencies]
datas = "0.1.8"
```

2. **Example Usage**:
```rust
use datas::Mean;
use datas::vector::Vector;
use datas::matrix::Matrix;

// Statistical calculations
let data = vec![1.0, 2.0, 3.0, 4.0];
let mean = Mean::mean_f(&data);
println!("Mean: {}", mean);

// Vector operations
let vec1 = Vector::<i64>::new(vec![1, 2, 3]);
let vec2 = Vector::<i64>::new(vec![4, 5, 6]);
let dot_product = vec1.dot_product(&vec2).unwrap();
println!("Dot product: {}", dot_product);

// Matrix operations
let matrix1 = Matrix::<i64>::new(vec![vec![1, 2], vec![3, 4]]).unwrap();
let matrix2 = Matrix::<i64>::new(vec![vec![2, 0], vec![1, 2]]).unwrap();
let product = matrix1.matrix_multiplication(&matrix2).unwrap();
println!("Matrix Product: {:?}", product);
```

This library offers efficient and flexible handling of common statistical, vector-based, and matrix-based operations, making it a powerful tool for mathematical computation in Rust projects. Contributions are welcome, as I aim to expand functionality and improve generic handling.
