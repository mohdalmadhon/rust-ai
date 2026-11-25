use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

pub trait Num: Copy + PartialOrd + PartialEq + Div + Sub + Add + Mul + AddAssign + SubAssign {
    fn to_f64(self) -> f64;
}

macro_rules! impl_num {
    ($($t:ty),*) => {
        $(impl Num for $t {
            fn to_f64(self) -> f64 {
                self as f64
            }
        })*
    };
}

impl_num!(f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);