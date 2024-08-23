use anyhow::Result;
use gix::bstr::BString;
use std::collections::HashMap;
use gix::ObjectId;
use crate::GitDiffMetricsVector;
use render::{Renderable};

#[derive(Debug)]
pub struct GitDiffMetrics {
    pub change_map: HashMap<BString, (u32, u32)>,
    pub total_number_of_files_changed: usize,
    pub total_number_of_insertions: u32,
    pub total_number_of_deletions: u32,
    pub commit: ObjectId,
    pub parent: Option<ObjectId>,
    pub committer: Option<shared::Sig>,
    pub author: Option<shared::Sig>,
}

impl GitDiffMetrics {
    pub fn new(
        change_map: HashMap<BString, (u32, u32)>,
        commit: ObjectId,
        parent: Option<ObjectId>,
        committer: Option<shared::Sig>,
        author: Option<shared::Sig>,
    ) -> Result<Self> {
        let total_number_of_files_changed = change_map.values().count();
        let totals = change_map.values().fold((0u32, 0u32), |acc, val| {
            (acc.0 + val.0, acc.1 + val.1)
        });
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
            author,
        })
    }
}

impl From<Vec<GitDiffMetrics>> for GitDiffMetricsVector {
    fn from(value: Vec<GitDiffMetrics>) -> Self {
        Self {
            value_vector: value
        }
    }
}

impl Renderable for GitDiffMetricsVector {
    fn headers() -> Vec<String> {
        vec![
            //"branch".to_string(),
            "commit".to_string(),
            "parent".to_string(),
            "files_changed".to_string(),
            "insertions".to_string(),
            "deletions".to_string(),
        ]
    }
    fn values(&self) -> Vec<Vec<String>> {
        let mut values: Vec<Vec<String>> = Vec::new();
        for val in &self.value_vector {
            // let parent_str = match val.parent {
            //     None => { render::const_values::NULL.clone() }
            //     Some(prnt) => {
            //         prnt.to_string()
            //     }
            // };

            values.push(vec![
                //val.branch.clone().unwrap_or(render::const_values::NULL.clone()),
                val.commit.to_string(),
                match val.parent {
                    None => { render::const_values::NULL.clone() }
                    Some(prnt) => {
                        prnt.to_string()
                    }
                },
                val.total_number_of_files_changed.to_string(),
                val.total_number_of_insertions.to_string(),
                val.total_number_of_deletions.to_string(),
            ]);
        }
        values
    }
}
