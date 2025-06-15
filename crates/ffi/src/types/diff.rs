use binocular_diff::GitDiffOutcome;
use gix::bstr::BString;
use gix::ObjectId;
use std::collections::HashMap;

pub type GixDiffAlgorithm = gix::diff::blob::Algorithm;
#[uniffi::remote(Enum)]
pub enum GixDiffAlgorithm {
    Histogram,
    Myers,
    MyersMinimal,
}

#[derive(Debug, uniffi::Record)]
pub struct BinocularDiffVec {
    pub change_map: HashMap<BString, BinocularDiffStats>,
    pub commit: ObjectId,
    pub parent: Option<ObjectId>,
    pub committer: Option<crate::types::signature::BinocularSig>,
    pub author: Option<crate::types::signature::BinocularSig>,
}
#[derive(Debug, uniffi::Record)]
pub struct BinocularDiffStats {
    insertions: u32,
    deletions: u32,
    kind: String,
}

impl From<GitDiffOutcome> for BinocularDiffVec {
    fn from(value: GitDiffOutcome) -> Self {
        Self {
            change_map: value
                .change_map
                .iter()
                .map(|(k, v)| {
                    (
                        k.to_owned(),
                        BinocularDiffStats {
                            insertions: v.0,
                            deletions: v.1,
                            kind: v.clone().2,
                        },
                    )
                })
                .collect(),
            commit: value.commit,
            parent: value.parent,
            committer: value.committer,
            author: value.author,
        }
    }
}

impl From<BinocularDiffVec> for GitDiffOutcome {
    fn from(value: BinocularDiffVec) -> Self {
        todo!()
    }
}
