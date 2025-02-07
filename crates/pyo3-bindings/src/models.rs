use cartography_diff::metrics::GitDiffMetrics;
use commits::GitCommitMetric;
use pyo3::IntoPyObject;
use shared::Sig;
use std::collections::HashMap;
#[derive(Debug, IntoPyObject)]
pub struct PySig {
    pub name: String,
    pub email: String,
    pub time: chrono::DateTime<chrono::Utc>,
}
#[derive(IntoPyObject, Debug)]
pub struct PyGitDiffMetric {
    pub change_map: HashMap<String, (u32, u32)>,
    pub total_number_of_files_changed: usize,
    pub total_number_of_insertions: u32,
    pub total_number_of_deletions: u32,
    pub commit: String,
    pub parent: Option<String>,
    pub committer: Option<PySig>,
    pub author: Option<PySig>,
}

#[derive(IntoPyObject, Debug)]
pub struct PyGitCommitMetric {
    pub commit_str: String,
    pub message: String,
    pub committer: Option<PySig>,
    pub author: Option<PySig>,
    pub branch: Option<String>,
    pub parents: Vec<String>,
}

impl From<cartography_diff::metrics::GitDiffMetrics> for PyGitDiffMetric {
    fn from(value: GitDiffMetrics) -> Self {
        Self {
            change_map: value
                .change_map
                .into_iter()
                .map(|(key, value)| (key.to_string(), value))
                .collect(),
            total_number_of_files_changed: value.total_number_of_files_changed,
            total_number_of_insertions: value.total_number_of_insertions,
            total_number_of_deletions: value.total_number_of_deletions,
            commit: value.commit.to_string(),
            parent: value.parent.map(|p| p.to_string()),
            committer: value.committer.map(|sig| sig.into()),
            author: value.author.map(|sig| sig.into()),
        }
    }
}

impl From<commits::GitCommitMetric> for PyGitCommitMetric {
    fn from(value: GitCommitMetric) -> Self {
        Self {
            commit_str: value.commit_str,
            message: value.message,
            branch: value.branch,
            parents: value.parents,
            committer: value.committer.map(|sig| sig.into()),
            author: value.author.map(|sig| sig.into()),
        }
    }
}

impl From<shared::Sig> for PySig {
    fn from(value: Sig) -> Self {
        Self {
            name: value.name.to_string(),
            email: value.email.to_string(),
            time: super::tz_utils::time_to_utc_with_offset(value.time),
        }
    }
}
