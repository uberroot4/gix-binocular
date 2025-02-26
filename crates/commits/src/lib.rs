mod git {
    pub mod traverse;
    pub mod metrics;
}

// pub use crate::git::traverse;

pub mod traversal {
    use polars::frame::DataFrame;
    use shared::VecDataFrameExt;
    pub use crate::git::traverse::{traverse_commit_graph as main};
    use crate::git::metrics::GitCommitMetricVec;

    pub fn process(
        repo: gix::Repository,
        branches: Vec<String>,
        skip_merges: bool,
    ) -> anyhow::Result<DataFrame> {
        let commit_metric_vec = main(
            repo,
            branches,
            skip_merges
        )?;

        let vectorized = GitCommitMetricVec(commit_metric_vec);
        let lf = vectorized.to_df()?;

        Ok(lf)
    }
}