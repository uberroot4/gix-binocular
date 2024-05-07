use gix::Repository;
use pyo3::prelude::*;
use csv::Writer;

fn discover_repository(git_dir: String) -> anyhow::Result<Repository> {
    let repo = gix::discover(git_dir.clone().trim())?;
    Ok(repo)
}

fn to_csv(records: Vec<diff::metrics::GitDiffMetrics>) -> Result<String, Box<dyn std::error::Error>> {
    let mut writer = Writer::from_writer(vec![]);
    writer.write_record(["commit", "parent", "total_number_of_files_changed", "total_number_of_insertions", "total_number_of_deletions"])?;
    for res in records {
        writer.write_record([res.commit.to_string(), res.parent.map_or_else(|| "NULL".to_string(), |parent| parent.to_string()), res.total_number_of_files_changed.to_string(), res.total_number_of_insertions.to_string(), res.total_number_of_deletions.to_string()])?;
    }
    Ok(String::from_utf8(writer.into_inner()?)?)
}

#[pyfunction]
#[pyo3(signature = (git_dir, threads, no_merges = false, breadth_first = false, committish = None, limit = None))]
fn traverse(py: Python<'_>, git_dir: String, threads: Option<usize>, no_merges: bool, breadth_first: bool, committish: Option<String>, limit: Option<usize>) -> PyResult<Option<String>> {
    let repo = discover_repository(git_dir).expect("Repository not found");
    let algo = gix::diff::blob::Algorithm::Histogram;
    use diff::traverse::traverse_commit_graph;
    let repo_sync = repo.clone().into_sync();

    env_logger::init();
    // println!("threads: {:?}", threads.unwrap_or(1));

    if let Ok(result) =
        // py.allow_threads({
        //     let repo = repo_sync.clone().to_thread_local();
        //     move || traverse_commit_graph(&repo, threads.unwrap_or(1), no_merges, Some(algo), breadth_first, committish, limit)
        // })
    traverse_commit_graph(&repo, threads.unwrap_or(1), no_merges, Some(algo), breadth_first, committish, limit)
    {
        if let Ok(csv) = to_csv(result) {
            // println!("{}", csv);
            return Ok(Some(csv));
        }
    }
    Ok(None)
}

/// A Python module implemented in Rust.
#[pymodule]
fn gix_pyo3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(traverse, m)?)?;
    Ok(())
}
