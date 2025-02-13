use shared::signature::Sig;
use crate::GitCommitMetricVector;
use base64::prelude::*;
use render::{Renderable, Value};

#[derive(Debug)]
pub struct GitCommitMetric {
    pub commit: gix::ObjectId,
    pub commit_str: String,
    pub message: String,
    pub committer: Option<Sig>,
    pub author: Option<Sig>,
    pub branch: Option<String>,
    pub parents: Vec<String>,
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

impl From<Vec<GitCommitMetric>> for GitCommitMetricVector {
    fn from(value: Vec<GitCommitMetric>) -> Self {
        Self {
            value_vector: value,
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

    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        for val in &self.value_vector {
            let (committer_name, committer_email, committer_time) = match &val.committer {
                None => (
                    render::const_values::NULL.clone(),
                    render::const_values::NULL.clone(),
                    render::const_values::NULL.clone(),
                ),
                Some(sig) => (
                    sig.name.to_string(),
                    sig.email.to_string(),
                    sig.time.format(gix::date::time::format::ISO8601_STRICT),
                ),
            };
            let (author_name, author_email, author_time) = match &val.author {
                None => (
                    render::const_values::NULL.clone(),
                    render::const_values::NULL.clone(),
                    render::const_values::NULL.clone(),
                ),
                Some(sig) => (
                    sig.name.to_string(),
                    sig.email.to_string(),
                    sig.time.format(gix::date::time::format::ISO8601_STRICT),
                ),
            };

            let value_list_parents: Vec<Value> = val
                .parents
                .iter() // borrow each element
                .cloned() // clone the String so we own it
                .map(Value::Str) // wrap each String in Value::Str
                .collect();

            values.push(Value::List(vec![
                Value::Str(
                    val.branch
                        .clone()
                        .unwrap_or(render::const_values::NULL.clone()),
                ),
                Value::Str((*(val.commit_str)).parse().unwrap()),
                Value::Str(committer_time),
                Value::Str(committer_name),
                Value::Str(committer_email),
                Value::Str((*(val.message)).parse().unwrap()),
                Value::Str(author_time),
                Value::Str(author_name),
                Value::Str(author_email),
                Value::List(value_list_parents),
            ]))
        }
        values
    }
}
