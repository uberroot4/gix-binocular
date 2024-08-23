use std::path::PathBuf;

#[derive(Debug, clap::Parser, Clone)]
#[clap(name = "commits", about = "commits")]
pub struct Args {
    //// Alternative git directory to use
    //#[clap(name = "dir", long = "git-dir")]
    //pub git_dir: PathBuf,

    /// Commits are sorted as they are mentioned in the commit graph.
    #[clap(name = "branches")]
    pub branches: Vec<String>,
}