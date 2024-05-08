use gix::Reference;
use log::{debug, trace};
use crate::git::metrics::GitCommitMetric;

pub fn traverse_commit_graph(
    repo: gix::Repository,
    branches: Vec<String>,
) -> anyhow::Result<Vec<GitCommitMetric>> {
    let prefixed_branches: Vec<String> = branches.iter().map(|b|
        if b.contains("origin/") {
            format!("refs/remotes/{b}")
        } else {
            format!("refs/heads/{b}")
        }
    ).collect();
    debug!("prefixed_branches ({:?}): {:?}", prefixed_branches.len(), prefixed_branches);
    let references = repo.references()?;
    let local_branches = references.local_branches()?;
    let remote_branches = references.remote_branches()?;
    let local_and_remote_branches = local_branches.chain(remote_branches).flatten().collect::<Vec<Reference>>();
    // debug!("local_and_remote_branches: {:?}", local_and_remote_branches);

    let available_branches: Vec<&Reference> = local_and_remote_branches.iter()
        .filter(|r| prefixed_branches.contains(&r.name().as_bstr().to_string()))
        .collect();

    trace!("available_branches: {:?}", available_branches.clone().iter().map(|b| b.name().as_bstr().to_string()).collect::<Vec<String>>());

    let mut commit_metric_vec: Vec<GitCommitMetric> = Vec::new();
    for branch in available_branches {
        let commits = if let Some(id) = branch.try_id() {
            if let Ok(revwalk) = id.ancestors().all() {
                revwalk.filter(|r| r.is_ok()).map(|r| r.unwrap().id).collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        // trace!("commit_count: {:?}", commits);
        let mut val: Vec<GitCommitMetric> = commits.iter().map(|c| GitCommitMetric::from(*c)).collect();
        commit_metric_vec.append(&mut val);
    }

    // Ok(vec![GitCommitMetric::new()])
    Ok(commit_metric_vec)
}