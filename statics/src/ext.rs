use crate::errors::Result;
use crate::descriptive;

pub trait Stats {
    fn mean(&self) -> Result<f64>;
}

impl Stats for [f64] {
    fn mean(&self) -> Result<f64> {
        descriptive::mean(self)
    }
}