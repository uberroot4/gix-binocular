use gix::bstr::BString;
use std::collections::HashMap;

pub fn calculate_changes(
    previous: &gix::Tree,
    current: &gix::Tree,
    rewrite_cache: &mut gix::diff::blob::Platform,
    diff_cache: &mut gix::diff::blob::Platform,
) -> HashMap<BString, (u32, u32)> {
    let change_map = gitoxide_diff_calculation(previous, current, rewrite_cache, diff_cache);

    change_map
}

fn gitoxide_diff_calculation(
    previous: &gix::Tree<'_>,
    current: &gix::Tree<'_>,
    _rewrite_cache: &mut gix::diff::blob::Platform,
    diff_cache: &mut gix::diff::blob::Platform,
) -> HashMap<BString, (u32, u32)> {
    let mut platform = previous.changes().unwrap();

    let opts_fn = |opts: &mut gix::diff::Options| {
        opts.track_path();
        // opts.track_filename();
    };

    let mut change_map = HashMap::new();

    let _outcome = platform
        .options(opts_fn)
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
                                *change_map
                                    .entry(change.location().into())
                                    .or_insert((u32::MIN, u32::MIN)) =
                                    (counts.insertions, counts.removals);
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
