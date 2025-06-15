use clap::Parser;
use cli::cmd::{Cli, Commands};
use cli::diff::DiffAlgorithm;
use cli::output_format::OutputFormat;
use dotenv::dotenv;
use log::{debug, trace};
use render::printer::{
    CSVPrinter, JSONPrinter, OutputPrinter, ParquetPrinter, Printer, VoidPrinter,
};
use shared::logging;
use std::time::Instant;

fn main() {
    dotenv().ok();
    let args = Cli::parse_from(gix::env::args_os());

    match &args.global_opts.verbose {
        Some(file) if !file.is_empty() => {
            logging::init_logging(Some(file.to_string()));
        }
        Some(_) => {
            logging::init_logging(None);
        }
        None => {
            println!("Verbosity not enabled");
        }
    }

    let now = Instant::now();
    trace!("args: {:?}", args);
    let git_dir = match args.global_opts.git_dir {
        None => {
            panic!("No Path to .git Repository provided")
        }
        Some(git_dir) => git_dir,
    };
    let repo = match gix::discover(git_dir) {
        Ok(r) => r,
        Err(_) => {
            panic!("Repository not found")
        }
    };

    let printer: Printer = match args.global_opts.output_format {
        OutputFormat::JSON => Printer::Json(JSONPrinter {
            file_path: args.global_opts.output_file,
        }),
        OutputFormat::CSV => Printer::Csv(CSVPrinter {
            file_path: args.global_opts.output_file,
        }),
        OutputFormat::Parquet => Printer::Parquet(ParquetPrinter {
            file_path: args.global_opts.output_file,
        }),
        _ => {
            println!("No output specified!");
            Printer::Void(VoidPrinter {})
        }
    };

    let algo = match &args.command {
        Commands::Diff(diff_args) => Some(&diff_args.algorithm),
        Commands::Blame(blame_args) => Some(&blame_args.algorithm),
        _ => None,
    }
    .map(|algo| match algo {
        DiffAlgorithm::Histogram => gix::diff::blob::Algorithm::Histogram,
        DiffAlgorithm::Myers => gix::diff::blob::Algorithm::Myers,
        DiffAlgorithm::MyersMinimal => gix::diff::blob::Algorithm::MyersMinimal,
    });

    let result_df = match &args.command {
        Commands::Diff(diff_args) => {
            trace!("{:?}", diff_args);
            use binocular_diff::traversal::process;

            let result = process(
                &repo,
                (*diff_args.delegate.commitlist).to_owned(),
                diff_args.threads.unwrap_or(1),
                args.global_opts.skip_merges,
                algo,
                diff_args.breadth_first,
                diff_args.follow,
                args.global_opts.limit,
            );
            result
        }
        Commands::Blame(blame_args) => {
            trace!("{:?}", blame_args);
            use binocular_blame::process_with_lookup;

            let result = process_with_lookup(
                &repo,
                (*blame_args.defines_file).parse().unwrap(),
                algo,
                blame_args.threads.unwrap_or(1),
            );
            result
        }
        Commands::Commits(commit_args) => {
            trace!("{:?}", commit_args);
            use commits::traversal::process;
            let result = process(
                repo,
                (*commit_args.branches).to_owned(),
                args.global_opts.skip_merges,
            );
            result
        }
    };

    match result_df {
        Ok(mut groups) => {
            printer.print_df(&mut groups);
        }
        Err(e) => {
            panic!("{}", e)
        }
    }

    let elapsed = now.elapsed();
    debug!("Elapsed: {:.2?}", elapsed);
}
