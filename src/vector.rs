use std::ops::Mul;

#[derive(Debug, PartialEq)]
pub enum VectorError {
    DimensionMismatch,
}

#[derive(Debug, PartialEq)]
pub struct Vector<T> {
    components: Vec<T>,
    dimensions: u64,
}

impl Vector<i64> {
    pub fn new(components: Vec<i64>) -> Self {
        let dimensions = components.len() as u64;
        Self {
            components,
            dimensions,
        }
    }

    pub fn magnitude(&self) -> f64 {
        self.components
            .iter()
            .map(|&component| (component as f64).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    pub fn add(&mut self, vector: &Vector<i64>) -> Result<(), VectorError> {
        if self.dimensions == vector.dimensions {
            for (index, component) in self.components.iter_mut().enumerate() {
                *component += vector.components[index];
            }
            return Ok(());
        } else {
            return Err(VectorError::DimensionMismatch);
        }
    }

    pub fn dot_product(&self, vector: &Vector<i64>) -> Result<i64, VectorError> {
        if self.dimensions != vector.dimensions {
            return Err(VectorError::DimensionMismatch);
        }
        Ok(self
            .components
            .iter()
            .zip(&vector.components)
            .map(|(a, b)| a * b)
            .sum())
    }
}

impl Mul<i64> for Vector<i64> {
    type Output = Vector<i64>;

    fn mul(mut self, scalar: i64) -> Vector<i64> {
        self.components
            .iter_mut()
            .for_each(|component| *component *= scalar);
        self
    }
}

impl Vector<f64> {
    pub fn new(components: Vec<f64>) -> Self {
        let dimensions = components.len() as u64;
        Self {
            components,
            dimensions,
        }
    }

    pub fn magnitude(&self) -> f64 {
        self.components
            .iter()
            .map(|&component| component.powi(2))
            .sum::<f64>()
            .sqrt()
    }

    pub fn add_i(&mut self, vector: &Vector<i64>) -> Result<(), VectorError> {
        if self.dimensions == vector.dimensions {
            for (index, component) in self.components.iter_mut().enumerate() {
                *component += vector.components[index] as f64;
            }
            return Ok(());
        } else {
            return Err(VectorError::DimensionMismatch);
        }
    }

    pub fn add_f(&mut self, vector: &Vector<f64>) -> Result<(), VectorError> {
        if self.dimensions == vector.dimensions {
            for (index, component) in self.components.iter_mut().enumerate() {
                *component += vector.components[index];
            }
            return Ok(());
        } else {
            return Err(VectorError::DimensionMismatch);
        }
    }

    pub fn dot_product_i(&self, vector: &Vector<i64>) -> Result<f64, VectorError> {
        if self.dimensions != vector.dimensions {
            return Err(VectorError::DimensionMismatch);
        }
        Ok(self
            .components
            .iter()
            .zip(&vector.components)
            .map(|(a, b)| a * (*b as f64))
            .sum())
    }

    pub fn dot_product_f(&self, vector: &Vector<f64>) -> Result<f64, VectorError> {
        if self.dimensions != vector.dimensions {
            return Err(VectorError::DimensionMismatch);
        }
        Ok(self
            .components
            .iter()
            .zip(&vector.components)
            .map(|(a, b)| a * b)
            .sum())
    }
}

impl Mul<f64> for Vector<f64> {
    type Output = Vector<f64>;

    fn mul(mut self, scalar: f64) -> Vector<f64> {
        self.components
            .iter_mut()
            .for_each(|component| *component *= scalar);
        self
    }
}

impl Mul<i64> for Vector<f64> {
    type Output = Vector<f64>;

    fn mul(mut self, scalar: i64) -> Vector<f64> {
        self.components
            .iter_mut()
            .for_each(|component| *component *= scalar as f64);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_ivec_creation_test() {
        let i_vector: Vector<i64> = Vector::<i64>::new(vec![1, 2, 3]);
        assert_eq!(
            i_vector,
            Vector::<i64> {
                components: vec![1, 2, 3],
                dimensions: 3
            }
        )
    }

    #[test]
    fn new_fvec_creation_test() {
        let f_vector: Vector<f64> = Vector::<f64>::new(vec![1.0, 2.2, 3.2]);
        assert_eq!(
            f_vector,
            Vector::<f64> {
                components: vec![1.0, 2.2, 3.2],
                dimensions: 3
            }
        )
    }

    #[test]
    fn ivec_magnitude() {
        let i_vector = Vector::<i64>::new(vec![3, 4]);
        assert_eq!(i_vector.magnitude(), 5.0);
    }

    #[test]
    fn fvec_magnitude() {
        let f_vector: Vector<f64> = Vector::<f64>::new(vec![6.0, 8.0]);
        assert_eq!(f_vector.magnitude(), 10.0);
    }

    #[test]
    fn ivec_addition() {
        let mut i_vector = Vector::<i64>::new(vec![3, 4]);
        let i_vector_2 = Vector::<i64>::new(vec![3, 4]);
        let _ = i_vector.add(&i_vector_2).expect("mismatched dimensions");
        assert_eq!(
            i_vector,
            Vector::<i64> {
                components: vec![6, 8],
                dimensions: 2
            }
        )
    }

    #[test]
    fn fvec_and_fvec_addition() {
        let mut f_vector = Vector::<f64>::new(vec![3.5, 4.2]);
        let f_vector_2 = Vector::<f64>::new(vec![3.2, 4.1]);
        let _ = f_vector.add_f(&f_vector_2).expect("mismatched dimensions");
        assert_eq!(
            f_vector,
            Vector::<f64> {
                components: vec![6.7, 8.3],
                dimensions: 2
            }
        )
    }

    #[test]
    fn fvec_and_ivec_addition() {
        let mut f_vector = Vector::<f64>::new(vec![3.5, 4.2]);
        let i_vector = Vector::<i64>::new(vec![3, 4]);
        let _ = f_vector.add_i(&i_vector).expect("mismatched dimensions");
        assert_eq!(
            f_vector,
            Vector::<f64> {
                components: vec![6.5, 8.2],
                dimensions: 2
            }
        )
    }

    #[test]
    fn ivec_addition_failure() {
        let mut i_vector = Vector::<i64>::new(vec![3, 4]);
        let i_vector_2 = Vector::<i64>::new(vec![3, 4, 5]);
        let res = i_vector.add(&i_vector_2).err();
        assert_eq!(res, Some(VectorError::DimensionMismatch));
    }

    #[test]
    fn fvec_and_fvec_addition_failure() {
        let mut f_vector = Vector::<f64>::new(vec![3.3, 4.1]);
        let f_vector_2 = Vector::<f64>::new(vec![3.2, 4.34, 5.2]);
        let res = f_vector.add_f(&f_vector_2).err();
        assert_eq!(res, Some(VectorError::DimensionMismatch));
    }

    #[test]
    fn fvec_and_ivec_addition_failure() {
        let mut f_vector = Vector::<f64>::new(vec![3.3, 4.1]);
        let i_vector = Vector::<i64>::new(vec![3, 4, 52]);
        let res = f_vector.add_i(&i_vector).err();
        assert_eq!(res, Some(VectorError::DimensionMismatch));
    }

    #[test]
    fn test_dot_product_i_success() {
        let vec1 = Vector::<i64>::new(vec![1, 2, 3]);
        let vec2 = Vector::<i64>::new(vec![4, 5, 6]);

        let result = vec1.dot_product(&vec2);
        assert_eq!(result, Ok(32));
    }

    #[test]
    fn test_dot_product_i_dimension_mismatch() {
        let vec1 = Vector::<i64>::new(vec![1, 2, 3]);
        let vec2 = Vector::<i64>::new(vec![4, 5]);

        let result = vec1.dot_product(&vec2);
        assert_eq!(result, Err(VectorError::DimensionMismatch));
    }

    #[test]
    fn test_dot_product_f_success() {
        let vec1 = Vector::<f64>::new(vec![1.0, 2.0, 3.0]);
        let vec2 = Vector::<f64>::new(vec![4.0, 5.0, 6.0]);

        let result = vec1.dot_product_f(&vec2);
        assert_eq!(result, Ok(32.0));
    }

    #[test]
    fn test_dot_product_f_and_i_success() {
        let vec1 = Vector::<f64>::new(vec![1.0, 2.0, 3.0]);
        let vec2 = Vector::<i64>::new(vec![4, 5, 6]);

        let result = vec1.dot_product_i(&vec2);
        assert_eq!(result, Ok(32.0));
    }

    #[test]
    fn test_dot_product_f_dimension_mismatch() {
        let vec1 = Vector::<f64>::new(vec![1.0, 2.0, 3.0]);
        let vec2 = Vector::<f64>::new(vec![4.0, 5.0]);

        let result = vec1.dot_product_f(&vec2);
        assert_eq!(result, Err(VectorError::DimensionMismatch));
    }

    #[test]
    fn test_dot_product_f_and_i_dimension_mismatch() {
        let vec1 = Vector::<f64>::new(vec![1.0, 2.0, 3.0]);
        let vec2 = Vector::<i64>::new(vec![40, 50]);

        let result = vec1.dot_product_i(&vec2);
        assert_eq!(result, Err(VectorError::DimensionMismatch));
    }

    #[test]
    fn test_dot_product_i_empty_vectors() {
        let vec1 = Vector::<i64>::new(vec![]);
        let vec2 = Vector::<i64>::new(vec![]);

        let result = vec1.dot_product(&vec2);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_dot_product_f_empty_vectors() {
        let vec1 = Vector::<f64>::new(vec![]);
        let vec2 = Vector::<f64>::new(vec![]);

        let result = vec1.dot_product_f(&vec2);
        assert_eq!(result, Ok(0.0));
    }
    #[test]
    fn test_dot_product_f_and_i_empty_vectors() {
        let vec1 = Vector::<f64>::new(vec![]);
        let vec2 = Vector::<i64>::new(vec![]);

        let result = vec1.dot_product_i(&vec2);
        assert_eq!(result, Ok(0.0));
    }
}
