use crate::errors::{Result, StatsErrors};

pub fn mean(data: &[f64]) -> Result<f64> {
    let n: usize = data.len();
    if n == 0 {
        return Err(StatsErrors::EmptyInput);
    }
    Ok(data.iter().sum::<f64>() / n as f64)
}

pub fn median(data: &[f64]) -> Result<f64> {
    let n: usize = data.len();
    if n == 0 {
        return Err(StatsErrors::EmptyInput);
    }
    Ok()
} 