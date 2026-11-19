pub mod descriptive;
pub mod errors;
pub mod ext;
pub mod utils;

pub use utils::sort;
pub use ext::*;
pub use descriptive::*;
pub use errors::{StatsErrors, Result};