use crate::errors::Result;
use crate::descriptive;
use crate::utils;
use crate::utils::num::Num;

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

fn to_f64_vec<T: Num>(data: &[T]) -> Vec<f64> {
    data.iter().map(|x| x.to_f64()).collect()
}

impl<T: Num> Stats for [T] {
    fn mean(&self) -> Result<f64> {
        descriptive::mean(&to_f64_vec(self))
    }
    fn median(&self) -> Result<f64> {
        descriptive::median(&to_f64_vec(self))
    }
    fn mode(&self) -> Result<Vec<f64>> {
        descriptive::mode(&to_f64_vec(self))
    }
    fn var(&self) -> Result<f64> {
        descriptive::var(&to_f64_vec(self))
    }
    fn pop_var(&self) -> Result<f64> {
        descriptive::pop_var(&to_f64_vec(self))
    }
    fn std_dev(&self) -> Result<f64> {
        descriptive::std_dev(&to_f64_vec(self))
    }
    fn pop_std_dev(&self) -> Result<f64> {
        descriptive::pop_std_dev(&to_f64_vec(self))
    }
    fn percentile(&self, x: f64) -> Result<f64> {
        descriptive::percentile(&to_f64_vec(self), x)
    }
}

impl Sort for [f64] {
    fn merge_sort(&mut self) -> () {
        utils::sort::merge_sort(self);
    }
}