use anyhow::Result;
use gix::bstr::BString;
use std::collections::HashMap;
use gix::ObjectId;
use crate::GitDiffMetricsVector;
use render::{Renderable};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
struct ChangesInfo {
    file: String,
    insertions: u32,
    deletions: u32,
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

        // println!("commit {:?}", commit);
        // println!("parent {:?}", parent);
        // for i in change_map.clone() {
        //     println!("{:?}", i);
        // }

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
            "details_json".to_string(),
        ]
    }
    fn values(&self) -> Vec<Vec<String>> {
        let mut values: Vec<Vec<String>> = Vec::new();
        for val in &self.value_vector {
            // println!("change_map\t{:?}", val.change_map.iter().map(|cm| cm.0.to_string()).collect::<Vec<String>>());

            let changes_info_vec = val.change_map.clone().iter().map(|cm| ChangesInfo {
                file: cm.0.to_string(),
                insertions: cm.1.0,
                deletions: cm.1.1,
            }).collect::<Vec<ChangesInfo>>();
            // println!("{:}", serde_json::to_string(&changes_info_vec).unwrap());

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
                // val.change_map.iter().map(|cm| cm.0.to_string()).collect::<Vec<String>>().join(","),
                format!("{:}", serde_json::to_string(&changes_info_vec).unwrap())
            ]);
        }
        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use gix::ObjectId;
    use gix::bstr::BString;

    // Mock ObjectId for testing
    fn mock_object_id() -> ObjectId {
        let hex_string = "35f39037f97d1a0da12a383506c83b1a58492917";

        // Convert the hex string to a buffer of 40 bytes
        let buffer = Vec::from(hex_string);

        ObjectId::from_hex(&*buffer).unwrap()
    }

    // Mock Sig for testing
    fn mock_signature() -> shared::Sig {
        shared::Sig {
            name: BString::from("John Doe"),
            email: BString::from("john@example.com"),
            time: gix::date::Time {
                seconds: 1609459200, // Jan 1, 2021
                offset: 0,
                sign: gix::date::time::Sign::Plus,
            },
        }
    }

    // Test 1: Test GitDiffMetrics::new() with a non-empty change_map
    #[test]
    fn test_git_diff_metrics_new_non_empty() {
        let mut change_map = HashMap::new();
        change_map.insert(BString::from("file1.txt"), (10, 5));
        change_map.insert(BString::from("file2.rs"), (3, 1));

        let commit = mock_object_id();
        let parent = Some(mock_object_id());
        let committer = Some(mock_signature());
        let author = Some(mock_signature());

        let metrics = GitDiffMetrics::new(change_map.clone(), commit, parent, committer, author).unwrap();

        assert_eq!(metrics.total_number_of_files_changed, 2);
        assert_eq!(metrics.total_number_of_insertions, 13);
        assert_eq!(metrics.total_number_of_deletions, 6);
        assert_eq!(metrics.change_map, change_map);
        assert_eq!(metrics.commit, commit);
        assert_eq!(metrics.parent.unwrap(), parent.unwrap());
        assert_eq!(metrics.committer.unwrap(), mock_signature());
        assert_eq!(metrics.author.unwrap(), mock_signature());
    }

    // Test 2: Test GitDiffMetrics::new() with an empty change_map
    #[test]
    fn test_git_diff_metrics_new_empty() {
        let change_map: HashMap<BString, (u32, u32)> = HashMap::new();
        let commit = mock_object_id();
        let parent = None;
        let committer = Some(mock_signature());
        let author = None;

        let metrics = GitDiffMetrics::new(change_map.clone(), commit, parent, committer, author).unwrap();

        assert_eq!(metrics.total_number_of_files_changed, 0);
        assert_eq!(metrics.total_number_of_insertions, 0);
        assert_eq!(metrics.total_number_of_deletions, 0);
        assert_eq!(metrics.change_map, change_map);
        assert_eq!(metrics.commit, commit);
        assert!(metrics.parent.is_none());
        assert_eq!(metrics.committer.unwrap(), mock_signature());
        assert!(metrics.author.is_none());
    }

    // Test 3: Test GitDiffMetricsVector::from()
    #[test]
    fn test_git_diff_metrics_vector_from() {
        let mut change_map = HashMap::new();
        change_map.insert(BString::from("file1.txt"), (10, 5));

        let commit = mock_object_id();
        let metrics = GitDiffMetrics::new(change_map.clone(), commit, None, None, None).unwrap();

        let vector = GitDiffMetricsVector::from(vec![metrics]);

        assert_eq!(vector.value_vector.len(), 1);
        assert_eq!(vector.value_vector[0].change_map, change_map);
    }

    // Test 4: Test Renderable headers() for GitDiffMetricsVector
    #[test]
    fn test_git_diff_metrics_vector_headers() {
        let headers = GitDiffMetricsVector::headers();
        assert_eq!(headers, vec![
            "commit".to_string(),
            "parent".to_string(),
            "files_changed".to_string(),
            "insertions".to_string(),
            "deletions".to_string(),
            "details_json".to_string(),
        ]);
    }

    // Test 5: Test Renderable values() for GitDiffMetricsVector with valid data
    #[test]
    fn test_git_diff_metrics_vector_values_non_empty() {
        let mut change_map = HashMap::new();
        change_map.insert(BString::from("file1.txt"), (10, 5));

        let commit = mock_object_id();
        let parent = mock_object_id();
        let metrics = GitDiffMetrics::new(change_map.clone(), commit, Some(parent), None, None).unwrap();

        let vector = GitDiffMetricsVector::from(vec![metrics]);

        let values = vector.values();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0][0], commit.to_string());
        assert_eq!(values[0][1], parent.to_string());
        assert_eq!(values[0][2], "1".to_string());
        assert_eq!(values[0][3], "10".to_string());
        assert_eq!(values[0][4], "5".to_string());
        assert!(values[0][5].contains("file1.txt"));
        assert!(values[0][5].contains("10"));
        assert!(values[0][5].contains("5"));
    }

    // Test 6: Test Renderable values() for GitDiffMetricsVector with empty data
    #[test]
    fn test_git_diff_metrics_vector_values_empty() {
        let vector = GitDiffMetricsVector::from(vec![]);
        let values = vector.values();
        assert!(values.is_empty());
    }

    // Test 7: Test GitDiffMetrics with None committer and author
    #[test]
    fn test_git_diff_metrics_none_committer_author() {
        let mut change_map = HashMap::new();
        change_map.insert(BString::from("file.rs"), (1, 0));

        let commit = mock_object_id();
        let metrics = GitDiffMetrics::new(change_map.clone(), commit, None, None, None).unwrap();

        assert!(metrics.committer.is_none());
        assert!(metrics.author.is_none());
    }

    // Test 8: Test Renderable values() with multiple GitDiffMetrics
    #[test]
    fn test_git_diff_metrics_vector_values_multiple() {
        let mut change_map1 = HashMap::new();
        change_map1.insert(BString::from("file1.txt"), (10, 5));

        let mut change_map2 = HashMap::new();
        change_map2.insert(BString::from("file2.rs"), (3, 1));

        let commit1 = mock_object_id();
        let commit2 = mock_object_id();

        let metrics1 = GitDiffMetrics::new(change_map1.clone(), commit1, None, None, None).unwrap();
        let metrics2 = GitDiffMetrics::new(change_map2.clone(), commit2, None, None, None).unwrap();

        let vector = GitDiffMetricsVector::from(vec![metrics1, metrics2]);

        let values = vector.values();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0][0], commit1.to_string());
        assert_eq!(values[1][0], commit2.to_string());
    }
}
