# Statistical and Vector Operations Library - Datas  ![Build Status]
[Build Status]: https://img.shields.io/github/actions/workflow/status/kris007iron/datas/rust.yml?branch=master

This Rust library provides statistical operations and vector algebra functionalities, working with both `f64` and `i64` data types. It also includes methods for calculating common statistical measures (mean, median, mode, variance, standard deviation) and operations on vectors (addition, dot product, magnitude).

## Modules

### 1. **Mean and Statistical Functions**

The `Mean` struct includes statistical functions for both `f64` and `i64` types:
- **Mean**:
  - `mean_f`: Calculates the mean of a list of `f64` numbers.
  - `mean_i`: Calculates the mean of a list of `i64` numbers.
  
- **Weighted Average**:
  - `weighted_average_f`: Calculates the weighted average for `f64` numbers.
  - `weighted_average_i`: Calculates the weighted average for `i64` numbers.
  
- **Median**:
  - `median_f`: Finds the median for `f64` numbers.
  - `median_i`: Finds the median for `i64` numbers.
  
- **Mode**:
  - `mode`: Calculates the mode for any `Hash`, `Ord`, and `Clone` types.
  - `mode_f64`: Specialized mode calculation for `f64` numbers.

- **Variance and Standard Deviation**:
  - `variance_f`: Computes variance for `f64` numbers.
  - `variance_i`: Computes variance for `i64` numbers.
  - `standard_deviation_f`: Computes standard deviation for `f64` numbers.
  - `standard_deviation_i`: Computes standard deviation for `i64` numbers.

### 2. **CalculusError for Error Calculation**

The `CalculusError` struct provides error calculation methods for both `f64` and `i64` values:
- **Absolute Error**:
  - `absolute_error_f`: Calculates the absolute error for `f64`.
  - `absolute_error_i`: Calculates the absolute error for `i64`.
  
- **Relative Error**:
  - `relative_error_f`: Calculates the relative error for `f64`.
  - `relative_error_i`: Calculates the relative error for `i64`.

### 3. **Vector Operations**

The `Vector` struct is implemented for both `i64` and `f64` types and supports basic vector algebra:
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

### 4. **VectorError for Error Handling**

The `VectorError` enum handles errors during vector operations:
- `DimensionMismatch`: Error raised when vectors of different dimensions are used in an operation.

## Tests

The library is fully tested with unit tests, covering:
- Calculation of mean, weighted average, median, mode, variance, and standard deviation for both `f64` and `i64`.
- Error handling for mismatched vector dimensions.
- Vector addition, scalar multiplication, and dot product operations.
- Magnitude computation for vectors.

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

// Statistical calculations
let data = vec![1.0, 2.0, 3.0, 4.0];
let mean = Mean::mean_f(&data);
println!("Mean: {}", mean);

// Vector operations
let vec1 = Vector::<i64>::new(vec![1, 2, 3]);
let vec2 = Vector::<i64>::new(vec![4, 5, 6]);
let dot_product = vec1.dot_product(&vec2).unwrap();
println!("Dot product: {}", dot_product);
```

This library offers efficient and flexible handling of common statistical and vector-based operations, making it a useful tool for mathematical computation in Rust projects. Still, it needs much work and improvements such as a more generic approach, so feel free to contribute.