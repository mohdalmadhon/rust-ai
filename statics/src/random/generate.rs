use super::rng::Rng;
use crate::errors::{Result, StatsErrors};

pub fn random_set(rng: &mut Rng, count: usize, min: f64, max: f64) -> Result<Vec<f64>> {
    if !(min < max) {
        return Err(StatsErrors::InvalidRange);
    }
    Ok((0..count).map(|_| rng.range_f64(min, max)).collect())
}

pub fn random_int_set(rng: &mut Rng, count: usize, min: i64, max: i64) -> Result<Vec<f64>> {
    if max <= min {
        return Err(StatsErrors::InvalidRange);
    }
    Ok((0..count).map(|_| rng.range_i64(min, max) as f64).collect())
}