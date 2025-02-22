mod objects {
    mod changes;
    mod outcome;

    use changes::ChangesInfo;
    pub(crate) use outcome::GitDiffOutcome;
}

mod git {
    pub(crate) mod commit;
    pub mod traverse;
}

mod utils {
    pub mod git_helper;
    pub mod structs;
}

pub mod traversal {
    use log::{info, trace};
    use crate::objects::GitDiffOutcome;

    pub fn main(
        repo: &gix::Repository,
        commitlist: Vec<String>,
        max_threads: usize,
        skip_merges: bool,
        diff_algorithm: Option<gix::diff::blob::Algorithm>,
        breadth_first: bool,
        follow: bool,
        limit: Option<usize>,
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
        let num_threads = num_threads(max_threads);
        trace!("threads used: {:?}", num_threads);
        let diffs =
            crate::git::traverse::traverse_commit_graph(repo, cl, num_threads, diff_algorithm)?;

        Ok(diffs)
    }

    fn num_threads(max_threads: usize) -> usize {
        std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(1)
            .min(max_threads)
    }

    #[cfg(test)]
    mod tests {
    }
}

pub use crate::git::traverse;
pub use crate::utils::git_helper;
pub use crate::utils::structs;

#[cfg(test)]
mod tests {
}
