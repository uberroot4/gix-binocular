mod objects {
    mod changes;
    mod outcome;

    use changes::ChangesInfo;
    pub(crate) use outcome::{GitDiffOutcome, GitDiffOutcomeVec};
}

mod git {
    pub(crate) mod commit;
    pub mod traverse;
}

mod utils {
    pub(crate) mod git_helper;
    pub(crate) mod thread_helper;
}

pub mod traversal {
    use crate::objects::GitDiffOutcome;
    use log::{info, trace};
    use polars::frame::DataFrame;

    pub fn main(
        repo: &gix::Repository,
        commitlist: Vec<String>,
        max_threads: usize,
        skip_merges: bool,
        diff_algorithm: Option<gix::diff::blob::Algorithm>,
        breadth_first: bool,
        follow: bool,
        limit: Option<usize>,
    ) -> anyhow::Result<DataFrame> {
        let cl = crate::git::commit::prepare_commit_list(
            repo,
            commitlist,
            skip_merges,
            breadth_first,
            follow,
            limit,
        )?;
        info!("Processing {} commit(s)", cl.iter().count());
        let num_threads = crate::utils::thread_helper::num_threads(max_threads);
        trace!("threads used: {:?}", num_threads);
        let diffs =
            crate::git::traverse::traverse_commit_graph(repo, cl, num_threads, diff_algorithm)?;

        Ok(diffs)
    }
}

pub use crate::git::traverse;