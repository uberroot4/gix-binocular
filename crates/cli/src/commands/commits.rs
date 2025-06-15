#[derive(Debug, clap::Parser, Clone)]
#[clap(name = "commits", about = "commits")]
pub struct Args {
    /// Commits are sorted as they are mentioned in the commit graph.
    #[clap(name = "branches", required=true)]
    pub branches: Vec<String>,

    // #[clap(long="from", required=false)]
    // pub source_commit: String,
    //
    // #[clap(long="to", required=false)]
    // pub target_commit: Option<String>,
}