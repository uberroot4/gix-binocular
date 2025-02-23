use base64::prelude::*;
use polars::datatypes::{DataType, TimeUnit, TimeZone};
use polars::{df, prelude::*};
use serde::ser::SerializeStruct;
use shared::signature::Sig;
use shared::{time_to_utc_with_offset, VecDataFrameExt};

#[derive(Debug, Clone)]
pub(crate) struct GitCommitMetric {
    pub commit: gix::ObjectId,
    #[deprecated]
    pub commit_str: String,
    pub message: String,
    pub committer: Option<Sig>,
    pub author: Option<Sig>,
    pub branch: Option<String>,
    pub parents: Vec<String>,
}

// Newtype wrapper around Vec<MyType>
pub(crate) struct GitCommitMetricVec(pub(crate) Vec<GitCommitMetric>);

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

impl VecDataFrameExt for GitCommitMetricVec {
    fn to_df(&self) -> PolarsResult<DataFrame> {
        let commit_metric_vec = &self.0;

        let capacity = commit_metric_vec.len();
        let mut commit_str_vec = Vec::with_capacity(capacity);
        let mut message_vec = Vec::with_capacity(capacity);
        let mut committer_name_vec = Vec::with_capacity(capacity);
        let mut committer_email_vec = Vec::with_capacity(capacity);
        let mut committer_date_vec = Vec::with_capacity(capacity);
        let mut author_name_vec = Vec::with_capacity(capacity);
        let mut author_email_vec = Vec::with_capacity(capacity);
        let mut author_date_vec = Vec::with_capacity(capacity);
        let mut branch_vec = Vec::with_capacity(capacity);
        let mut parents_vec = Vec::with_capacity(capacity);

        let match_sig = |sig: &Option<Sig>,
                         name_vec: &mut Vec<Option<String>>,
                         email_vec: &mut Vec<Option<String>>,
                         dt_vec: &mut Vec<Option<i64>>|
         -> () {
            match sig {
                None => {
                    name_vec.push(None);
                    email_vec.push(None);
                    dt_vec.push(None);
                }
                Some(sig) => {
                    name_vec.push(Some(sig.name.to_string()));
                    email_vec.push(Some(sig.email.to_string()));
                    dt_vec.push(Some(time_to_utc_with_offset(sig.time).timestamp_millis()));
                }
            }
        };

        for val in commit_metric_vec.iter() {
            commit_str_vec.push(val.commit.to_string());
            message_vec.push(val.message.clone());
            match_sig(
                &val.committer,
                &mut committer_name_vec,
                &mut committer_email_vec,
                &mut committer_date_vec,
            );
            match_sig(
                &val.author,
                &mut author_name_vec,
                &mut author_email_vec,
                &mut author_date_vec,
            );
            branch_vec.push(val.branch.clone());
            parents_vec.push(val.parents.clone());
        }
        let mut df = df![
            "commit" => commit_str_vec,
            "message" => message_vec,
            "branch" => branch_vec,
            "committer_name" =>  committer_name_vec,
            "committer_email" =>  committer_email_vec,
            "commit_dt" =>  committer_date_vec,
            "author_name" =>  author_name_vec,
            "author_email" =>  author_email_vec,
            "author_dt" =>  author_date_vec,
            "parents" => {
                // Map each inner Vec<String> into a Series.
                // The inner series' names are unimportant; they get replaced by the outer ListChunked.
                let series_iter = parents_vec.into_iter().map(|inner| Series::new("".into(), inner));
                // Create a ListChunked from the iterator.
                let list_chunked = ListChunked::from_iter(series_iter);
                // Convert the ListChunked into a Series.
                list_chunked.into_series()
            },
        ]?;
        let convert_dt_column = |df: &mut DataFrame, column: &str| {
            df.with_column(
                df.column(column)
                    .unwrap()
                    .cast(&DataType::Datetime(
                        TimeUnit::Milliseconds,
                        Some(TimeZone::from("Utc".to_owned())),
                    ))
                    .unwrap(),
            )
            .expect(&*format!("Could not convert dt column '{}'", column));
        };
        convert_dt_column(&mut df, "commit_dt");
        convert_dt_column(&mut df, "author_dt");
        debug_assert_eq!(commit_metric_vec.len(), df.height());
        Ok(df)
        // TODO add .lazy()
    }
}
