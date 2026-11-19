use super::rng::Rng;
use crate::errors::{Result, StatsErrors};
use std::f64::consts::PI;

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

pub fn random_normal_set(rng: &mut Rng, count: usize, mean: f64, std_dev: f64) -> Result<Vec<f64>> {
    if !(std_dev >= 0.0) {
        return Err(StatsErrors::InvalidStdDev);
    }

    let mut set = Vec::with_capacity(count);

    while set.len() < count {
        let u1 = 1.0 - rng.next_f64();
        let u2 = rng.next_f64();

        let r = (-2.0 * u1.ln()).sqrt();
        let theta = 2.0 * PI * u2;

        set.push(mean + std_dev * r * theta.cos());
        if set.len() < count {
            set.push(mean + std_dev * r * theta.sin());
        }
    }

    Ok(set)
}