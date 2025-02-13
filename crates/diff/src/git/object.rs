use std::collections::HashMap;
use gix::bstr::BString;
use gix::ObjectId;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use shared::signature::Sig;
use crate::git;

#[derive(Debug, Clone)]
pub struct GitDiffOutcome {
    pub change_map: HashMap<BString, (u32, u32)>,
    pub total_number_of_files_changed: usize,
    pub total_number_of_insertions: u32,
    pub total_number_of_deletions: u32,
    pub commit: ObjectId,
    pub parent: Option<ObjectId>,
    pub committer: Option<Sig>,
    pub author: Option<Sig>
}

impl serde::ser::Serialize for GitDiffOutcome {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("GitDiffOutcome", 8)?;
        state.serialize_field("commit", &self.commit.to_string())?;
        state.serialize_field("parent", &self.parent.map_or(None,|p| Some(p.to_string())))?;
        state.serialize_field("total_number_of_files_changed", &self.total_number_of_files_changed)?;
        state.serialize_field("total_number_of_insertions", &self.total_number_of_insertions)?;
        state.serialize_field("total_number_of_deletions", &self.total_number_of_deletions)?;
        state.serialize_field("committer", &self.clone().committer.map_or(None,|p| Some(p)))?;
        state.serialize_field("author", &self.clone().author.map_or(None,|p| Some(p)))?;
        let changes_info_vec = &self.clone().change_map
            .into_iter()
            .map(|(k, (insertions, deletions))| git::object::ChangesInfo {
                file: k.to_string(),
                insertions,
                deletions,
            })
            .collect::<Vec<git::object::ChangesInfo>>();
        state.serialize_field("changes", changes_info_vec)?;
        state.end()
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct ChangesInfo {
    pub file: String,
    pub insertions: u32,
    pub deletions: u32,
}