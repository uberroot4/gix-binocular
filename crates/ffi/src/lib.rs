pub(crate) mod types {
    use thiserror::Error;
    use uniffi::Enum;

    pub(crate) mod blame;
    pub(crate) mod branch;
    pub(crate) mod commit;
    pub(crate) mod diff;
    pub(crate) mod repo;
    pub(crate) mod signature;

    type AnyhowError = anyhow::Error;
    // For interfaces, wrap a unit struct with `#[uniffi::remote]`.
    #[uniffi::remote(Object)]
    pub struct AnyhowError;

    #[derive(Debug, Error, Enum)]
    pub enum BinocularError {
        #[error("Invalid input: {0}")]
        InvalidInput(String),

        #[error("Operation failed: {0}")]
        OperationFailed(String),
    }

    type LogLevel = log::Level;

    // Use #[uniffi::remote] to enable support for passing the types across the FFI

    // For records/enums, wrap the item definition with `#[uniffi::remote]`.
    // Copy each field/variant definitions exactly as they appear in the remote crate.
    #[uniffi::remote(Enum)]
    pub enum LogLevel {
        Error = 1,
        Warn = 2,
        Info = 3,
        Debug = 4,
        Trace = 5,
    }
}

pub mod ffi {
    use crate::types::branch::BinocularBranch;
    use crate::types::commit::GixCommit;
    use crate::types::BinocularError;
    use gix::ThreadSafeRepository;
    use std::collections::HashMap;
    use std::ops::Deref;

    #[uniffi::export]
    fn hello() {
        println!("Hello, world!");
    }

    #[uniffi::export]
    fn find_repo(path: String) -> anyhow::Result<ThreadSafeRepository> {
        let repo = match gix::discover(path) {
            Ok(r) => r.into_sync(),
            Err(e) => panic!("{:?}", e),
        };

        println!("Repo found at {:?}", repo.git_dir());
        println!("Repo common_dir {:?}", repo.common_dir);
        println!("Repo work_tree {:?}", repo.work_tree);

        let binding = repo.to_thread_local();
        let references = binding
            .remote_names()
            .into_iter()
            .map(|rm| rm.to_string())
            .collect::<Vec<_>>();

        println!("references {:?}", references);

        Ok(repo)
    }

    #[uniffi::export]
    fn find_commit(repo: &gix::ThreadSafeRepository, hash: String) -> anyhow::Result<GixCommit> {
        println!("repo at {:?}", repo);

        let binding = repo.to_thread_local();
        let commit = binding
            .rev_parse_single(hash.deref())?
            .object()?
            .try_into_commit()?;

        Ok(commit.id)
    }

    #[uniffi::export]
    fn find_all_branches(repo: &gix::ThreadSafeRepository) -> anyhow::Result<Vec<BinocularBranch>> {
        println!("repo at {:?}", repo);

        let binding = repo.to_thread_local();

        let references = binding.references()?;
        let local_branches = references.local_branches()?;
        let remote_branches = references.remote_branches()?;

        Ok(remote_branches
            .chain(local_branches)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .map(BinocularBranch::from)
            .collect())
    }

    #[uniffi::export]
    fn traverse_branch(
        repo: &gix::ThreadSafeRepository,
        // source_commit: GixCommit,
        // target_commit: Option<GixCommit>,
        branch: String,
    ) -> Result<Vec<commits::GitCommitMetric>, BinocularError> {
        let binding = repo.to_thread_local();

        match commits::traversal::main(binding, vec![branch], false) {
            Ok(r) => Ok(r),
            Err(e) => Err(BinocularError::OperationFailed(e.to_string())),
        }

        // let cmts = commits::traversal::main(binding, vec![branch], false)?;
        // Ok(cmts)
    }

    #[uniffi::export]
    fn traverse(
        repo: &gix::ThreadSafeRepository,
        source_commit: GixCommit,
        target_commit: Option<GixCommit>,
    ) -> anyhow::Result<Vec<commits::GitCommitMetric>> {
        let binding = repo.to_thread_local();
        let cmt = binding.find_commit(source_commit)?;
        let trgt = match target_commit {
            None => None,
            Some(c) => Option::from(binding.find_commit(c)?),
        };

        let result = commits::traversal::traverse_from_to(&binding, &cmt, &trgt);

        result
    }

    #[uniffi::export]
    fn diffs(
        repo: &ThreadSafeRepository,
        commitlist: Vec<String>,
        max_threads: u8,
        skip_merges: bool,
        diff_algorithm: Option<crate::types::diff::GixDiffAlgorithm>,
        breadth_first: bool,
        follow: bool,
    ) -> anyhow::Result<Vec<crate::types::diff::BinocularDiffVec>> {
        use binocular_diff::traversal::main;

        let binding = repo.to_thread_local();
        let r = main(
            &binding,
            commitlist,
            max_threads as usize,
            skip_merges,
            diff_algorithm,
            breadth_first,
            follow,
            None,
        );

        let mapped = r?
            .into_iter()
            .map(crate::types::diff::BinocularDiffVec::from)
            .collect();

        Ok(mapped)
    }

    #[uniffi::export]
    fn blames(
        repo: &ThreadSafeRepository,
        defines: HashMap<gix::ObjectId, Vec<String>>,
        diff_algorithm: Option<crate::types::diff::GixDiffAlgorithm>,
        max_threads: u8,
    ) -> anyhow::Result<Vec<crate::types::blame::BinocularBlameResult>> {
        use binocular_blame::process;
        use std::time::Instant;

        let binding = repo.to_thread_local();

        // println!(
        //     "process(repo={:?},#defines={},algo={:?},threads={})",
        //     repo,
        //     defines.len(),
        //     diff_algorithm,
        //     max_threads
        // );

        let mut start = Instant::now();
        let iterable = gix::hashtable::HashMap::from_iter(defines);
        let mut duration = start.elapsed();

        println!("Time elapsed in from_iter() is: {:?}", duration);

        start = Instant::now();
        let result = process(&binding, iterable, diff_algorithm, max_threads as usize);
        duration = start.elapsed();
        println!("Time elapsed in process() is: {:?}", duration);

        // println!("Found {} blames", result?.len());

        Ok(result?
            .into_iter()
            .map(crate::types::blame::BinocularBlameResult::from)
            .collect())
        // Ok(())
    }
}

uniffi::setup_scaffolding!();
