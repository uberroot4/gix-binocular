mod commands {
    pub mod cmd;
    pub(crate) mod commits;
    pub mod diff;
    pub mod output_format;
}

pub use crate::commands::cmd;
pub use crate::commands::diff;
pub use crate::commands::output_format;