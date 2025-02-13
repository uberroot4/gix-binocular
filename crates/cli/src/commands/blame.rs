use crate::diff::DiffAlgorithm;

#[derive(Debug, clap::Parser, Clone)]
#[clap(name = "blame", about = "blame")]
pub struct Args {
    /// Number of threads to use during commit processing
    #[clap(short, long)]
    pub threads: Option<usize>,
    /// Algorithm to use
    #[clap(short, long, value_enum, default_value_t = DiffAlgorithm::Histogram)]
    pub algorithm: DiffAlgorithm,
    /// The definition JSON-file for processing blames
    #[clap(name = "file", required = true)]
    pub defines_file: String,
}
