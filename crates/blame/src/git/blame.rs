use crate::git::objects::BlameOutcome;
use crate::objects::blame_result::{BlameResult, BlameResultVec};
use anyhow::bail;
use gix::bstr::BStr;
use gix::traverse::commit::Info;
use log::{debug, error, trace, warn};
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;
use polars::frame::DataFrame;
use tqdm::tqdm;
use shared::VecDataFrameExt;

fn retrieve_blame<E>(
    odb: &gix::odb::Handle,
    commits: Vec<Result<gix::traverse::commit::Info, E>>,
    file_path: &String,
    mut rewrite_cache: gix::diff::blob::Platform,
    range: Option<Range<u32>>,
) -> anyhow::Result<BlameOutcome>
where
    E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    let lines_blamed = gix::blame::file(
        &odb,
        commits,
        &mut rewrite_cache,
        BStr::new(file_path.as_bytes()),
        range,
    );

    match lines_blamed {
        Ok(outcome) => anyhow::Ok(BlameOutcome {
            entries: outcome.entries,
            statistics: outcome.statistics,
            file_path: file_path.to_owned(),
        }),
        Err(e) => {
            bail!("{}", e);
        }
    }
}

type CommitFilenameTuple = (gix::ObjectId, String);
#[derive(Debug)]
struct BlameOperationResult {
    blame: BlameOutcome,
    commit_oid: gix::ObjectId,
}

pub(crate) fn process(
    repo: &gix::Repository,
    diff_results: gix::hashtable::HashMap<gix::ObjectId, Vec<String>>,
    diff_algorithm: Option<gix::diff::blob::Algorithm>,
    max_threads: usize,
) -> anyhow::Result<DataFrame> {
    let odb_handle = &repo.objects;
    let mut rewrite_cache =
        repo.diff_resource_cache(gix::diff::blob::pipeline::Mode::ToGit, Default::default())?;
    rewrite_cache
        .options
        .skip_internal_diff_if_external_is_configured = false;
    rewrite_cache.options.algorithm = diff_algorithm;
    let mut children = Vec::with_capacity(max_threads);

    let diffs_to_process: Vec<CommitFilenameTuple> = diff_results
        // .par_iter()
        .iter()
        .map(|(k, v)| (k.to_owned(), v))
        .map(|(commit_id, files)| {
            files
                .into_iter()
                .map(|filename| {
                    // (gix::ObjectId::from_str(commit_id), filename.to_owned())
                    (commit_id, filename.to_owned())
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    let num_diffs_to_process = diffs_to_process.iter().count();
    let num_diffs = diff_results.iter().count();
    trace!(
        "{} diffs to process for {} commits",
        num_diffs_to_process,
        num_diffs
    );

    let mut lru: gix::hashtable::HashMap<gix::ObjectId, Vec<Info>> =
        gix::hashtable::HashMap::with_capacity_and_hasher(
            num_diffs,
            gix::hashtable::hash::Builder::default(),
        );

    debug!(
        "Channel size: {}/{}={}",
        num_diffs_to_process,
        max_threads,
        num_diffs_to_process / max_threads
    );
    let (tx_main, rx_thread) =
        crossbeam_channel::bounded::<CommitFilenameTuple>(num_diffs_to_process / max_threads);
    let (tx_thread, rx_main) =
        crossbeam_channel::bounded::<BlameOperationResult>(num_diffs_to_process);

    // Fill LRU cache
    for (commit, _) in diff_results {
        let val: Vec<_> =
            crate::git::commits::commits_topo(&odb_handle, &commit, repo.commit_graph().ok())
                // .into_par_iter()
                .into_iter()
                .filter(Result::is_ok)
                .map(Result::unwrap)
                .collect();
        trace!("commits_topo({}) returned {} values", commit, val.len());
        assert_eq!(None, lru.insert(commit, val));
    }

    debug!("Commit-Cache Size: {}", lru.len());
    assert_eq!(num_diffs, lru.len());
    let storage = Arc::new(lru.clone());

    for _t in 0..max_threads {
        let child = std::thread::spawn({
            // let ts_repo = repo_sync.clone().to_thread_local();
            let rx_thread_clone = rx_thread.clone();
            let tx_thread_clone = tx_thread.clone();
            let lru = Arc::clone(&storage);
            let odb_handle = odb_handle.clone();
            let mut rewrite_cache = rewrite_cache.clone();
            move || -> anyhow::Result<_> {
                while let Ok((commit_oid, file_path)) = rx_thread_clone.recv() {
                    trace!(
                        "Processing blame for {} and {}",
                        commit_oid.to_string(),
                        file_path
                    );

                    // let commit_obj = ts_repo.find_commit(commit_oid)?;
                    let commits_topo_list = lru.get(&commit_oid).unwrap();
                    trace!(
                        "Cache has {} values for {}",
                        commits_topo_list.len(),
                        commit_oid
                    );

                    match self::retrieve_blame::<anyhow::Error>(
                        &odb_handle.clone(),
                        commits_topo_list.iter().map(|v| Ok(v.clone())).collect(),
                        &file_path,
                        rewrite_cache.clone(),
                        None,
                    ) {
                        Ok(blame_result) => {
                            match tx_thread_clone.send(BlameOperationResult {
                                blame: blame_result,
                                commit_oid,
                            }) {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("Send Error {:?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            error!("BlameResult Error: {}", e)
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

    for val in tqdm(diffs_to_process).desc(Some("Sending blame suspects")) {
        let commit_oid = val.0;
        let file_path = val.1;
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

    let receiver_child = std::thread::spawn({
        move || -> Vec<BlameOperationResult> {
            let mut blame_results =
                Vec::<BlameOperationResult>::with_capacity(num_diffs_to_process);
            let mut pbar = tqdm::pbar(Some(num_diffs_to_process));
            while let Ok(br) = rx_main.recv() {
                trace!(
                    "Received blame operation result {:?} {:?}",
                    br.commit_oid,
                    br.blame.file_path
                );
                pbar.update(1).unwrap();
                blame_results.push(br);
            }
            drop(rx_main);
            trace!("Received {} blame results", blame_results.iter().count());
            blame_results
        }
    });

    for child in children {
        match child.join() {
            Ok(_) => {}
            Err(e) => {
                error!("{:?}", e)
            }
        }
    }
    trace!("All child-threads finished");

    let blame_results = match receiver_child.join() {
        Ok(r) => {
            debug!("Received {} blame results (2)", r.iter().count());
            r
        }
        Err(e) => {
            panic!("Error receiving in main-thread: {:?}", e);
        }
    };

    let groups = group_blame_operations(blame_results);
    debug!("Merged {} blame results", groups.iter().count());
    if groups.iter().count() != num_diffs {
        warn!(
            "Some commits are missing in the final blame result: {} vs. expected {} \n\
        Should be fixed in the future as deleted files should simply not be blamed\
        ",
            groups.iter().count(),
            num_diffs
        )
    }

    let vectorized = BlameResultVec(groups);
    let lf = vectorized.to_df()?;

    Ok(lf)
}

fn group_blame_operations(operations: Vec<BlameOperationResult>) -> Vec<BlameResult> {
    // Create a HashMap to group GitBlameMetric values by commit_oid.
    // let mut groups: gix::hashtable::HashMap<gix::ObjectId, HashMap<BString, Vec<GitBlameMetric>>> =
    let mut groups: gix::hashtable::HashMap<gix::ObjectId, Vec<BlameOutcome>> =
        gix::hashtable::HashMap::with_capacity_and_hasher(
            operations.iter().count(),
            gix::hashtable::hash::Builder::default(),
        );

    debug!("Processing {} operation results", operations.iter().count());
    // Iterate over all operations and push each blame into the appropriate group.
    for op in operations {
        groups
            .entry(op.commit_oid)
            .or_insert_with(Vec::new)
            .push(op.blame);
    }

    // Convert the HashMap into a Vec<BlameResult>
    groups
        .into_iter()
        .map(|(commit_oid, tuple)| BlameResult {
            blames: tuple,
            commit_oid,
        })
        .collect()
}
