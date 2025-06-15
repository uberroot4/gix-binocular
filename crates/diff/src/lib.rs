mod objects {
    mod outcome;

    pub use outcome::GitDiffOutcome;
    pub(crate) use outcome::GitDiffOutcomeVec;
}

pub use objects::GitDiffOutcome;

mod git {
    pub(crate) mod commit;
    pub(crate) mod traverse;
}

pub mod utils {
    pub mod git_helper;
    pub(crate) mod thread_helper;
}

pub mod traversal {
    use crate::objects::{GitDiffOutcome, GitDiffOutcomeVec};
    use log::{info, trace};
    use polars::frame::DataFrame;
    use shared::VecDataFrameExt;

    pub fn main(
        repo: &gix::Repository,
        commitlist: Vec<String>,
        max_threads: usize,
        skip_merges: bool,
        diff_algorithm: Option<gix::diff::blob::Algorithm>,
        breadth_first: bool,
        follow: bool,
        limit: Option<usize>, // TODO remove or implement smth else here
    ) -> anyhow::Result<Vec<GitDiffOutcome>> {
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

    pub fn process(
        repo: &gix::Repository,
        commitlist: Vec<String>,
        max_threads: usize,
        skip_merges: bool,
        diff_algorithm: Option<gix::diff::blob::Algorithm>,
        breadth_first: bool,
        follow: bool,
        limit: Option<usize>, // TODO remove or implement smth else here
    ) -> anyhow::Result<DataFrame> {
        let diff_results = main(
            repo,
            commitlist,
            max_threads,
            skip_merges,
            diff_algorithm,
            breadth_first,
            follow,
            limit,
        )?;
        let df = GitDiffOutcomeVec(diff_results).to_df()?;
        Ok(df)
    }
}
