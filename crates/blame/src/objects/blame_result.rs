use std::ops::Range;
use crate::git::objects::BlameOutcome;
use polars::{df, prelude::*};
use serde::ser::SerializeStruct;
use shared::VecDataFrameExt;

#[derive(Debug, Clone)]
pub(crate) struct BlameResult {
    pub blames: Vec<BlameOutcome>,
    pub commit_oid: gix::ObjectId,
}

pub(crate) struct BlameResultVec(pub(crate) Vec<BlameResult>);

impl VecDataFrameExt for BlameResultVec {
    fn to_df(&self) -> PolarsResult<DataFrame> {
        struct BlameResultDfHelper {
            commit: String,
            file_path: String,
            start_in_source_file: u32,
            end_in_source_file: u32,
            start_in_blamed_file: u32,
            end_in_blamed_file: u32,
        }

        let blame_result_vec = &self.0;
        let exploded: Vec<_> = blame_result_vec
            .iter()
            .flat_map(|blame_result| {
                let commit = blame_result.commit_oid;
                blame_result
                    .blames
                    .iter()
                    .flat_map(|bo| {
                        bo.entries.iter().map(|entry| {
                            (
                                bo.file_path.to_owned(),
                                entry.commit_id,
                                entry.start_in_source_file,
                                entry.range_in_source_file(),
                                entry.start_in_blamed_file,
                                entry.range_in_blamed_file(),
                            )
                        })
                    })
                    .map(
                        |(
                            file_path,
                            commit_id,
                            start_in_source_file,
                            range_in_source_file,
                            start_in_blamed_file,
                            range_in_blamed_file,
                        )| BlameResultDfHelper {
                            commit: commit_id.to_string(),
                            file_path,
                            start_in_source_file,
                            end_in_source_file: range_in_source_file.end as u32,
                            start_in_blamed_file,
                            end_in_blamed_file: range_in_blamed_file.end as u32
                        },
                    )
            })
            .collect();

        let capacity = exploded.len();
        let mut commit_vec = Vec::with_capacity(capacity);
        let mut file_path_vec = Vec::with_capacity(capacity);
        let mut start_in_source_file_vec = Vec::with_capacity(capacity);
        let mut end_in_source_file_vec = Vec::with_capacity(capacity);
        let mut start_in_blamed_file_vec = Vec::with_capacity(capacity);
        let mut end_in_blamed_file_vec = Vec::with_capacity(capacity);

        for val in exploded {
            commit_vec.push(val.commit);
            file_path_vec.push(val.file_path);
            start_in_source_file_vec.push(val.start_in_source_file);
            end_in_source_file_vec.push(val.end_in_source_file);
            start_in_blamed_file_vec.push(val.start_in_blamed_file);
            end_in_blamed_file_vec.push(val.end_in_blamed_file);
        }

        df![
            "commit" => commit_vec,
            "file_path" => file_path_vec,
            "start_in_source_file" => start_in_source_file_vec,
            "end_in_source_file" => end_in_source_file_vec,
            "start_in_blamed_file" => start_in_blamed_file_vec,
            "end_in_blamed_file" => end_in_blamed_file_vec,
        ]
    }
}

impl serde::ser::Serialize for BlameResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut state = serializer.serialize_struct("BlameResult", 2)?;
        state.serialize_field("commit", &self.commit_oid.to_string())?;
        state.serialize_field("blames", &self.blames)?;
        state.end()
    }
}
