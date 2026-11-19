use crate::{Sort, errors::{Result, StatsErrors}};
use super::central::mean;

fn sum_sq_dev(data: &[f64]) -> Result<f64> {
    let m = mean(data)?;
    Ok(data.iter().map(|x| (x - m).powi(2)).sum())
}

pub fn var(data: &[f64]) -> Result<f64> {
    let n = data.len();
    if n < 2 {
        return Err(StatsErrors::NotEnoughData { needed: 2, got: n });
    }
    Ok(sum_sq_dev(data)? / (n - 1) as f64)
}

pub fn pop_var(data: &[f64]) -> Result<f64> {
    let n = data.len();
    if n == 0 {
        return Err(StatsErrors::EmptyInput);
    }
    Ok(sum_sq_dev(data)? / n as f64)
}

pub fn std_dev(data: &[f64]) -> Result<f64> {
    Ok(var(data)?.sqrt())
}

pub fn pop_std_dev(data: &[f64]) -> Result<f64> {
    Ok(pop_var(data)?.sqrt())
}

pub fn percentile(data: &[f64], x: f64) -> Result<f64> {
    if data.is_empty() {
        return Err(StatsErrors::EmptyInput);
    }
    let mut vec: Vec<f64> = data.to_vec();
    vec.merge_sort();

    let r = (x / 100.0) * (data.len() - 1) as f64;
    let i = r.floor() as usize;
    let f = r - i as f64;

    if i + 1 < vec.len() {
        Ok(vec[i] + f * (vec[i + 1] - vec[i]))
    } else {
        Ok(vec[i])
    }
}