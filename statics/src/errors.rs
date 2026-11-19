use std::fmt;

#[derive(Debug, PartialEq)]
pub enum StatsErrors {
    EmptyInput,
    LengthMismatch,
    NotEnoughData { needed: usize, got: usize },
    InvalidRange,
    InvalidStdDev,
}

impl fmt::Display for StatsErrors {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        match self {
            StatsErrors::EmptyInput => write!(f, "input slice is empty"),
            StatsErrors::LengthMismatch => write!(f, "inputs have different lengths"),
            StatsErrors::NotEnoughData { needed, got } => {
                write!(f, "need at least {needed} values, got {got}")
            },
            StatsErrors::InvalidRange => write!(f, "min must be less than max"),
            StatsErrors::InvalidStdDev => write!(f, "standard deviation must be zero or positive"),
        }
    }
}

impl std::error::Error for StatsErrors {}

pub type Result<T> = std::result::Result<T, StatsErrors>;