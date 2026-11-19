pub mod descriptive;
pub mod errors;
pub mod ext;

pub use ext::Stats;
pub use descriptive::{mean};
pub use errors::{StatsErrors, Result};