use crate::git;
use crate::metrics::GitDiffMetrics;
use blame::GitBlameMetric;
use gix::bstr::BString;
use gix::traverse::commit::Info;
use log::{debug, error, trace, warn};
use rayon::prelude::*;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;
use tqdm::tqdm;

type ObjectFileTuple = (gix::ObjectId, BString);

pub fn process(
    repo: &gix::Repository,
    diff_results: Vec<git::metrics::GitDiffMetrics>,
    diff_algorithm: Option<gix::diff::blob::Algorithm>,
    max_threads: usize,
) -> anyhow::Result<Vec<GitDiffMetrics>> {
    let worktree_path = PathBuf::from(repo.work_dir().unwrap());
    let odb_handle = gix::odb::at(worktree_path.join(".git/objects"))?;
    let mut rewrite_cache =
        repo.diff_resource_cache(gix::diff::blob::pipeline::Mode::ToGit, Default::default())?;
    rewrite_cache
        .options
        .skip_internal_diff_if_external_is_configured = false;
    rewrite_cache.options.algorithm = diff_algorithm;
    let repo_sync = repo.clone().into_sync();
    let mut children = Vec::with_capacity(max_threads);

    let diffs_to_process: Vec<ObjectFileTuple> = diff_results
        // .par_iter()
        .iter()
        .map(|metric| {
            metric
                .change_map
                .to_owned()
                .keys()
                .map(|a| a.to_owned())
                .map(|b| (metric.commit, b))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    let num_diffs_to_process = diffs_to_process.iter().count();
    trace!("{} diffs to process", num_diffs_to_process);

    let mut lru: gix::hashtable::HashMap<gix::ObjectId, Vec<Info>> =
        gix::hashtable::HashMap::with_capacity_and_hasher(
            diff_results.iter().count(),
            gix::hashtable::hash::Builder::default(),
        );

    debug!(
        "Channel size: {}/{}={}",
        num_diffs_to_process,
        max_threads,
        num_diffs_to_process / max_threads
    );
    let (tx_main, rx_thread) =
        crossbeam_channel::bounded::<ObjectFileTuple>(num_diffs_to_process / max_threads);
    let (tx_thread, rx_main) =
        crossbeam_channel::bounded::<GitBlameMetric>(num_diffs_to_process / max_threads);

    // Fill LRU cache
    let _: Vec<_> = diff_results
        // .par_iter()
        .iter()
        .map(|metric| {
            let commit = metric.commit;
            let val: Vec<_> = crate::utils::blame_info::commits_topo(
                &odb_handle,
                &commit,
                repo.commit_graph().ok(),
            )
            // .into_par_iter()
            .into_iter()
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();
            (commit, val)
        })
        .map(|val| lru.insert(val.0, val.1))
        .collect();

    let storage = Arc::new(lru.clone());

    for _t in 0..max_threads {
        let child = std::thread::spawn({
            let ts_repo = repo_sync.clone().to_thread_local();
            let rx_thread_clone = rx_thread.clone();
            let tx_thread_clone = tx_thread.clone();
            let lru = Arc::clone(&storage);
            let odb_handle = odb_handle.clone();
            let mut rewrite_cache = rewrite_cache.clone();
            move || -> anyhow::Result<_> {
                while let Ok(tuple) = rx_thread_clone.recv() {
                    let commit_oid = tuple.0;
                    let file_path = tuple.1;

                    // let commit_obj = ts_repo.find_commit(commit_oid)?;
                    let commits_topo_list = lru.get(&commit_oid).unwrap();

                    if let Ok(blame_result) = process_blame_threaded::<anyhow::Error>(
                        &odb_handle.clone(),
                        commits_topo_list.iter().map(|v| Ok(v.clone())).collect(),
                        // &commit_obj,
                        &file_path,
                        rewrite_cache.clone(),
                        None,
                    ) {
                        match tx_thread_clone.send(blame_result) {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Send Error {:?}", e);
                            }
                        }
                    }

                    rewrite_cache.clear_resource_cache();
                }
                drop(rx_thread_clone);
                drop(tx_thread_clone);
                Ok(())
            }
        });
        children.push(child);
    }
    drop(rx_thread);
    drop(tx_thread);

    let receiver_child = std::thread::spawn({
        let rx_main_clone = rx_main.clone();
        move || -> Vec<GitBlameMetric> {
            let mut blame_results = Vec::<GitBlameMetric>::with_capacity(num_diffs_to_process);
            while let Ok(br) = rx_main_clone.recv() {
                blame_results.push(br);
            }
            trace!("Received {} blame results", blame_results.iter().count());
            blame_results
        }
    });

    for val in tqdm(diffs_to_process) {
        let commit_oid = val.0;
        let file_path = val.1;
        if !lru.contains_key(&commit_oid) {
            warn!("Cache miss for {:?}", commit_oid);
            eprintln!("Cache key '{}' not present", commit_oid);
            panic!("Cache key not present")
        }
        assert!(
            lru.contains_key(&commit_oid),
            "Cache key {} mut be present",
            commit_oid
        );
        match tx_main.send((commit_oid, file_path)) {
            Ok(_) => {}
            Err(e) => {
                error!("Send Error {:?}", e);
            }
        }
    }

    drop(tx_main);
    for child in children {
        // let result = ;
        match child.join().expect("oops! the child thread panicked") {
            Ok(_) => {}
            Err(e) => {
                error!("{}", e)
            }
        }
    }

    trace!("All child-threads finished");
    drop(rx_main);

    match receiver_child.join() {
        Ok(r) => {
            trace!("Received {} blame results (2)", r.iter().count());
        }
        Err(e) => {
            panic!("Error receiving in main-thread: {:?}", e);
        }
    }

    Ok(vec![])
}

fn process_blame_threaded<E>(
    odb: &gix::odb::Handle,
    commits: Vec<Result<gix::traverse::commit::Info, E>>,
    // source_commit: &gix::Commit,
    file_path: &BString,
    mut rewrite_cache: gix::diff::blob::Platform,
    range: Option<Range<u32>>,
) -> anyhow::Result<GitBlameMetric>
where
    E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    blame::lookup(odb, commits/*, source_commit*/, file_path, rewrite_cache, range)
}
