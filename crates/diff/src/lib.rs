mod git {
    pub mod traverse;
    pub mod metrics;
}
pub use crate::git::traverse;
pub use crate::git::metrics;

mod utils {
    pub mod structs;
    pub mod git_helper;
}
pub use crate::utils::structs;
pub use crate::utils::git_helper;