use std::path::PathBuf;

#[derive(Debug, clap::Parser, Clone)]
#[clap(name = "diff", about = "diff")]
pub struct Args {
    //// Alternative git directory to use
    //#[clap(name = "dir", long = "git-dir")]
    //pub git_dir: PathBuf,

    /// Commits are sorted as they are mentioned in the commit graph.
    #[clap(short, long)]
    pub breadth_first: bool,

    /// The ref-spec for the first commit to use, or HEAD (default).
    #[clap(name = "commit")]
    pub committish: Option<String>,
    /// Number of threads to use during commit processing
    #[clap(short, long)]
    pub threads: Option<usize>,
    /// Algorithm to use
    #[clap(short, long, value_enum, default_value_t = DiffAlgorithm::Histogram)]
    pub algorithm: DiffAlgorithm,
    //// Alternative git directory to use
    //#[clap(short = 'o', long = "output-format", value_enum, default_value_t = crate::output_format::OutputFormat::Render)]
    //pub output_format: crate::output_format::OutputFormat,
}

// gix::diff::blob::Algorithm
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DiffAlgorithm {
    Histogram, //gix::diff::blob::Algorithm::Histogram
    Myers,        // gix::diff::blob::Algorithm::Myers
    MyersMinimal, // gix::diff::blob::Algorithm::MyersMinimal
}
