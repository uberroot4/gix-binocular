use crate::git::metrics::GitCommitMetric;
use gix::Reference;
use log::{debug, trace};
use shared::signature::Sig;

pub fn traverse_commit_graph(
    repo: gix::Repository,
    branches: Vec<String>,
    skip_merges: bool,
) -> anyhow::Result<Vec<GitCommitMetric>> {
    let mailmap = repo.open_mailmap();
    let prefixed_branches: Vec<String> = branches
        .iter()
        .map(|b| {
            if b.contains("origin/") {
                format!("refs/remotes/{b}")
            } else {
                format!("refs/heads/{b}")
            }
        })
        .collect();
    debug!(
        "prefixed_branches ({:?}): {:?}",
        prefixed_branches.len(),
        prefixed_branches
    );
    let references = repo.references()?;
    let local_branches = references.local_branches()?;
    let remote_branches = references.remote_branches()?;
    debug!(
        "local_branches: {:?}",
        references.local_branches()?.collect::<Vec<_>>()
    );
    let local_and_remote_branches = local_branches
        .chain(remote_branches)
        .flatten()
        .collect::<Vec<Reference>>();
    debug!("local_and_remote_branches: {:?}", local_and_remote_branches);

    let available_branches: Vec<&Reference> = local_and_remote_branches
        .iter()
        .filter(|r| prefixed_branches.contains(&r.name().as_bstr().to_string()))
        .collect();

    trace!(
        "available_branches: {:?}",
        available_branches
            .clone()
            .iter()
            .map(|b| b.name().as_bstr().to_string())
            .collect::<Vec<String>>()
    );

    let mut commit_metric_vec: Vec<GitCommitMetric> = Vec::new();
    for branch in available_branches {
        let commits = if let Some(id) = branch.try_id() {
            if let Ok(revwalk) = id.ancestors().all() {
                revwalk.filter(|r| r.is_ok()).map(|r| r.unwrap()).collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        // trace!("commit_count: {:?}", commits);
        let mut val: Vec<GitCommitMetric> = commits
            .iter()
            .filter(|c| {
                return if skip_merges && c.parent_ids.len() > 1 {
                    trace!("Skipping Merge Commit {:?}", c.id);
                    false
                } else {
                    true
                };
            })
            .map(|c| {
                let mut author: Option<Sig> = None;
                let mut committer: Option<Sig> = None;
                if let Ok(commit) = c.object() {
                    author = if let Ok(author_sig) = commit.author() {
                        Some(Sig::from(mailmap.resolve(author_sig)))
                    } else {
                        None
                    };

                    committer = if let Ok(committer_sig) = commit.committer() {
                        Some(Sig::from(mailmap.resolve(committer_sig)))
                    } else {
                        None
                    };
                }
                let mut gcm = GitCommitMetric::from((*c).clone());
                // gcm.author = Some(author.into());
                // gcm.committer = Some(committer.into());
                gcm.author = Option::from(author.unwrap_or(gcm.author.unwrap()));
                gcm.committer = Option::from(committer.unwrap_or(gcm.committer.unwrap()));
                gcm.branch = Option::from(branch.name().shorten().to_string());

                gcm
            })
            .collect();
        commit_metric_vec.append(&mut val);
    }

    Ok(commit_metric_vec)
}
