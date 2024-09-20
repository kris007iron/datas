use std::vec;

#[derive(Debug)]
pub enum VectorError {
    DimensionMismatch,
}

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

    pub fn add(&mut self, vector: Vector<i64>) -> Result<(), VectorError> {
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
