#[derive(Debug, clap::Parser, Clone)]
#[clap(name = "diff", about = "diff")]
pub struct Args {
    /// Follow the given commit history (only allowed with one SHA!)
    #[clap(long, default_value_t = false)]
    pub follow: bool,

    /// Commits are sorted as they are mentioned in the commit graph.
    #[clap(short, long)]
    pub breadth_first: bool,

    /// Number of threads to use during commit processing
    #[clap(short, long)]
    pub threads: Option<usize>,

    /// Algorithm to use
    #[clap(short, long, value_enum, default_value_t = DiffAlgorithm::Histogram)]
    pub algorithm: DiffAlgorithm,

    #[command(flatten)]
    pub delegate: CommitArgs,
}

// gix::diff::blob::Algorithm
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DiffAlgorithm {
    Histogram,    //gix::diff::blob::Algorithm::Histogram
    Myers,        // gix::diff::blob::Algorithm::Myers
    MyersMinimal, // gix::diff::blob::Algorithm::MyersMinimal
}

#[derive(clap::Args, Clone, Debug)]
#[group(multiple = false, id = "commit_args")]
pub struct CommitArgs {
    /// The ref-spec list for the commits to process (only processes given commit hashes!)
    #[clap(name = "commitlist")]
    pub commitlist: Vec<String>,
}
