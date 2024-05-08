mod commands {
    pub mod cmd;
    pub(crate) mod commits;
    pub(crate) mod diff;
}
pub use crate::commands::cmd;