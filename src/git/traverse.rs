use crate::git::metrics::GitMetrics;
use crate::git::sig::Sig;
use crate::utils;
use anyhow::Result;
use gix::bstr::BString;
use gix::prelude::ObjectIdExt;
use gix::traverse::commit::Sorting;
use gix::{Commit, ObjectId};
use std::cmp::min_by;
// use regex::Regex;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

pub fn traverse_commit_graph(
    repo: &gix::Repository,
    // no_bots: &Option<Option<MyRegex>>,
    max_threads: usize,
    no_merges: bool,
    diff_algorithm: Option<gix::diff::blob::Algorithm>,
) -> Result<GitMetrics> {
    let mut time_of_most_recent_commit = None;
    let mut time_of_first_commit = None;
    // let mut number_of_commits_by_signature: HashMap<Sig, usize> = HashMap::new();
    let mailmap = repo.open_mailmap();
    // let bot_regex_pattern = get_no_bots_regex(no_bots)?;
    let has_commit_graph_traversal_ended = Arc::new(AtomicBool::default());
    let total_number_of_commits = Arc::new(AtomicUsize::default());

    let num_threads = std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(1);
    let threads_used = min_by(max_threads, num_threads, |x, y| x.cmp(&y));
    println!("threads_used: {:?}", threads_used);
    let commit_graph = repo.commit_graph().ok();
    let can_use_author_threads = num_threads > 1 && commit_graph.is_some();
    println!("can_use_author_threads: {:?}", can_use_author_threads);
    println!("commit_graph.is_some(): {:?}", commit_graph.is_some());
    println!("num_threads: {:?}", num_threads);
    println!("Algorithm: {:?}", diff_algorithm);

    let commit_iter = repo
        .head_commit()?
        .id()
        .ancestors()
        .sorting(Sorting::ByCommitTimeNewestFirst)
        .use_commit_graph(can_use_author_threads)
        .with_commit_graph(commit_graph)
        .all()?;

    let (churn_threads, churn_tx) = get_churn_channel(
        repo,
        &has_commit_graph_traversal_ended,
        threads_used,
        &mailmap,
        diff_algorithm
    )?;

    let commit_iter_clone = repo
        .head_commit()?
        .id()
        .ancestors()
        .sorting(Sorting::ByCommitTimeNewestFirst)
        .use_commit_graph(can_use_author_threads)
        .with_commit_graph(repo.commit_graph().ok())
        .all()?;
    let commits: Vec<_> = commit_iter_clone.collect(); // Collecting all commits into a vector
    println!("Number of commits: {}", commits.len());

    let mut count = 0;
    for commit in commit_iter {
        let commit = commit?;
        {
            if no_merges && commit.parent_ids.len() > 1 {
                println!("Skipping Merge Commit {:?}", commit.id);
                continue;
            }

            match churn_tx.send(commit.id) {
                Ok(_) => {},
                Err(e) => {
                    println!("Send Error {:?}", e);
                } 
            }

            let commit_time = gix::date::Time::new(
                commit
                    .commit_time
                    .expect("sorting by time yields this field as part of traversal"),
                0,
            );
            time_of_most_recent_commit.get_or_insert(commit_time);
            time_of_first_commit = commit_time.into();

            count += 1;
        }
    }

    total_number_of_commits.store(count, Ordering::SeqCst);
    has_commit_graph_traversal_ended.store(true, Ordering::SeqCst);
    println!("total_number_of_commits: {:?}", total_number_of_commits);

    drop(churn_tx);

    // Wait for the threads to complete any remaining work
    for child in churn_threads {
        let result = child.join().expect("oops! the child thread panicked");
        match result {
            Ok(r) => println!("result.len: {:?}", r.len()),
            Err(_) => {}
        }
    }

    let git_metrics = GitMetrics::new(
        HashMap::new(), //number_of_commits_by_signature,
        HashMap::new(), // number_of_commits_by_file_path,
        0,              //churn_pool_size,
        time_of_first_commit,
        time_of_most_recent_commit,
    )?;

    Ok(git_metrics)
}

type NumberOfCommitsBySignature = HashMap<Sig, usize>;

type NumberOfCommitsByFilepath = HashMap<BString, usize>;
type ChurnPair = (NumberOfCommitsByFilepath, usize);

fn get_churn_channel(
    repo: &gix::Repository,
    has_commit_graph_traversal_ended: &Arc<AtomicBool>,
    max_threads: usize,
    mailmap: &gix::mailmap::Snapshot,
    diff_algorithm: Option<gix::diff::blob::Algorithm>,
) -> Result<(
    Vec<JoinHandle<Result<Vec<ObjectId>>>>,
    crossbeam_channel::Sender<ObjectId>,
)> {
    let (tx, rx) = crossbeam_channel::bounded::<gix::hash::ObjectId>(8 * 100);
    println!(
        "(get_churn_channel) thread_id: {:?}",
        std::thread::current().id()
    );
    let repo_sync = repo.clone().into_sync();
    let mut children = Vec::new();

    let mut rewrite_cache = repo
        .diff_resource_cache(gix::diff::blob::pipeline::Mode::ToGit, Default::default())
        .unwrap();
    rewrite_cache
        .options
        .skip_internal_diff_if_external_is_configured = false;
    // rewrite_cache.options.algorithm = Some(gix::diff::blob::Algorithm::Histogram);
    rewrite_cache.options.algorithm = diff_algorithm;

    for _ in 0..max_threads {
        let child = std::thread::spawn({
            let repo = repo_sync.clone().to_thread_local();
            let rx_clone = rx.clone();
            let mut commits_vec = Vec::new();
            let has_commit_graph_traversal_ended = has_commit_graph_traversal_ended.clone();
            let mut rewrite_cache = rewrite_cache.clone();
            let mailmap = mailmap.clone();
            rewrite_cache.clear_resource_cache();
            move || -> Result<_> {
                println!("std::thread::spawn: {:?}", std::thread::current().id());
                while let Ok(commit_id) = rx_clone.recv() {
                    let commit = repo.find_object(commit_id)?.into_commit();
                    println!(
                        "(recv) thread_id: {:?}\t{:?}",
                        std::thread::current().id(),
                        commit.id
                    );
                    compute_diff_with_parent(
                        &mut HashMap::new(),
                        &commit,
                        &repo,
                        &mut rewrite_cache,
                    )?;
                    let author = mailmap.resolve(commit.author()?);
                    let committer = mailmap.resolve(commit.committer()?);

                    // stop threads here if ended
                    // if has_commit_graph_traversal_ended.load(Ordering::Relaxed) {
                    //     break;
                    // }

                    println!("Done for {:?}", commit.id);

                    commits_vec.push(commit.id);
                }
                Ok(commits_vec)
            }
        });
        children.push(child);
    }

    Ok((children, tx))
}

fn compute_diff_with_parent(
    change_map: &mut HashMap<BString, usize>,
    commit: &Commit,
    repo: &gix::Repository,
    rewrite_cache: &mut gix_diff::blob::Platform,
) -> Result<()> {
    let mut parents = commit.parent_ids();
    let parents: (gix::Id<'_>, Option<gix::Id<'_>>) = (
        parents
            .next()
            .and_then(|parent_id| parent_id.object().ok()?.into_commit().tree_id().ok())
            .unwrap_or_else(|| gix::hash::ObjectId::empty_tree(repo.object_hash()).attach(repo)),
        parents.next(),
    );
    // let (to, from) = utils::git_helper::get_trees(commit, repo);

    println!("commit {:?}", commit);
    println!(
        "parents {:?}",
        commit.parent_ids().collect::<Vec<gix::Id>>()
    );
    println!(
        "compute_diff_with_parent thread_id: {:?}",
        std::thread::current().id()
    );

    let (mut insertions, mut deletions, mut files_changed) = (0, 0, 0);

    if let (tree_id, None) = parents {
        (insertions, deletions, files_changed) = utils::git_helper::calculate_changes(
            &tree_id.object().unwrap().into_tree(),
            &commit.tree().unwrap(),
            rewrite_cache,
            &mut rewrite_cache.clone(),
        );
    }
    println!(
        "parents-commit {:?}\t{:?}\t|\t{:?} files changed, {:?} insertions(+), {:?} deletions(-)",
        commit.parent_ids().collect::<Vec<gix::Id>>(),
        commit.id,
        files_changed,
        insertions,
        deletions
    );
    // println!("parents {:?}", commit.parent_ids().collect::<Vec<gix::Id>>());

    Ok(())
}

// fn get_no_bots_regex(no_bots: &Option<Option<MyRegex>>) -> Result<Option<MyRegex>> {
//     let reg = if let Some(r) = no_bots.clone() {
//         match r {
//             Some(p) => Some(p),
//             None => Some(MyRegex(Regex::from_str(r"(?:-|\s)[Bb]ot$|\[[Bb]ot\]")?)),
//         }
//     } else {
//         None
//     };

//     Ok(reg)
// }

fn is_bot(author_name: &BString /*, bot_regex_pattern: &Option<MyRegex>*/) -> bool {
    // bot_regex_pattern.as_ref().map_or(false, |regex| {
    //     regex.0.is_match(author_name.to_str_lossy().as_ref())
    // })
    return false;
}
