use std::collections::HashMap;
use std::hash::Hash;

pub mod vector;
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
}
