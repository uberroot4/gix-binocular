pub fn get_trees<'a>(
    commit: &'a gix::Commit,
    repo: &'a gix::Repository,
) -> (gix::Tree<'a>, gix::Tree<'a>) {
    let parent_commit = get_first_parent(commit);
    println!("parent_commit: {:?}", parent_commit);
    let from = match parent_commit {
        Some(id) => {
            match repo
                .find_object(id)
                .ok()
                .and_then(|c| c.peel_to_tree().ok())
            {
                Some(tree) => tree,
                None => panic!("parent_commit could not be found"),
            }
        }
        None => repo.empty_tree(),
    };
    let to = match repo
        .find_object(commit.id)
        .ok()
        .and_then(|c| c.peel_to_tree().ok())
    {
        Some(c) => c,
        None => panic!("commit could not be found"),
    };
    (to, from)
}

pub fn get_first_parent(commit: &gix::Commit<'_>) -> Option<gix::ObjectId> {
    let parent_commit = commit
        .parent_ids()
        .next()
        .map(|id| id.object().unwrap().into_commit().id);
    parent_commit
}

pub fn calculate_changes(
    previous: &gix::Tree,
    current: &gix::Tree,
    rewrite_cache: &mut gix::diff::blob::Platform,
    diff_cache: &mut gix::diff::blob::Platform,
) -> (usize, u32, u32) {
    let (mut insertions, mut deletions, mut files_changed) = (0, 0, 0);

    gitoxide_diff_calculation(
        previous,
        current,
        rewrite_cache,
        &mut files_changed,
        diff_cache,
        &mut deletions,
        &mut insertions,
    );

    (files_changed, insertions, deletions)
}

fn gitoxide_diff_calculation(
    previous: &gix::Tree<'_>,
    current: &gix::Tree<'_>,
    rewrite_cache: &mut gix::diff::blob::Platform,
    files_changed: &mut usize,
    diff_cache: &mut gix::diff::blob::Platform,
    deletions: &mut u32,
    insertions: &mut u32,
) {
    let mut platform = previous.changes().unwrap();

    let rename_cfg = gix::diff::Rewrites {
        // copies: Some(gix::diff::rewrites::Copies::default()),
        copies: Some(gix::diff::rewrites::Copies {
            source: gix::diff::rewrites::CopySource::FromSetOfModifiedFiles, // --find-copies-harder
            // source: gix::diff::rewrites::CopySource::default(),
            // //     // source: gix::diff::rewrites::CopySource::default(),
            // percentage: Some(0.9995), // -C9
            percentage: None,
            ..Default::default()
        }),
        percentage: Some(0.5), // -M5
        // limit: 00,
        ..Default::default()
    };
    // println!("rename_cfg: {:?}", rename_cfg);
    let mut change_map = std::collections::HashMap::new();

    let _outcome = platform
        .track_filename()
        .track_path()
        // .track_rewrites(rename_cfg.into())
        .for_each_to_obtain_tree(
            &current,
            // rewrite_cache,
            |change| -> Result<_, gix::object::blob::diff::init::Error> {
                *files_changed += usize::from(change.event.entry_mode().is_no_tree());

                if let Ok(cache) = change.diff(diff_cache).map(|p| p.resource_cache) {
                    if let Ok(prep) = cache.prepare_diff() {
                        let tokens = prep.interned_input();
                        match prep.operation {
                            gix::diff::blob::platform::prepare_diff::Operation::InternalDiff {
                                algorithm,
                            } => {
                                // println!("algorithm: {:?}", algorithm);
                                // let counts = gix::diff::blob::diff(
                                let counts = gix_diff::blob::diff(
                                    algorithm,
                                    &tokens,
                                    gix::diff::blob::sink::Counter::default(),
                                );
                                // println!("change {:?}\t|\t{:?}|{:?}:\t{:?}", change.location, counts.insertions, counts.removals, counts.removals+counts.insertions);
                                // println!(" {:?}", change.location);
                                // source_location
                                *deletions += counts.removals;
                                *insertions += counts.insertions;
                                // println!("insertions, deletions:\t\t{:?}|{:?}", insertions,deletions);
                                *change_map.entry(change.location.to_owned()).or_insert((
                                    usize::from(change.event.entry_mode().is_no_tree()),
                                    u32::MIN,
                                    u32::MIN,
                                )) = (
                                    usize::from(change.event.entry_mode().is_no_tree()),
                                    counts.insertions,
                                    counts.removals,
                                );
                            }
                            _ => (),
                        }
                    }
                }

                // println!("files_changed: {:?}", files_changed);
                Ok(gix::object::tree::diff::Action::Continue)
            }, // for_each_to_obtain_tree
        )
        .unwrap();
    // println!("outcome: {:?}", outcome);

    // println!("actual {:?}", actual.len());
    // println!("change_map {:?}", change_map);
}

// pub fn compute_diff_with_parent(
//     change_map: &mut std::collections::HashMap<gix::bstr::BString, usize>,
//     commit: &gix::Commit,
//     tsrepo: &gix::ThreadSafeRepository,
// ) -> anyhow::Result<()> {
//     use gix::bstr::Utf8Error;
//     use gix::object::tree::diff::change::Event;
//     use gix::object::tree::diff::Action;
//     use gix::prelude::ObjectIdExt;

//     let repo = tsrepo.clone().to_thread_local();

//     let mut rewrite_cache = repo
//         .diff_resource_cache(gix::diff::blob::pipeline::Mode::ToGit, Default::default())
//         .unwrap();
//     rewrite_cache
//         .options
//         .skip_internal_diff_if_external_is_configured = false;
//     rewrite_cache.options.algorithm = Some(gix::diff::blob::Algorithm::MyersMinimal);

//     let mut parents = commit.parent_ids();
//     let parents = (
//         parents
//             .next()
//             .and_then(|parent_id| parent_id.object().ok()?.into_commit().tree_id().ok())
//             .unwrap_or_else(|| gix::hash::ObjectId::empty_tree(repo.object_hash()).attach(&repo)),
//         parents.next(),
//     );
//     println!("parents: {:?}", parents);

//     if let (tree_id, None) = parents {
//         tree_id
//             .object()?
//             .into_tree()
//             .changes()?
//             .track_path()
//             .track_rewrites(None)
//             .for_each_to_obtain_tree(&commit.tree()?, |change| {
//                 let is_file_change = match change.event {
//                     Event::Addition { entry_mode, .. } | Event::Modification { entry_mode, .. } => {
//                         entry_mode.is_blob()
//                     }
//                     Event::Deletion { .. } | Event::Rewrite { .. } => false,
//                 };
//                 if is_file_change {
//                     let path = change.location;
//                     *change_map.entry(path.to_owned()).or_insert(0) += 1;
//                 }
//                 // let diff = if let Some(anyhow::Ok(diff)) = change.diff(rewrite_cache) {
//                 let diff = if let Ok(mut cache) = change.diff(&mut rewrite_cache) {
//                     cache.line_counts().unwrap().unwrap()
//                 } else {
//                     return Ok(Action::Continue);
//                 };
//                 println!(
//                     "change.location\t{:?}\t\tis_file_change: {:?}",
//                     is_file_change, change.location
//                 );
//                 println!("diff: {:?} {:?}", diff.insertions, diff.removals);

//                 Ok::<Action, Utf8Error>(Action::Continue)
//             })?;
//     } /*else if let (first_tree_id, _second_tree_id) = parents {
//         first_tree_id
//             .object()?
//             .into_tree()
//             .changes()?
//             .track_path()
//             .track_rewrites(None)
//             .for_each_to_obtain_tree(&commit.tree()?, |change| {
//                 let is_file_change = match change.event {
//                     Event::Addition { entry_mode, .. } | Event::Modification { entry_mode, .. } => {
//                         entry_mode.is_blob()
//                     }
//                     Event::Deletion { .. } | Event::Rewrite { .. } => false,
//                 };
//                 if is_file_change {
//                     let path = change.location;
//                     *change_map.entry(path.to_owned()).or_insert(0) += 1;
//                 }
//                 // let diff = if let Some(anyhow::Ok(diff)) = change.diff(rewrite_cache) {
//                 let diff = if let Ok(mut cache) = change.diff(&mut rewrite_cache) {
//                     cache.line_counts().unwrap().unwrap()
//                 } else {
//                     return Ok(Action::Continue);
//                 };
//                 println!(
//                     "change.location\t{:?}\t\tis_file_change: {:?}",
//                     is_file_change, change.location
//                 );
//                 println!("diff: {:?} {:?}", diff.insertions, diff.removals);

//                 Ok::<Action, Utf8Error>(Action::Continue)
//             })?;
//     }*/ else {
//         println!("No tree")
//     }

//     Ok(())
// }
