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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_f_test() {
        let data = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(mean_f(&data), 2.5);
    }

    #[test]
    fn mean_i_test() {
        let data = [1, 2, 3, 4, 5];
        assert_eq!(mean_i(&data), 3.0);
    }

    #[test]
    fn weighted_average_f_test() {
        let data = [1.0, 2.0, 3.0, 4.0];
        let weights = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(weighted_average_f(&data, &weights), 3.0);
    }

    #[test]
    fn weighted_average_i_test() {
        let data = [1, 2, 3, 4];
        let weights = [1, 2, 3, 4];
        assert_eq!(weighted_average_i(&data, &weights), 3.0);
    }
}
