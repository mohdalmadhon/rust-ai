pub mod descriptive;
pub mod errors;
pub mod ext;
pub mod utils;
pub mod random;
pub mod arrays;

pub use arrays::*;
pub use random::*;
pub use utils::sort;
pub use ext::*;
pub use descriptive::*;
pub use errors::{StatsErrors, Result};