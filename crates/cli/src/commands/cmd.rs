use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "gitinfo")]
#[command(about = "GitInfo CLI", long_about = None, arg_required_else_help = true, version)]
pub struct Cli {
    #[clap(flatten)]
    pub global_opts: GlobalOpts,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Compare two commits, summary of diffs
    Diff(crate::commands::diff::Args),
    /// Get the commit ids of a repository.
    Commits(crate::commands::commits::Args),
    /// Perform blame for multiple commits and files
    Blame(crate::commands::blame::Args)
}


#[derive(Debug, clap::Args)]
pub struct GlobalOpts {
    /// Print log to stdout or LOGFILE if specified
    #[clap(
        short, long, value_name = "LOGFILE", action = clap::ArgAction::Set, default_missing_value = "", num_args = 0..=1, global = true
    )]
    pub verbose: Option<String>,
    /// Define how the output is printed
    #[clap(
        long = "format", value_enum, default_value_t = crate::output_format::OutputFormat::Render, global = true
    )]
    pub output_format: crate::output_format::OutputFormat,
    #[clap(long = "out", global = true)]
    /// File to write the output to
    pub output_file: Option<PathBuf>,
    /// Show only non-merge commits (implies --max-parents=1)
    #[clap(long, global = true)]
    pub skip_merges: bool,
    /// Number of commits to return
    #[clap(long, global = true)]
    pub limit: Option<usize>,
    /// git directory to use
    #[clap(name = "dir", long = "git-dir", global = true, default_missing_value = ".")]
    pub git_dir: Option<PathBuf>,
}