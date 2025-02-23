mod utils {
    pub mod logging;
}

use polars::error::PolarsResult;
use polars::prelude::DataFrame;
pub use crate::utils::logging;

pub mod signature;
mod tz_utils;
pub use crate::tz_utils::time_to_utc_with_offset;

// Define an extension trait for Vec<MyType>
pub trait VecDataFrameExt {
    fn to_df(&self) -> PolarsResult<DataFrame>;
}