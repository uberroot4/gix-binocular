use gix::blame::Statistics;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct BlameStatistics /*(blame::Statistics);*/ {
    pub commits_traversed: usize,
    pub commits_to_tree: usize,
    pub trees_decoded: usize,
    pub trees_diffed: usize,
    pub blobs_diffed: usize,
}

impl From<gix::blame::Statistics> for BlameStatistics {
    fn from(value: Statistics) -> Self {
        Self {
            commits_traversed: value.commits_to_tree,
            commits_to_tree: value.commits_to_tree,
            trees_decoded: value.trees_decoded,
            trees_diffed: value.trees_diffed,
            blobs_diffed: value.blobs_diffed,
        }
    }
}