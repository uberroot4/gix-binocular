use anyhow::Result;
use gix::bstr::BString;
use std::collections::HashMap;
use gix_hash::ObjectId;
use crate::git::sig::Sig;

#[derive(Debug)]
pub struct GitDiffMetrics {
    pub change_map: HashMap<BString, (u32, u32)>,
    pub total_number_of_files_changed: usize,
    pub total_number_of_insertions: u32,
    pub total_number_of_deletions: u32,
    pub commit: ObjectId,
    pub parent: Option<ObjectId>,
    pub committer: Option<Sig>,
    pub author: Option<Sig>,
}

impl GitDiffMetrics {
    pub fn new(
        change_map: HashMap<BString, (u32, u32)>,
        commit: ObjectId,
        parent: Option<ObjectId>,
        committer: Option<Sig>,
        author: Option<Sig>,
    ) -> Result<Self> {
        let total_number_of_files_changed = change_map.values().count();
        let totals = change_map.values().fold((0u32, 0u32), |mut acc, val| {
            (acc.0 + val.0, acc.1 + val.1)
        });
        /*change_map.iter().for_each(|cm| {
            println!("cm.0.to_string(): {:?} {:?}", cm.0.to_string(),cm.1);
        });*/
        let total_number_of_insertions = totals.0;
        let total_number_of_deletions = totals.1;

        Ok(Self {
            change_map,
            total_number_of_files_changed,
            total_number_of_insertions,
            total_number_of_deletions,
            commit,
            parent,
            committer,
            author
        })
    }
}
