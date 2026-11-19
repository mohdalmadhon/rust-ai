use crate::errors::Result;
use crate::descriptive;
use crate::utils;

pub trait Stats {
    fn mean(&self) -> Result<f64>;
    fn median(&self) -> Result<f64>;
    fn mode(&self) -> Result<Vec<f64>>;
    fn var(&self) -> Result<f64>;
    fn pop_var(&self) -> Result<f64>;
    fn std_dev(&self) -> Result<f64>;
    fn pop_std_dev(&self) -> Result<f64>;
    fn percentile(&self, x: f64) -> Result<f64>;
}

pub trait Sort {
    fn merge_sort(&mut self) -> ();
}

impl Stats for [f64] {
    fn mean(&self) -> Result<f64> {
        descriptive::mean(self)
    }
    fn median(&self) -> Result<f64> {
        descriptive::median(self)
    }
    fn mode(&self) -> Result<Vec<f64>> {
       descriptive::mode(self) 
    }
    fn var(&self) -> Result<f64> {
        descriptive::var(self)
    }
    fn pop_var(&self) -> Result<f64> {
        descriptive::pop_var(self)
    }
    fn std_dev(&self) -> Result<f64> {
        descriptive::std_dev(self)
    }
    fn pop_std_dev(&self) -> Result<f64> {
        descriptive::pop_std_dev(self)
    }
    fn percentile(&self, x: f64) -> Result<f64> {
        descriptive::percentile(self, x)
    }
}

impl Sort for [f64] {
    fn merge_sort(&mut self) -> () {
        utils::sort::merge_sort(self);
    }
}