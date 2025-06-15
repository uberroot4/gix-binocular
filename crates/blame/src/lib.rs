use log::trace;
#[cfg(feature = "polars")]
use {crate::extensions::polars::ToDataFrameExt, polars::prelude::*};

mod git;
pub(crate) mod objects;

#[cfg(any(feature = "cli", feature = "preprocess"))]
mod preprocess;

mod extensions;

pub mod types {
    pub use crate::objects::blame_result::BlameResult;
    pub use crate::git::objects::BlameOutcome;
}

#[cfg(feature = "preprocess")]
pub mod input {
    use crate::preprocess::input::read_json_content;
    use log::{debug, trace};

    pub fn preprocess(
        defines_file_path: String,
    ) -> anyhow::Result<gix::hashtable::HashMap<gix::ObjectId, Vec<String>>> {
        let _span = gix::trace::coarse!("preprocess({})", defines_file_path);

        let defines = read_json_content(defines_file_path)?;

        let mut tmp: gix::hashtable::HashMap<gix::ObjectId, Vec<String>> =
            gix::hashtable::HashMap::with_capacity_and_hasher(
                defines.len(),
                gix::hashtable::hash::Builder::default(),
            );
        debug!(
            "Initialized Map with {} capacity for {} definitions",
            tmp.capacity(),
            defines.len()
        );
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

        trace!(
            "Stored {} defines in HashMap for {} blame suspects",
            tmp.len(),
            tmp.iter().map(|(_k, v)| v.len()).sum::<usize>()
        );
        for (idx, (k, v)) in tmp.iter().enumerate() {
            trace!("{:4} {} has {} values", idx, k, v.len())
        }
        Ok(tmp)
    }
}

#[cfg(all(feature = "cli", feature = "polars"))]
pub fn process_with_lookup(
    repo: &gix::Repository,
    defines_file_path: String,
    diff_algorithm: Option<gix::diff::blob::Algorithm>,
    max_threads: usize,
) -> anyhow::Result<DataFrame> {
    gix::trace::coarse!("lookup");
    trace!("lookup({:?}, {})", repo, defines_file_path);
    let tmp = crate::input::preprocess(defines_file_path)?;

    let blame_results = process(repo, tmp, diff_algorithm, max_threads)?;

    blame_results.to_df()
}

#[cfg(feature = "polars")]
pub fn process_to_df(
    repo: &gix::Repository,
    defines: gix::hashtable::HashMap<gix::ObjectId, Vec<String>>,
    diff_algorithm: Option<gix::diff::blob::Algorithm>,
    max_threads: usize,
) -> anyhow::Result<DataFrame> {
    crate::process(repo, defines, diff_algorithm, max_threads)?.to_df()
}

pub fn process(
    repo: &gix::Repository,
    defines: gix::hashtable::HashMap<gix::ObjectId, Vec<String>>,
    diff_algorithm: Option<gix::diff::blob::Algorithm>,
    max_threads: usize,
) -> anyhow::Result<Vec<types::BlameResult>> {
    let _span = gix::trace::coarse!(
        "process(repo={:?},#defines={},algo={:?},threads={})",
        repo,
        defines.len(),
        diff_algorithm,
        max_threads
    );
    trace!(
        "process(repo={:?},#defines={},algo={:?},threads={})",
        repo,
        defines.len(),
        diff_algorithm,
        max_threads
    );
    crate::git::blame::process(repo, defines, diff_algorithm, max_threads)
}
