use std::{
    path::Path,
    time::Instant,
};

use gix_test::git_helper::{get_trees,get_first_parent, calculate_changes};

use anyhow::Ok;
/// A toy-version of `git log`.
use clap::Parser;
use gix::{
    bstr::BString,
    traverse::commit::Sorting,
};

use gix_test::structs::Args;

#[derive(Default)]
struct Adder {
    commit_id: String,
}

#[derive(Default)]
struct LogEntryInfo {
    commit_id: String,
    parents: Vec<String>,
    author: BString,
    time: String,
    message: BString,
}

// impl gix::parallel::Reduce for Adder {
//     type Input = String;
//     type FeedProduce = String;
//     type Output = String;
//     type Error = ();

//     fn feed(&mut self, item: Self::Input) -> Result<Self::FeedProduce, Self::Error> {
//         // self.count += item;
//         Ok(item)
//     }

//     fn finalize(self) -> Result<Self::Output, Self::Error> {
//         Ok(self.commit_id)
//     }
// }

fn main() {
    let args = Args::parse_from(gix::env::args_os());
    println!("{:?}", args);
    let now = Instant::now();
    // for argument in gix::env::args_os() {
    //     println!("{argument:?}");
    // }
    match run(args) {
        std::result::Result::Ok(()) => {}
        Err(e) => eprintln!("error: {e}"),
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn run(args: Args) -> anyhow::Result<()> {
    let repo = gix::discover(args.git_dir.as_deref().unwrap_or(Path::new(".")))?;

    let mut rewrite_cache = repo
        .diff_resource_cache(gix::diff::blob::pipeline::Mode::ToGit, Default::default())
        .unwrap();
    rewrite_cache
        .options
        .skip_internal_diff_if_external_is_configured = false;
    rewrite_cache.options.algorithm = Some(gix::diff::blob::Algorithm::MyersMinimal);
    let mut diff_cache = rewrite_cache.clone();

    let child_commit = repo
        .rev_parse_single("4969b1eb178b23b3b67c5bddf6f7ff3eb91e1d74")?
        .object()?
        .try_into_commit()?;
    let parent_commit = repo
        .rev_parse_single("b34ddbe")?
        .object()?
        .try_into_commit()?;

    // let db = gix_odb::at(repo.common_dir().join("objects")).unwrap();
    // let mut buf = Vec::new();

    let first_parent_oid = get_first_parent(&child_commit).unwrap();
    let (current, first_parent) = get_trees(&child_commit, &repo);
    println!("current, first_parent: {:?} {:?} {:?} {:?}", child_commit, parent_commit, first_parent_oid, parent_commit.id == first_parent_oid);
    let (files_changed, insertions, deletions) =
        calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
    println!(
        "calculate_changes: {:?}, {:?}, {:?} (fc,i,d)",
        files_changed, insertions, deletions
    );

    let commit = repo
        .rev_parse_single({
            args.committish
                .map(|mut c| {
                    c.push_str("^{commit}");
                    c
                })
                .as_deref()
                .unwrap_or("HEAD")
        })?
        .object()?
        .try_into_commit()?;

    let sorting = if args.breadth_first {
        Sorting::BreadthFirst
    } else {
        // else if args.newest_first {
        Sorting::ByCommitTimeNewestFirst
    };

    // let input = Box::new(
    //     repo.rev_walk([commit.id]).sorting(sorting).all()?
    // .map(|info| {
    //     match info {
    //         std::result::Result::Ok(i) => {
    //             let commit = i.object()?;
    //             let commit_str = commit.id().to_hex().to_string();
    //             Ok(commit_str) // Ensure to return Ok variant of Result
    //         },
    //         Err(e) => Ok(Err(e)?), // Properly pass through errors
    //     }
    // }));

    // for i in input {
    //     match i {
    //         std::result::Result::Ok(commit_str) => println!("Commit: {:?}", commit_str),
    //         Err(e) => println!("Error processing commit: {:?}", e),
    //     }
    // }

    // gix::parallel::in_parallel(input, Some(1), |_n| (), |a, _state| a, Adder::default());

    // repo.into_sync().to_thread_local().rev
    // match db.find_commit(&commit.id, &mut buf) {
    //     Ok(c) => {
    //         println!("c {:?}", c);
    //     },
    //     Err(_) => {}
    // }
    // .ok
    // .ok_or_else(|| format!("start commit {commit:?} to be present")).unwrap()
    // .0
    // .decode().unwrap()
    // .into_commit();

    let mut min_parents = args.min_parents.unwrap_or(0);
    let mut max_parents = args.max_parents.unwrap_or(usize::MAX);
    if args.merges {
        min_parents = 2;
    }
    if args.no_merges {
        max_parents = 1;
    }

    // gix::commitgraph::Graph::new

    // let w = repo.into_sync().to_thread_local().rev_walk(tips)

    // let eagerIter = gix::parallel::EagerIter::new(w, 10, 5);

    // let res = gix::parallel::in_parallel(gix::parallel::InOrderIter::from(w), Some(1), |_n| (), |input, _state| input, Adder::default());
    // gix::parallel::reduce::Stepwise::new(log_iter, Some(1), |_n| (),|input, _state| input, reducer);
    // ;

    // let mut log_iter: Box<dyn Iterator<Item = Result<LogEntryInfo, _>>> = Box::new(
    //     repo.rev_walk([commit.id])
    //         .sorting(sorting)
    //         .all()?
    //         .filter(|info| {
    //             info.as_ref().map_or(true, |info| {
    //                 info.parent_ids.len() <= max_parents &&
    //                 info.parent_ids.len() >= min_parents &&
    //                 // if the list of paths is empty the filter passes.
    //                 // if paths are provided check that any one of them are
    //                 // in fact relevant for the current commit.
    //                 (args.paths.is_empty() || args.paths.iter().any(|path| {
    //                     // TODO: should make use of the `git2::DiffOptions`
    //                     //       counterpart in gix for a set of files and also to
    //                     //       generate diffs. When ready, also make paths resistant
    //                     //       to illformed UTF8 by not using ".display()".
    //                     // PERFORMANCE WARNING: What follows is a clever implementation
    //                     //    that is also **very** slow - do not use on bigger sample
    //                     //    repositories as this needs native support in `gix` to
    //                     //    be fast enough.
    //                     match repo.rev_parse_single(
    //                         format!("{}:{}", info.id, path.display()).as_str()
    //                     ) {
    //                         // check by parsing the revspec on the path with
    //                         // the prefix of the tree of the current commit,
    //                         // vs. the same counterpart but using each of
    //                         // commit's parents; if any pairs don't match,
    //                         // this indicates this path was changed in this
    //                         // commit thus should be included in output.
    //                         // naturally, root commits have no parents and
    //                         // by definition whatever paths in there must
    //                         // have been introduced there, so include them.
    //                         Ok(oid) => info.parent_ids.is_empty() || info
    //                             .parent_ids
    //                             .iter()
    //                             .any(|id| {
    //                                 repo.rev_parse_single(
    //                                     format!("{id}:{}", path.display()).as_str()
    //                                 ).ok() != Some(oid)
    //                             }),
    //                         // no oid for the path resolved with this commit
    //                         // so this commit can be omitted from output.
    //                         Err(_) => false,
    //                     }
    //                 }))
    //             })
    //         })
    //         .map(|info| -> anyhow::Result<_> {
    //             let info = info?;
    //             let commit = info.object()?;
    //             let commit_ref = commit.decode()?;
    //             Ok(LogEntryInfo {
    //                 commit_id: commit.id().to_hex().to_string(),
    //                 parents: info.parent_ids().map(|id| id.shorten_or_id().to_string()).collect(),
    //                 author: {
    //                     let mut buf = Vec::new();
    //                     commit_ref.author.actor().write_to(&mut buf)?;
    //                     buf.into()
    //                 },
    //                 time: commit_ref.author.time.format(format::DEFAULT),
    //                 message: commit_ref.message.to_owned(),
    //             })
    //         }),
    // );
    // if args.reverse {
    //     let mut results: Vec<_> = log_iter.collect();
    //     results.reverse();
    //     log_iter = Box::new(results.into_iter())
    // }

    // let mut log_iter = log_iter
    //     .skip(args.skip.unwrap_or_default())
    //     .take(args.count.unwrap_or(usize::MAX))
    //     .peekable();

    // let mut out = stdout().lock();
    // let mut buf = Vec::new();

    // while let Some(entry) = log_iter.next() {
    //     buf.clear();
    //     let entry = entry?;
    //     writeln!(buf, "commit {}", entry.commit_id)?;
    //     if entry.parents.len() > 1 {
    //         writeln!(buf, "Merge: {}", entry.parents.join(" "))?;
    //     }
    //     writeln!(buf, "Author: {}", entry.author)?;
    //     writeln!(buf, "Date:   {}\n", entry.time)?;
    //     for line in entry.message.lines() {
    //         write!(buf, "    ")?;
    //         buf.write_all(line)?;
    //         writeln!(buf)?;
    //     }
    //     // only include newline if more log entries, mimicking `git log`
    //     if log_iter.peek().is_some() {
    //         writeln!(buf)?;
    //     }
    //     out.write_all(&buf)?;
    // }

    Ok(())
}
