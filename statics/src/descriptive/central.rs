use crate::Sort;
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
    let mut vec: Vec<f64> = data.to_vec();
    vec.merge_sort();

    if n % 2 == 0 {
        Ok((vec[n / 2 - 1] + vec[n / 2]) / 2.0)
    } else {
        Ok(vec[n / 2])
    }
} 