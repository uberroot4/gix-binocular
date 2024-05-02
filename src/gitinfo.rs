use std::{path::Path, time::Instant};

use gix_test::structs::{Args, DiffAlgorithm};

fn main() {
    let args = <Args as clap::Parser>::parse_from(gix::env::args_os());
    println!("{:?}", args);
    let now = Instant::now();

    match run(args) {
        Ok(()) => {}
        Err(e) => eprintln!("error: {e}"),
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn run(args: Args) -> anyhow::Result<()> {
    let repo = gix::discover(args.git_dir.as_deref().unwrap_or(Path::new(".")))?;

    let algo = match args.algorithm {
        DiffAlgorithm::Histogram => gix::diff::blob::Algorithm::Histogram,
        DiffAlgorithm::Myers => gix::diff::blob::Algorithm::Myers,
        DiffAlgorithm::MyersMinimal => gix::diff::blob::Algorithm::MyersMinimal,
        // None => gix::diff::blob::Algorithm::Histogram,
    };
    println!("algo: {:?}", algo);

    use gix_test::traverse::traverse_commit_graph;
    if let Ok(result) =
        traverse_commit_graph(&repo, args.threads.unwrap_or(1), args.no_merges, Some(algo))
    {
        println!(
            "number_of_commits_by_signature: {:?}",
            result.number_of_commits_by_signature
        );
        println!(
            "number_of_commits_by_file_path: {:?}",
            result.number_of_commits_by_file_path
        );
    } else {
        println!("Failure");
    }

    Ok(())
}
