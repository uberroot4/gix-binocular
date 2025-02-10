mod git {
    pub mod blame;
    pub mod metrics;
}

pub use crate::git::blame::{traverse_commit_graph as lookup};
pub use crate::git::metrics::GitBlameMetric;

pub struct GitCommitMetricVector {
    value_vector: Vec<GitBlameMetric>,
}