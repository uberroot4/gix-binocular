mod git {
    pub mod metrics;
    pub mod traverse;
}

// pub use crate::git::traverse;

pub mod traversal {
    use crate::git::metrics::GitCommitMetricVec;
    pub use crate::git::traverse::{traverse_commit_graph as main, traverse_from_to};
    use polars::frame::DataFrame;
    use shared::VecDataFrameExt;
    use std::ops::Deref;

    pub fn from_to(
        repo: gix::Repository,
        source: &String,
        target: &Option<String>,
    ) -> anyhow::Result<DataFrame> {
        let binding = repo.clone();
        let source_commit = binding
            .rev_parse_single(source.deref())?
            .object()?
            .try_into_commit()?;
        let target_commit = match target {
            None => None,
            Some(target) => Option::from({
                binding
                    .rev_parse_single(target.deref())?
                    .object()?
                    .try_into_commit()?
            }),
        };

        let commit_metric_vec = traverse_from_to(&repo, &source_commit, &target_commit)?;

        let vectorized = GitCommitMetricVec(commit_metric_vec);
        let lf = vectorized.to_df()?;

        Ok(lf)
        // Ok(polars::frame::DataFrame::empty())
    }

    pub fn process(
        repo: gix::Repository,
        branches: Vec<String>,
        skip_merges: bool,
    ) -> anyhow::Result<DataFrame> {
        let commit_metric_vec = main(repo, branches, skip_merges)?;

        let vectorized = GitCommitMetricVec(commit_metric_vec);
        let lf = vectorized.to_df()?;

        Ok(lf)
    }
}
