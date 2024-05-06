use crate::git::metrics::{GitDiffMetrics};
use crate::utils;
use anyhow::Result;
use gix::bstr::BString;
use gix::traverse::commit::simple::Sorting;
use gix::{Commit, ObjectId};
use std::cmp::min_by;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use gix::diff::blob::Platform;
use log::{debug, error, info, trace};

pub fn traverse_commit_graph(
    repo: &gix::Repository,
    // no_bots: &Option<Option<MyRegex>>,
    max_threads: usize,
    no_merges: bool,
    diff_algorithm: Option<gix::diff::blob::Algorithm>,
    breadth_first: bool,
    committish: Option<String>,
    limit: Option<usize>,
) -> Result<Vec<GitDiffMetrics>> {
    let mailmap = repo.open_mailmap();
    // let total_number_of_commits = Arc::new(AtomicUsize::default());

    let num_threads = std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(1);
    let threads_used = min_by(max_threads, num_threads, |x, y| x.cmp(&y));
    trace!("threads_used: {:?}", threads_used);
    let commit_graph = repo.commit_graph().ok();
    let can_use_author_threads = num_threads > 1 && commit_graph.is_some();
    trace!("can_use_author_threads: {:?}", can_use_author_threads);
    trace!("commit_graph.is_some(): {:?}", commit_graph.is_some());
    trace!("Algorithm: {:?}", diff_algorithm);

    let sorting = if breadth_first {
        Sorting::BreadthFirst
    } else {
        Sorting::ByCommitTimeNewestFirst
    };

    let commit = repo
        .rev_parse_single({
            committish
                .map(|mut c| {
                    c.push_str("^{commit}");
                    c
                })
                .as_deref()
                .unwrap_or("HEAD")
        })?
        .object()?
        .try_into_commit()?;
    debug!("commit: {:?}", commit.id());

    let commit_iter = commit
        .id()
        .ancestors()
        .sorting(sorting)
        .use_commit_graph(can_use_author_threads)
        .with_commit_graph(commit_graph)
        .all()?;

    let (churn_threads, churn_tx) = get_churn_channel(
        repo,
        threads_used,
        &mailmap,
        diff_algorithm,
    )?;

    let commit_iter_clone = commit
        .id()
        .ancestors()
        .sorting(sorting)
        .use_commit_graph(can_use_author_threads)
        .with_commit_graph(repo.commit_graph().ok())
        .all()?;
    let commits: Vec<_> = commit_iter_clone.collect(); // Collecting all commits into a vector
    trace!("Number of commits: {}", commits.len());
    let limit = limit.unwrap_or(usize::MAX);
    info!("Processing {} commit(s)", limit);

    let mut count = 0;
    for commit in commit_iter.take(limit) {
        let commit = commit?;
        {
            if no_merges && commit.parent_ids.len() > 1 {
                trace!("Skipping Merge Commit {:?}", commit.id);
                continue;
            }

            match churn_tx.send(commit.id) {
                Ok(_) => {}
                Err(e) => {
                    error!("Send Error {:?}", e);
                }
            }
            count += 1;
        }
    }

    // total_number_of_commits.store(count, Ordering::SeqCst);
    debug!("total_number_of_commits: {:?}", count);

    drop(churn_tx);

    // Wait for the threads to complete any remaining work
    let mut diff_results: Vec<GitDiffMetrics> = Vec::new();
    for child in churn_threads {
        let result = child.join().expect("oops! the child thread panicked");
        match result {
            Ok(mut r) => {
                debug!("result.len: {:?}", r.iter().clone().len());
                diff_results.append(&mut r);
            }
            Err(_) => {}
        }
    }

    Ok(diff_results)
}

fn get_churn_channel(
    repo: &gix::Repository,
    max_threads: usize,
    mailmap: &gix::mailmap::Snapshot,
    diff_algorithm: Option<gix::diff::blob::Algorithm>,
) -> Result<(
    Vec<JoinHandle<Result<Vec<GitDiffMetrics>>>>,
    crossbeam_channel::Sender<ObjectId>,
)> {
    let (tx, rx) = crossbeam_channel::bounded::<gix::hash::ObjectId>(8 * 100);
    let repo_sync = repo.clone().into_sync();
    let mut children = Vec::new();

    let mut rewrite_cache = repo
        .diff_resource_cache(gix::diff::blob::pipeline::Mode::ToGit, Default::default())
        .unwrap();
    rewrite_cache
        .options
        .skip_internal_diff_if_external_is_configured = false;
    rewrite_cache.options.algorithm = diff_algorithm;

    for _ in 0..max_threads {
        let child = std::thread::spawn({
            let repo = repo_sync.clone().to_thread_local();
            let rx_clone = rx.clone();
            let mut commits_vec = Vec::new();
            let mut rewrite_cache = rewrite_cache.clone();
            let mailmap = mailmap.clone();
            rewrite_cache.clear_resource_cache();
            move || -> Result<_> {
                debug!("std::thread::spawn: {:?}", std::thread::current().id());
                while let Ok(commit_id) = rx_clone.recv() {
                    let commit = repo.find_object(commit_id)?.into_commit();
                    debug!(
                        "(recv)\t{:?}",
                        commit.id
                    );
                    let mut diff_result = compute_diff_with_parent(
                        //&mut HashMap::new(),
                        &commit,
                        &repo,
                        &mut rewrite_cache,
                    )?;
                    let author = mailmap.resolve(commit.author()?);
                    let committer = mailmap.resolve(commit.committer()?);

                    for mut diff in &mut diff_result {
                        diff.author = Some(author.clone().into());
                        diff.committer = Some(committer.clone().into());
                    }


                    commits_vec.append(diff_result.as_mut())
                }
                Ok(commits_vec)
            }
        });
        children.push(child);
    }

    Ok((children, tx))
}

fn compute_diff_with_parent(
    //change_map: &mut HashMap<BString, usize>,
    commit: &Commit,
    repo: &gix::Repository,
    rewrite_cache: &mut Platform,
) -> Result<Vec<GitDiffMetrics>> {
    let parent_commits: Vec<Commit> = commit.parent_ids()
        .filter(|p| p.object().is_ok())
        .map(|p| p.object().unwrap().into_commit())
        .collect();
    let mut parent_trees : Vec<(Option<&Commit>, gix::Tree)> = parent_commits.iter()
        .map(|parent_commit| {
            let parent_commit_id = parent_commit.id();
            match repo
                .find_object(parent_commit_id)
                .ok()
                .and_then(|c| c.peel_to_tree().ok())
            {
                Some(tree) => {
                    (Some(parent_commit), tree)
                }
                None => panic!("parent_commit could not be found"),
            }
        }).collect();
    if parent_trees.is_empty() {
        debug!("Adding empty tree");
        parent_trees.push((None, repo.empty_tree()));
    }

    trace!("commit {:?}\tparents {:?}", commit, parent_commits);

    let diffs: Vec<GitDiffMetrics> = parent_trees.iter().map(|(parent_commit, parent_tree)| {
        let mut change_map = Default::default();
        change_map = utils::git_helper::calculate_changes(
            &parent_tree,
            &commit.tree().unwrap(),
            rewrite_cache,
            &mut rewrite_cache.clone(),
        );
        GitDiffMetrics::new(
            change_map,
            commit.id,
            match parent_commit {
                Some(pc) => Some(pc.id),
                None => None
            },
            None,
            None
        ).expect("Diff result should be processable")
    }).collect();

    debug!(
        "parents-commit {:?}\t{:?}\t|\t{:?} diffs",
        parent_commits,
        commit.id,
        diffs.len(),
    );
    for d in diffs.iter().clone() {
        debug!(
            "\t{:?}\t{:?}\t|\t{:?} files changed, {:?} insertions(+), {:?} deletions(-)",
            d.parent,
            d.commit,
            d.total_number_of_files_changed,
            d.total_number_of_deletions,
            d.total_number_of_deletions,
        );
    }


    Ok(diffs)
}

