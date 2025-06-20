use crate::objects::blame_result::{BlameResult, BlameResultVec};
use polars::df;
use polars::error::PolarsResult;
use polars::frame::DataFrame;
use shared::VecDataFrameExt;

pub(crate) trait ToDataFrameExt {
    fn to_df(self) -> anyhow::Result<DataFrame>;
}

impl ToDataFrameExt for Vec<BlameResult> {
    fn to_df(self) -> anyhow::Result<DataFrame> {
        let vectorized = BlameResultVec(self);
        let lf = vectorized.to_df()?;

        Ok(lf)
    }
}

impl VecDataFrameExt for BlameResultVec {
    fn to_df(&self) -> PolarsResult<DataFrame> {
        let _span = gix::trace::coarse!("to_df");
        struct BlameResultDfHelper {
            blame_commit: String,
            owner_commit: String,
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
                let blame_commit = blame_result.commit_oid;
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
                        move |(
                            file_path,
                            commit_id,
                            start_in_source_file,
                            range_in_source_file,
                            start_in_blamed_file,
                            range_in_blamed_file,
                        )| BlameResultDfHelper {
                            blame_commit: blame_commit.to_string(),
                            owner_commit: commit_id.to_string(),
                            file_path,
                            start_in_source_file,
                            end_in_source_file: range_in_source_file.end as u32,
                            start_in_blamed_file,
                            end_in_blamed_file: range_in_blamed_file.end as u32,
                        },
                    )
            })
            .collect();

        gix::trace::debug!("finished exploding struct");

        let capacity = exploded.len();
        let mut owner_commit_vec = Vec::with_capacity(capacity);
        let mut blame_commit_vec = Vec::with_capacity(capacity);
        let mut file_path_vec = Vec::with_capacity(capacity);
        let mut start_in_source_file_vec = Vec::with_capacity(capacity);
        let mut end_in_source_file_vec = Vec::with_capacity(capacity);
        let mut start_in_blamed_file_vec = Vec::with_capacity(capacity);
        let mut end_in_blamed_file_vec = Vec::with_capacity(capacity);

        for val in exploded {
            blame_commit_vec.push(val.blame_commit);
            owner_commit_vec.push(val.owner_commit);
            file_path_vec.push(val.file_path);
            start_in_source_file_vec.push(val.start_in_source_file);
            end_in_source_file_vec.push(val.end_in_source_file);
            start_in_blamed_file_vec.push(val.start_in_blamed_file);
            end_in_blamed_file_vec.push(val.end_in_blamed_file);
        }

        gix::trace::debug!("created vector");

        df![
            "blame_commit" => blame_commit_vec,
            "owner_commit" => owner_commit_vec,
            "file_path" => file_path_vec,
            "start_in_source_file" => start_in_source_file_vec,
            "end_in_source_file" => end_in_source_file_vec,
            "start_in_blamed_file" => start_in_blamed_file_vec,
            "end_in_blamed_file" => end_in_blamed_file_vec,
        ]
    }
}
