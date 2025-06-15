mod utils {
    pub mod logging;
}
pub use crate::utils::logging;
#[cfg(not(target_os = "wasi"))]
use polars::{error::PolarsResult, prelude::DataFrame};

pub mod signature;
mod tz_utils;
pub use crate::tz_utils::time_to_utc_with_offset;

// Define an extension trait for Vec<MyType>
#[cfg(not(target_os = "wasi"))]
pub trait VecDataFrameExt {
    fn to_df(&self) -> PolarsResult<DataFrame>;
}
