use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(name = "log", about = "git log example", version = option_env!("GIX_VERSION"))]
pub struct Args {
    /// Alternative git directory to use
    #[clap(name = "dir", long = "git-dir")]
    pub git_dir: Option<PathBuf>,
    /// Number of commits to return
    #[clap(short, long)]
    pub limit: Option<usize>,
    /// Commits are sorted as they are mentioned in the commit graph.
    #[clap(short, long)]
    pub breadth_first: bool,
    /// Show only non-merge commits (implies --max-parents=1)
    #[clap(long)]
    pub no_merges: bool,
    /// The ref-spec for the first commit to use, or HEAD (default).
    #[clap(name = "commit")]
    pub committish: Option<String>,
    /// Number of threads to use during commit processing
    #[clap(short, long)]
    pub threads: Option<usize>,
    /// Algorithm to use
    #[clap(short, long, value_enum, default_value_t = DiffAlgorithm::Histogram)]
    pub algorithm: DiffAlgorithm,
    /// Alternative git directory to use
    #[clap(short = 'o', long = "output-format", value_enum, default_value_t = OutputFormat::Render)]
    pub output_format: OutputFormat,
    /// Print log to stdout
    #[clap(short, long, value_name = "LOGFILE", action = clap::ArgAction::Set, default_missing_value = "", num_args = 0..=1)]
    pub verbose: Option<String>,
}

// gix::diff::blob::Algorithm
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DiffAlgorithm {
    Histogram, //gix::diff::blob::Algorithm::Histogram
    Myers,        // gix::diff::blob::Algorithm::Myers
    MyersMinimal, // gix::diff::blob::Algorithm::MyersMinimal
}

#[derive(clap::ValueEnum, Debug, PartialEq, Clone)]
/// Represent the different type of available formats
pub enum OutputFormat {
    /// Render the output as table
    Render,
    /// Print the output in json format
    JSON,
    /// Print the output in csv format
    CSV,
}
