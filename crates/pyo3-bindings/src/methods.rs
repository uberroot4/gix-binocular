use crate::discover_repository;
use pyo3::exceptions::PyValueError;
use pyo3::{pyfunction, PyResult, Python};

#[pyfunction]
#[pyo3(
    signature = (git_dir, branches, skip_merges = false)
)]
pub(crate) fn traverse_commit_graph(
    #[allow(unused_variables)] py: Python<'_>,
    git_dir: String,
    branches: Vec<String>,
    skip_merges: bool,
) -> PyResult<Vec<crate::models::PyGitCommitMetric>> {
    let repo = discover_repository(git_dir).expect("Repository not found");

    use commits::traverse;
    if let Ok(result) = traverse::traverse_commit_graph(repo, branches, skip_merges) {
        let b: Vec<crate::models::PyGitCommitMetric> = result
            .into_iter()
            .map(|r| crate::models::PyGitCommitMetric::from(r))
            .collect();
        return Ok(b);
    }
    Err(PyValueError::new_err("Error traversing commit graph"))
}

#[pyfunction]
#[pyo3(
    signature = (git_dir, commitlist, threads, skip_merges = false, breadth_first = false, follow = false, limit = None)

)]
pub(crate) fn get_diffs(
    #[allow(unused_variables)] py: Python<'_>,
    git_dir: String,
    commitlist: Vec<String>,
    threads: Option<usize>,
    skip_merges: bool,
    breadth_first: bool,
    follow: bool,
    limit: Option<usize>,
) -> PyResult<Vec<crate::models::PyGitDiffMetric>> {
    let repo = discover_repository(git_dir).expect("Repository not found");
    let algo = gix::diff::blob::Algorithm::Histogram;
    use cartography_diff::traverse::traverse_commit_graph;

    if let Ok(result) = py.allow_threads(move || {
        traverse_commit_graph(
            &repo,
            commitlist,
            threads.unwrap_or(1),
            skip_merges,
            Some(algo),
            breadth_first,
            follow,
            limit,
        )
    }) {
        let b: Vec<crate::models::PyGitDiffMetric> = result
            .into_iter()
            .map(|r| crate::models::PyGitDiffMetric::from(r))
            .collect();
        return Ok(b);
    }
    Err(PyValueError::new_err(
        "Error traversing commit graph for diffs",
    ))
}
