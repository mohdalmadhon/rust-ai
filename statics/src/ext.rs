use crate::errors::Result;
use crate::descriptive;
use crate::utils;

pub trait Stats {
    fn mean(&self) -> Result<f64>;
    fn median(&self) -> Result<f64>;
    fn mode(&self) -> Result<Vec<f64>>;
}

pub trait Sort {
    fn merge_sort(&mut self) -> ();
}

impl Stats for [f64] {
    fn mean(&self) -> Result<f64> {
        descriptive::mean(self)
    }
    fn median(&self) -> Result<f64> {
        descriptive::mean(self)
    }
    fn mode(&self) -> Result<Vec<f64>> {
       descriptive::mode(self) 
    }
}

impl Sort for [f64] {
    fn merge_sort(&mut self) -> () {
        utils::sort::merge_sort(self);
    }
}