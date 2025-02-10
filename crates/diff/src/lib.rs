mod git {
    pub(crate) mod blame;
    pub(crate) mod commit;
    pub mod metrics;
    pub mod traverse;
}

mod utils {
    pub mod blame_info;
    pub mod git_helper;
    pub mod structs;
}

pub mod traversal {
    use crate::git::blame;
    use log::{info, trace};
    // use std::cmp::min_by;
    use crate::metrics::GitDiffMetrics;

    pub fn main(
        repo: &gix::Repository,
        commitlist: Vec<String>,
        max_threads: usize,
        skip_merges: bool,
        diff_algorithm: Option<gix::diff::blob::Algorithm>,
        breadth_first: bool,
        follow: bool,
        limit: Option<usize>,
    ) -> anyhow::Result<Vec<GitDiffMetrics>> {
        let cl = crate::git::commit::prepare_commit_list(
            repo,
            commitlist,
            skip_merges,
            breadth_first,
            follow,
            limit,
        )?;
        info!("Processing {} commit(s)", cl.iter().count());
        let num_threads = num_threads(max_threads);
        trace!("threads used: {:?}", num_threads);
        let diffs =
            crate::git::traverse::traverse_commit_graph(repo, cl, num_threads, diff_algorithm)?;
        let blames = blame::process(repo, diffs, diff_algorithm, num_threads);
        blames
    }

    fn num_threads(max_threads: usize) -> usize {
        std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(1)
            .min(max_threads)
    }
}
pub use crate::git::metrics;
pub use crate::git::traverse;
pub use crate::utils::git_helper;
pub use crate::utils::structs;

pub struct GitDiffMetricsVector {
    value_vector: Vec<git::metrics::GitDiffMetrics>,
}
