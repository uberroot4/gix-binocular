use gix::Repository;
use pyo3::prelude::*;
use cli::output_format::OutputFormat;
use commits::traverse;
use render::Renderable;

fn discover_repository(git_dir: String) -> anyhow::Result<Repository> {
    let repo = gix::discover(git_dir.clone().trim())?;
    Ok(repo)
}

#[pyfunction]
#[pyo3(
    signature = (git_dir, branches/*threads*/, no_merges = false/*, breadth_first = false, committish = None, limit = None*/)
)]
fn traverse_commit_graph(py: Python<'_>, git_dir: String, branches: Vec<String>/*threads: Option<usize>*/, no_merges: bool /*, breadth_first: bool, committish: Option<String>, limit: Option<usize>*/) -> PyResult<Option<String>> {
    let repo = discover_repository(git_dir).expect("Repository not found");
    //let algo = gix::diff::blob::Algorithm::Histogram;
    //use diff::traverse::traverse_commit_graph;
    //let repo_sync = repo.clone().into_sync();

    // env_logger::init();
    // println!("threads: {:?}", threads.unwrap_or(1));

    use commits::traverse;
    if let Ok(result) = traverse::traverse_commit_graph(repo, branches, no_merges)
    //traverse_commit_graph(&repo, threads.unwrap_or(1), no_merges, Some(algo), breadth_first, committish, limit)
    {
        let printable_result: commits::GitCommitMetricVector = result.into();

        return Ok(Option::from(printable_result.format(OutputFormat::CSV)));
    }
    Ok(None)
}

#[pyfunction]
#[pyo3(
    signature = (git_dir, threads, no_merges = false, breadth_first = false, committish = None, limit = None)

)]
fn get_diffs(py: Python<'_>, git_dir: String, threads: Option<usize>, no_merges: bool, breadth_first: bool, committish: Option<String>, limit: Option<usize>) -> PyResult<Option<String>> {
    let repo = discover_repository(git_dir).expect("Repository not found");
    let algo = gix::diff::blob::Algorithm::Histogram;
    use diff::traverse::traverse_commit_graph;
    //let repo_sync = repo.clone().into_sync();

    if let Ok(result) = traverse_commit_graph(&repo, threads.unwrap_or(1), no_merges, Some(algo), breadth_first, committish, limit)
    {
        let printable_result: diff::GitDiffMetricsVector = result.into();

        return Ok(Option::from(printable_result.format(OutputFormat::CSV)));
    }
    Ok(None)
}

/// A Python module implemented in Rust.
#[pymodule]
fn gix_pyo3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(traverse_commit_graph, m)?)?;
    m.add_function(wrap_pyfunction!(get_diffs, m)?)?;
    Ok(())
}
