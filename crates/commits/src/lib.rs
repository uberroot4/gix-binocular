mod git {
    pub mod traverse;
    // pub(crate) mod sig;
    pub mod metrics;
}

pub use crate::git::traverse;

pub use crate::git::metrics::GitCommitMetric;
pub struct GitCommitMetricVector {
    value_vector: Vec<GitCommitMetric>,
}