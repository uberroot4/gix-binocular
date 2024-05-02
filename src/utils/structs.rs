use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(name = "log", about = "git log example", version = option_env!("GIX_VERSION"))]
pub struct Args {
    /// Alternative git directory to use
    #[clap(name = "dir", long = "git-dir")]
    pub git_dir: Option<PathBuf>,
    /// Number of commits to return
    #[clap(short, long)]
    pub count: Option<usize>,
    // /// Number of commits to skip
    // #[clap(short, long)]
    // pub skip: Option<usize>,
    /// Commits are sorted as they are mentioned in the commit graph.
    #[clap(short, long)]
    pub breadth_first: bool,
    /// Commits are sorted by their commit time in descending order.
    #[clap(short, long)]
    pub newest_first: bool,
    // /// Show commits with the specified minimum number of parents
    // #[clap(long)]
    // pub min_parents: Option<usize>,
    // /// Show commits with the specified maximum number of parents
    // #[clap(long)]
    // pub max_parents: Option<usize>,
    // /// Show only merge commits (implies --min-parents=2)
    // #[clap(long)]
    // pub merges: bool,
    /// Show only non-merge commits (implies --max-parents=1)
    #[clap(long)]
    pub no_merges: bool,
    /// Reverse the commit sort order (and loads all of them into memory).
    #[clap(short, long)]
    pub reverse: bool,
    /// The ref-spec for the first commit to log, or HEAD.
    #[clap(name = "commit")]
    pub committish: Option<String>,
    // /// The path interested in log history of
    // #[clap(name = "path")]
    // pub paths: Vec<PathBuf>,
    /// Number of threads to use during commit processing
    #[clap(short, long)]
    pub threads: Option<usize>,
    /// Algorithm to use
    #[clap(short, long, value_enum, default_value_t=DiffAlgorithm::Histogram)]
    pub algorithm: DiffAlgorithm,
}

// gix::diff::blob::Algorithm
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DiffAlgorithm {
    Histogram, //gix::diff::blob::Algorithm::Histogram
    Myers,        // gix::diff::blob::Algorithm::Myers
    MyersMinimal, // gix::diff::blob::Algorithm::MyersMinimal
}
