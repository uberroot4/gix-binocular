
#[derive(Debug)]
pub struct GitCommitMetric {
    pub commit: gix::ObjectId,
    pub commit_str: String,
    // pub committer: Option<Sig>,
    // pub author: Option<Sig>,
}

impl From<gix::ObjectId> for GitCommitMetric {
    fn from(oid: gix::ObjectId) -> Self {
        Self {
            commit: oid,
            commit_str: oid.to_string(),
        }
    }
}

impl From<gix::Commit<'_>> for GitCommitMetric {
    fn from(gix::Commit { id, .. }: gix::Commit) -> Self {
        Self {
            commit: id,
            commit_str: id.to_string(),
        }
    }
}