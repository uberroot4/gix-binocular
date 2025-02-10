use crate::git::metrics::GitBlameMetric;
use anyhow::bail;
use gix;
use gix::bstr::BString;
use log::{debug, trace, warn};
use std::ops::Range;

pub fn traverse_commit_graph<E>(
    odb: &gix::odb::Handle,
    commits: Vec<Result<gix::traverse::commit::Info, E>>,
    //commits: impl IntoIterator<Item = Result<gix::traverse::commit::Info, gix::traverse::commit::topo::Error>>,
    // source_commit: &gix::Commit,
    file_path: &BString,
    mut rewrite_cache: gix::diff::blob::Platform,
    range: Option<Range<u32>>,
) -> anyhow::Result<GitBlameMetric>
where
    E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    let lines_blamed =
        gix_blame::file(&odb, commits, &mut rewrite_cache, file_path.as_ref(), range);

    match lines_blamed {
        Ok(outcome) => {
            // for entry in outcome.entries {
            //     println!("{}\t{}\t{:?}", entry.commit_id, entry.len, entry.range_in_source_file());
            // }
            // debug!("statistics: {:?}", outcome.statistics);
            anyhow::Ok(GitBlameMetric {
                entries: outcome.entries,
                statistics: outcome.statistics,
            })
        }
        Err(e) => {
            warn!("{}", e);
            bail!("{}", e)
        }
    }
}

// fn string_to_commit(
//     repo: &gix::Repository,
//     commit: String,
// ) -> anyhow::Result<gix::Commit<'static>> {
//     let rps = repo
//         .rev_parse_single({ BStr::new(commit.as_bytes()) })?
//         .object()?
//         .try_into_commit()?;
//     Ok(rps)
// }
