use std::collections::HashMap;
use gix::bstr::BString;
// use gix_diff::blob::platform::resource::Data;

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
) -> HashMap<BString, (u32, u32)> {
    let change_map = gitoxide_diff_calculation(
        previous,
        current,
        rewrite_cache,
        diff_cache,
    );

    change_map
}

fn gitoxide_diff_calculation(
    previous: &gix::Tree<'_>,
    current: &gix::Tree<'_>,
    _rewrite_cache: &mut gix::diff::blob::Platform,
    diff_cache: &mut gix::diff::blob::Platform,
) -> HashMap<BString, (u32, u32)> {
    let mut platform = previous.changes().unwrap();

    let _rename_cfg = gix::diff::Rewrites {
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
                if let Ok(cache) = change.diff(diff_cache).map(|p| p.resource_cache) {
                    if let Ok(prep) = cache.prepare_diff() {
                        let tokens = prep.interned_input();
                        match prep.operation {
                            gix::diff::blob::platform::prepare_diff::Operation::InternalDiff {
                                algorithm,
                            } => {
                                let counts = gix::diff::blob::diff(
                                    algorithm,
                                    &tokens,
                                    gix::diff::blob::sink::Counter::default(),
                                );
                                // println!("change {:?}\t|\t{:?}|{:?}:\t{:?}", change.location, counts.insertions, counts.removals, counts.removals + counts.insertions);
                                // println!(" {:?}", change.location);
                                *change_map.entry(change.location.to_owned()).or_insert((
                                    u32::MIN,
                                    u32::MIN,
                                )) = (
                                    counts.insertions,
                                    counts.removals,
                                );
                            }
                            _ => (),
                        }
                    }
                }

                Ok(gix::object::tree::diff::Action::Continue)
            },
        )
        .unwrap();
    // println!("outcome: {:?}", outcome);
    change_map
}