use std::collections::HashMap;
use std::hash::Hash;
pub struct Mean;

impl Mean {
    pub fn mean_f(data: &[f64]) -> f64 {
        let mut sum = 0.0;
        for x in data {
            sum += x;
        }
        sum / (data.len() as f64)
    }

    pub fn mean_i(data: &[i64]) -> f64 {
        let mut sum = 0;
        for x in data {
            sum += x;
        }
        (sum / (data.len() as i64)) as f64
    }

    pub fn weighted_average_f(data: &[f64], weights: &[f64]) -> f64 {
        let mut sum = 0.0;
        let mut sum_weights = 0.0;
        if data.len() != weights.len() {
            panic!("data and weights must have the same length");
        }
        for (x, w) in data.iter().zip(weights) {
            sum += x * w;
            sum_weights += w;
        }
        sum / sum_weights
    }

    pub fn weighted_average_i(data: &[i64], weights: &[i64]) -> f64 {
        let mut sum = 0;
        let mut sum_weights = 0;
        if data.len() != weights.len() {
            panic!("data and weights must have the same length");
        }
        for (x, w) in data.iter().zip(weights) {
            sum += x * w;
            sum_weights += w;
        }
        (sum / sum_weights) as f64
    }

    pub fn median_f(data: &[f64]) -> f64 {
        let mut data = data.to_vec();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = data.len() / 2;
        if data.len() % 2 == 0 {
            (data[mid] + data[mid - 1]) / 2.0
        } else {
            data[mid]
        }
    }

    pub fn median_i(data: &[i64]) -> f64 {
        let mut data = data.to_vec();
        data.sort();
        let mid = data.len() / 2;
        if data.len() % 2 == 0 {
            ((data[mid] + data[mid - 1]) / 2) as f64
        } else {
            data[mid] as f64
        }
    }

    pub fn mode<T: Eq + Ord + Clone + Hash>(data: &[T]) -> Option<T> {
        let mut counts = HashMap::new();

        for value in data {
            *counts.entry(value).or_insert(0) += 1;
        }

        let (mode, _) = counts.into_iter().max_by_key(|&(_, count)| count)?;

        Some(mode.clone())
    }

    pub fn mode_f64(data: &[f64]) -> Option<f64> {
        let mut counts = HashMap::new();

        for &value in data {
            *counts.entry(value.to_bits()).or_insert(0) += 1;
        }

        let (mode_bits, _) = counts.into_iter().max_by_key(|&(_, count)| count)?;

        Some(f64::from_bits(mode_bits))
    }

    pub fn variance_f(data: &[f64]) -> f64 {
        let mean = Mean::mean_f(data);
        let mut sum = 0.0;
        for x in data {
            sum += (x - mean).powi(2);
        }
        sum / (data.len() as f64)
    }

    pub fn variance_i(data: &[i64]) -> f64 {
        let mean = Mean::mean_i(data);
        let mut sum = 0;
        for x in data {
            sum += (x - mean as i64).pow(2);
        }
        (sum / (data.len() as i64)) as f64
    }

    pub fn standard_deviation_f(data: &[f64]) -> f64 {
        Mean::variance_f(data).sqrt()
    }

    pub fn standard_deviation_i(data: &[i64]) -> f64 {
        Mean::variance_i(data).sqrt()
    }
}

pub struct CalculusError;

impl CalculusError {
    pub fn absolute_error_f(x: f64, y: f64) -> f64 {
        (x - y).abs()
    }
    pub fn absolute_error_i(x: i64, y: i64) -> i64 {
        (x - y).abs()
    }
    pub fn relative_error_f(x: f64, y: f64) -> f64 {
        (x - y).abs() / x.abs()
    }
    pub fn relative_error_i(x: i64, y: i64) -> f64 {
        ((x - y).abs() / x.abs()) as f64
    }
}

pub struct LinearRegression {
    coefficients: Vec<f64>,
}

impl LinearRegression {
    pub fn new() -> LinearRegression {
        LinearRegression {
            coefficients: Vec::new(),
        }
    }

    pub fn fit(&mut self, x: &Vec<Vec<f64>>, y: &Vec<f64>) -> Result<(), &'static str> {
        let n = x.len();
        if n == 0 || x[0].len() == 0 || n != y.len() {
            return Err("Invalid input dimensions");
        }

        // Augment the input matrix with a column of ones for the intercept term
        let augmented_x: Vec<Vec<f64>> = x
            .iter()
            .map(|row| {
                let mut augmented_row = row.clone();
                augmented_row.insert(0, 1.0);
                augmented_row
            })
            .collect();

        // Calculate the coefficients using the normal equation: (X^T * X)^-1 * X^T * y
        let x_transpose = transpose(&augmented_x);
        let x_transpose_x = matrix_multiply(&x_transpose, &augmented_x)?;
        let x_transpose_x_inverse = inverse(&x_transpose_x).map_err(|_| "Singular matrix")?;
        let x_transpose_y = matrix_multiply(&x_transpose, &vec![y.clone()])?;
        self.coefficients = matrix_multiply(&x_transpose_x_inverse, &x_transpose_y)?
            .into_iter()
            .flat_map(|row| row)
            .collect();

        Ok(())
    }

    pub fn predict(&self, x: &Vec<Vec<f64>>) -> Result<Vec<f64>, &'static str> {
        if self.coefficients.is_empty() {
            return Err("Model not fitted");
        }

        let augmented_x: Vec<Vec<f64>> = x
            .iter()
            .map(|row| {
                let mut augmented_row = row.clone();
                augmented_row.insert(0, 1.0);
                augmented_row
            })
            .collect();

        let predictions = matrix_multiply(&augmented_x, &vec![self.coefficients.clone()])?;
        Ok(predictions.into_iter().flat_map(|row| row).collect())
    }
}

// Helper functions for matrix operations
fn transpose(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    (0..cols)
        .map(|j| (0..rows).map(|i| matrix[i][j]).collect())
        .collect()
}

fn matrix_multiply(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, &'static str> {
    let rows_a = a.len();
    let cols_a = a[0].len();
    let rows_b = b.len();
    let cols_b = b[0].len();

    if cols_a != rows_b {
        return Err("Incompatible matrix dimensions for multiplication");
    }

    Ok((0..rows_a)
        .map(|i| {
            (0..cols_b)
                .map(|j| (0..cols_a).fold(0.0, |acc, k| acc + a[i][k] * b[k][j]))
                .collect()
        })
        .collect())
}

fn inverse(matrix: &Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, &'static str> {
    let size = matrix.len();

    // Create an identity matrix for augmentation
    let mut augmented_matrix: Vec<Vec<f64>> = matrix
        .iter()
        .cloned()
        .zip((0..size).map(|i| (0..size).map(|j| if i == j { 1.0 } else { 0.0 }).collect()))
        .map(|(mut row, mut identity)| {
            row.append(&mut identity);
            row
        })
        .collect();

    for i in 0..size {
        // Find pivot row
        let pivot_row = (i..size)
            .max_by(|&x, &y| {
                augmented_matrix[x][i]
                    .partial_cmp(&augmented_matrix[y][i])
                    .unwrap()
            })
            .unwrap();

        // Swap rows
        augmented_matrix.swap(i, pivot_row);

        // Scale pivot row
        let pivot_element = augmented_matrix[i][i];
        if pivot_element == 0.0 {
            return Err("Matrix is singular"); // Matrix is singular
        }
        for j in 0..2 * size {
            augmented_matrix[i][j] /= pivot_element;
        }

        // Eliminate other rows
        for k in 0..size {
            if k != i {
                let factor = augmented_matrix[k][i];
                for j in 0..2 * size {
                    augmented_matrix[k][j] -= factor * augmented_matrix[i][j];
                }
            }
        }
    }

    // Extract the right half of the augmented matrix as the inverse
    let inverse_matrix: Vec<Vec<f64>> = augmented_matrix
        .into_iter()
        .map(|mut row| row.split_off(size))
        .collect();
    Ok(inverse_matrix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_f_test() {
        let data = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(Mean::mean_f(&data), 2.5);
    }

    #[test]
    fn mean_i_test() {
        let data = [1, 2, 3, 4, 5];
        assert_eq!(Mean::mean_i(&data), 3.0);
    }

    #[test]
    fn weighted_average_f_test() {
        let data = [1.0, 2.0, 3.0, 4.0];
        let weights = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(Mean::weighted_average_f(&data, &weights), 3.0);
    }

    #[test]
    fn weighted_average_i_test() {
        let data = [1, 2, 3, 4];
        let weights = [1, 2, 3, 4];
        assert_eq!(Mean::weighted_average_i(&data, &weights), 3.0);
    }

    #[test]
    fn absolute_error_f_test() {
        assert_eq!(CalculusError::absolute_error_f(1.0, 2.0), 1.0);
    }

    #[test]
    fn absolute_error_i_test() {
        assert_eq!(CalculusError::absolute_error_i(1, 2), 1);
    }

    #[test]
    fn relative_error_f_test() {
        assert_eq!(CalculusError::relative_error_f(1.0, 2.0), 1.0);
    }

    #[test]
    fn relative_error_i_test() {
        assert_eq!(CalculusError::relative_error_i(1, 2), 1.0);
    }

    #[test]
    fn median_f_test() {
        let data = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(Mean::median_f(&data), 2.5);
    }

    #[test]
    fn median_i_test() {
        let data = [1, 2, 3, 4, 5];
        assert_eq!(Mean::median_i(&data), 3.0);
    }

    #[test]
    fn mode_f_test() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 4.0];
        assert_eq!(Mean::mode_f64(&data), Some(4.0));
    }

    #[test]
    fn mode_i_test() {
        let data = vec![1, 2, 3, 4, 4];
        assert_eq!(Mean::mode(&data), Some(4));
    }

    #[test]
    fn variance_f_test() {
        let data = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(Mean::variance_f(&data), 1.25);
        assert_eq!(Mean::standard_deviation_f(&data), 1.118033988749895);
    }

    #[test]
    fn variance_i_test() {
        let data = [1, 2, 3, 4, 5];
        assert_eq!(Mean::variance_i(&data), 2.0);
        assert_eq!(Mean::standard_deviation_i(&data), 1.4142135623730951);
    }
    #[test]
    fn test_linear_regression() {
        // Training data
        let x_train = vec![
            vec![1.0, 2.0],
            vec![2.0, 3.0],
            vec![3.0, 4.0],
            vec![4.0, 5.0],
        ];
        let y_train = vec![3.0, 4.0, 5.0, 6.0];

        // Test data
        let x_test = vec![
            vec![5.0, 6.0],
            vec![6.0, 7.0],
            vec![7.0, 8.0],
            vec![8.0, 9.0],
        ];

        // Create and fit the model
        let mut model = LinearRegression::new();
        model.fit(&x_train, &y_train).unwrap();

        // Make predictions on the test data
        let predictions = model.predict(&x_test).unwrap();

        // Define a tolerance for floating-point comparisons
        let tolerance = 1e-6;

        // Expected values calculated manually
        let expected_values = vec![11.0, 13.0, 15.0, 17.0];

        // Check the predictions against expected values
        assert_eq!(predictions.len(), x_test.len());

        for (predicted, expected) in predictions.iter().zip(expected_values.iter()) {
            assert!((predicted - expected).abs() < tolerance);
        }
    }
}
