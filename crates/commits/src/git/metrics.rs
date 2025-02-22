use base64::prelude::*;
use serde::ser::SerializeStruct;
use shared::signature::Sig;

#[derive(Debug, Clone)]
pub struct GitCommitMetric {
    pub commit: gix::ObjectId,
    pub commit_str: String,
    pub message: String,
    pub committer: Option<Sig>,
    pub author: Option<Sig>,
    pub branch: Option<String>,
    pub parents: Vec<String>,
}

impl serde::ser::Serialize for GitCommitMetric {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("GitCommitMetric", 8)?;
        state.serialize_field("commit", &self.commit.to_string())?;
        state.skip_field("commit_str")?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field(
            "committer",
            &self.clone().committer.map_or(None, |p| Some(p)),
        )?;
        state.serialize_field("author", &self.clone().author.map_or(None, |p| Some(p)))?;
        state.serialize_field("parents", &self.parents)?;
        state.end()
    }
}

impl From<gix::revision::walk::Info<'_>> for GitCommitMetric {
    fn from(info: gix::revision::walk::Info) -> Self {
        let commit = info.object().unwrap();
        let commit_ref = commit.decode().unwrap();
        let parents = commit
            .parent_ids()
            .filter(|p| p.object().is_ok())
            .map(|p| p.object().unwrap().into_commit())
            .map(|p_id| p_id.id.to_string())
            .collect::<Vec<_>>();
        // .join(";");
        Self {
            commit: commit.id,
            commit_str: commit.id.to_string(),
            //message: commit_ref.message.to_string().trim().to_string(),
            message: BASE64_STANDARD.encode(commit_ref.message.to_string().trim()),
            author: Some(Sig::from(commit_ref.author)),
            committer: Some(Sig::from(commit_ref.committer)),
            branch: None,
            parents,
        }
    }
}
