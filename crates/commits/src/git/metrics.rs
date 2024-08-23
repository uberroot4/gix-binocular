use render::{Renderable};
use crate::GitCommitMetricVector;
use base64::prelude::*;

#[derive(Debug)]
pub struct GitCommitMetric {
    pub commit: gix::ObjectId,
    pub commit_str: String,
    pub message: String,
    pub committer: Option<shared::Sig>,
    pub author: Option<shared::Sig>,
    pub branch: Option<String>,
    pub parents: String,
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
            .collect::<Vec<_>>()
            .join(";");
        Self {
            commit: commit.id,
            commit_str: commit.id.to_string(),
            //message: commit_ref.message.to_string().trim().to_string(),
            message: BASE64_STANDARD.encode(commit_ref.message.to_string().trim()),
            author: Some(shared::Sig::from(commit_ref.author)),
            committer: Some(shared::Sig::from(commit_ref.committer)),
            branch: None,
            parents,
        }
    }
}

impl From<Vec<GitCommitMetric>> for GitCommitMetricVector {
    fn from(value: Vec<GitCommitMetric>) -> Self {
        Self {
            value_vector: value
        }
    }
}

impl Renderable for GitCommitMetricVector {
    fn headers() -> Vec<String> {
        vec![
            "branch".to_string(),
            "commit".to_string(),
            "commit_date".to_string(),
            "committer_name".to_string(),
            "committer_email".to_string(),
            "message".to_string(),
            "author_date".to_string(),
            "author_name".to_string(),
            "author_email".to_string(),
            "parents".to_string(),
        ]
    }

    fn values(&self) -> Vec<Vec<String>> {
        let mut values: Vec<Vec<String>> = Vec::new();
        for val in &self.value_vector {
            let (committer_name, committer_email, committer_time) = match &val.committer {
                None => (render::const_values::NULL.clone(), render::const_values::NULL.clone(), render::const_values::NULL.clone()),
                Some(sig) => (
                    sig.name.to_string(),
                    sig.email.to_string(),
                    sig.time.format(gix::date::time::format::ISO8601_STRICT)
                )
            };
            let (author_name, author_email, author_time) = match &val.author {
                None => (render::const_values::NULL.clone(), render::const_values::NULL.clone(), render::const_values::NULL.clone()),
                Some(sig) => (
                    sig.name.to_string(),
                    sig.email.to_string(),
                    sig.time.format(gix::date::time::format::ISO8601_STRICT)
                )
            };

            values.push(vec![
                val.branch.clone().unwrap_or(render::const_values::NULL.clone()),
                (*(val.commit_str)).parse().unwrap(),
                committer_time,
                committer_name,
                committer_email,
                (*(val.message)).parse().unwrap(),
                author_time,
                author_name,
                author_email,
                (*(val.parents)).parse().unwrap(),
                // (*(val.committer.unwrap().name).to_string()).parse().unwrap()
            ]);
        }
        values
    }
}