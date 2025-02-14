mod git {
    pub mod traverse;
    pub mod metrics;
}

pub use crate::git::traverse;
pub use crate::git::metrics::GitCommitMetric;