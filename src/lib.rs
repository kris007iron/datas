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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean_f(&data), 3.0);
    }
}
