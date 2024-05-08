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
    /// Compare two commits
    Diff(crate::commands::diff::Args)
}

#[derive(Debug, clap::Args)]
pub struct GlobalOpts {
    /// Print log to stdout or LOGFILE if specified
    #[clap(short, long, value_name = "LOGFILE", action = clap::ArgAction::Set, default_missing_value = "", num_args = 0..=1)]
    pub verbose: Option<String>,
}