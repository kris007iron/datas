use std::ops::Mul;

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

    pub fn add_i(&mut self, vector: Vector<i64>) -> Result<(), VectorError> {
        if self.dimensions == vector.dimensions {
            for (index, component) in self.components.iter_mut().enumerate() {
                *component += vector.components[index] as f64;
            }
            return Ok(());
        } else {
            return Err(VectorError::DimensionMismatch);
        }
    }

    pub fn add_f(&mut self, vector: Vector<f64>) -> Result<(), VectorError> {
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
