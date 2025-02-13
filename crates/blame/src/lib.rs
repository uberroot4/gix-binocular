use crate::objects::blame_result::BlameResult;
use log::{debug, trace};

mod git;
pub(crate) mod objects;

#[cfg(feature = "cli")]
mod input;

#[cfg(feature = "cli")]
pub fn lookup(
    repo: &gix::Repository,
    defines_file_path: String,
    diff_algorithm: Option<gix::diff::blob::Algorithm>,
    max_threads: usize,
) -> anyhow::Result<Vec<BlameResult>> {
    trace!("lookup({:?}, {})", repo, defines_file_path);
    let defines = input::read_json_content(defines_file_path)?;

    let mut tmp: gix::hashtable::HashMap<gix::ObjectId, Vec<String>> =
        gix::hashtable::HashMap::with_capacity_and_hasher(
            defines.len(),
            gix::hashtable::hash::Builder::default(),
        );
    debug!("Initialized Map with {} capacity for {} definitions", tmp.capacity(), defines.len());
    assert!(tmp.capacity() >= defines.len());

    for define in defines.iter() {
        trace!(
            "Found {} with {} file entries",
            define.commit,
            define.files.len()
        )
    }

    for define in defines {
        tmp.entry(define.commit)
            .or_default()
            .extend(define.files.iter().cloned())
    }

    trace!("Stored {} defines in HashMap for {} blame suspects", tmp.len(), tmp.iter().map(|(_k,v)| v.len()).sum::<usize>());
    for (idx, (k, v)) in tmp.iter().enumerate() {
        trace!("{:4} {} has {} values", idx, k, v.len())
    }


    let blame_results = process(repo, tmp, diff_algorithm, max_threads)?;

    // let mut carto_obj = CartographyObject::default();
    // for title in vec![
    //     "commit".to_string(),
    //     // "parent".to_string(),
    //     // "files_changed".to_string(),
    //     // "insertions".to_string(),
    //     // "deletions".to_string(),
    //     // "diff_details".to_string(),
    //     "blames".to_string(),
    // ] {
    //     carto_obj.titles.push(title);
    // }
    //
    // let diff_rows = blame_results.into_iter().map(Row::from).collect();
    // carto_obj.groups.push(Group { rows: diff_rows });

    Ok(blame_results)
}

pub fn process(
    repo: &gix::Repository,
    defines: gix::hashtable::HashMap<gix::ObjectId, Vec<String>>,
    diff_algorithm: Option<gix::diff::blob::Algorithm>,
    max_threads: usize,
) -> anyhow::Result<Vec<BlameResult>> {
    trace!(
        "process(repo={:?},#defines={},algo={:?},threads={})",
        repo,
        defines.len(),
        diff_algorithm,
        max_threads
    );
    crate::git::blame::process(repo, defines, diff_algorithm, max_threads)
}
