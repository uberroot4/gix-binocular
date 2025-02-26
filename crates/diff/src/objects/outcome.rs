use gix::bstr::BString;
use gix::ObjectId;
use polars::prelude::*;
use shared::signature::Sig;
use shared::VecDataFrameExt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GitDiffOutcome {
    pub change_map: HashMap<BString, (u32, u32)>,
    pub commit: ObjectId,
    pub parent: Option<ObjectId>,
    pub committer: Option<Sig>,
    pub author: Option<Sig>,
}
pub(crate) struct GitDiffOutcomeVec(pub(crate) Vec<GitDiffOutcome>);

impl GitDiffOutcome {
    pub fn new(
        change_map: HashMap<BString, (u32, u32)>,
        commit: ObjectId,
        parent: Option<ObjectId>,
        committer: Option<Sig>,
        author: Option<Sig>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            change_map,
            commit,
            parent,
            committer,
            author,
        })
    }
}

impl VecDataFrameExt for GitDiffOutcomeVec {
    fn to_df(&self) -> PolarsResult<DataFrame> {
        struct OutcomeDfHelper {
            commit: String,
            parent: Option<String>,
            filename: String,
            insertions: u32,
            deletions: u32,
        }

        let exploded: Vec<_> = self
            .0
            .iter()
            .flat_map(|diff| {
                // Capture the commit for use in the inner closure
                let commit = diff.commit;
                let parent = diff.parent;
                diff.change_map
                    .iter()
                    .map(move |(filename, (insertions, deletions))| {
                        OutcomeDfHelper {
                            commit: commit.to_string(),
                            parent: parent.map(|p| p.to_string()),
                            filename: filename.to_string(),
                            insertions: *insertions,
                            deletions: *deletions,
                        }
                    })
            })
            .collect();

        let capacity = exploded.len();
        let mut commit_vec = Vec::with_capacity(capacity);
        let mut parent_vec = Vec::with_capacity(capacity);
        let mut filename_vec = Vec::with_capacity(capacity);
        let mut insertions_vec = Vec::with_capacity(capacity);
        let mut deletions_vec = Vec::with_capacity(capacity);

        for val in exploded {
            commit_vec.push(val.commit);
            parent_vec.push(val.parent);
            filename_vec.push(val.filename);
            insertions_vec.push(val.insertions);
            deletions_vec.push(val.deletions);
        }

        let df = df![
            "commit" => commit_vec,
            "parent" => parent_vec,
            "filename" => filename_vec,
            "insertions" => insertions_vec,
            "deletions" => deletions_vec,
        ]?;
        debug_assert_eq!(capacity, df.height());
        Ok(df)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gix::bstr::BString;
    use gix::ObjectId;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    // Mock ObjectId for testing
    fn mock_object_id() -> ObjectId {
        let hex_string = "35f39037f97d1a0da12a383506c83b1a58492917";

        // Convert the hex string to a buffer of 40 bytes
        let buffer = Vec::from(hex_string);

        ObjectId::from_hex(&*buffer).unwrap()
    }

    // Mock Sig for testing
    fn mock_signature() -> Sig {
        Sig {
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

        let metrics =
            GitDiffOutcome::new(change_map.clone(), commit, parent, committer, author).unwrap();

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

        let metrics =
            GitDiffOutcome::new(change_map.clone(), commit, parent, committer, author).unwrap();

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
        let metrics = GitDiffOutcome::new(change_map.clone(), commit, None, None, None).unwrap();

        assert_eq!(metrics.change_map, change_map);
    }

    // Test 5: Test Renderable values() for GitDiffMetricsVector with valid data
    #[test]
    fn test_git_diff_metrics_vector_values_non_empty() {
        let mut change_map = HashMap::new();
        change_map.insert(BString::from("file1.txt"), (10, 5));

        let commit = mock_object_id();
        let parent = mock_object_id();
        let metrics =
            GitDiffOutcome::new(change_map.clone(), commit, Some(parent), None, None).unwrap();

        assert_eq!(metrics.commit.to_string(), commit.to_string());
        assert_eq!(
            metrics.parent.expect("expected").to_string(),
            parent.to_string()
        );
        assert_eq!(metrics.total_number_of_files_changed, 1);
        assert_eq!(metrics.total_number_of_insertions, 10);
        assert_eq!(metrics.total_number_of_deletions, 5);

        assert_eq!(metrics.change_map, change_map);
    }
}

#[cfg(test)]
mod ser_tests {
    use super::*;
    use serde_test::{assert_ser_tokens, Token};
    use std::str::FromStr;
    // Mock ObjectId for testing
    fn mock_object_id() -> ObjectId {
        let hex_string = "35f39037f97d1a0da12a383506c83b1a58492917";

        // Convert the hex string to a buffer of 40 bytes
        let buffer = Vec::from(hex_string);

        ObjectId::from_hex(&*buffer).unwrap()
    }

    // Mock Sig for testing
    fn mock_signature() -> Sig {
        Sig {
            name: BString::from("John Doe"),
            email: BString::from("john@example.com"),
            time: gix::date::Time {
                seconds: 1609459200, // Jan 1, 2021
                offset: 0,
                sign: gix::date::time::Sign::Plus,
            },
        }
    }

    #[test]
    fn serialize_git_diff_outcome_with_all_fields() {
        // Create a deterministic change_map using a BTreeMap.
        let mut change_map = HashMap::new();
        change_map.insert(BString::from("file1"), (10, 2));
        change_map.insert(BString::from("file2"), (0, 5));

        let outcome = GitDiffOutcome {
            commit: gix::ObjectId::from_str("35f39037f97d1a0da12a383506c83b1a58492917").unwrap(),
            parent: Some(
                gix::ObjectId::from_str("35f39037f97d1a0da12a383506c83b1a58492917").unwrap(),
            ),
            total_number_of_files_changed: 3,
            total_number_of_insertions: 10,
            total_number_of_deletions: 5,
            committer: Some(mock_signature()),
            author: Some(mock_signature()),
            change_map,
        };

        assert_ser_tokens(
            &outcome,
            &[
                Token::Struct {
                    name: "GitDiffOutcome",
                    len: 8,
                },
                // Field "commit"
                Token::Str("commit"),
                Token::String("35f39037f97d1a0da12a383506c83b1a58492917"),
                // Field "parent"
                Token::Str("parent"),
                Token::Some,
                Token::String("35f39037f97d1a0da12a383506c83b1a58492917"),
                // Field "total_number_of_files_changed"
                Token::Str("total_number_of_files_changed"),
                Token::U64(3),
                // Field "total_number_of_insertions"
                Token::Str("total_number_of_insertions"),
                Token::U32(10),
                // Field "total_number_of_deletions"
                Token::Str("total_number_of_deletions"),
                Token::U32(5),
                // Field "committer"
                Token::Str("committer"),
                Token::Some,
                Token::Struct {
                    name: "Signature",
                    len: 3,
                },
                Token::Str("name"),
                Token::String("John Doe"),
                Token::Str("email"),
                Token::String("john@example.com"),
                Token::Str("time"),
                Token::String("2021-01-01T00:00:00Z"),
                Token::StructEnd,
                // Token::SomeEnd,
                // Field "author"
                Token::Str("author"),
                Token::Some,
                Token::Struct {
                    name: "Signature",
                    len: 3,
                },
                Token::Str("name"),
                Token::String("John Doe"),
                Token::Str("email"),
                Token::String("john@example.com"),
                Token::Str("time"),
                Token::String("2021-01-01T00:00:00Z"),
                Token::StructEnd,
                // Field "changes" as a sequence of ChangesInfo structs.
                Token::Str("changes"),
                Token::Seq { len: Some(2) },
                // ChangesInfo (for "file1")
                Token::Struct {
                    name: "ChangesInfo",
                    len: 3,
                },
                Token::Str("file"),
                Token::String("file1"),
                Token::Str("insertions"),
                Token::U32(10),
                Token::Str("deletions"),
                Token::U32(2),
                Token::StructEnd,
                // ChangesInfo (for "file2")
                Token::Struct {
                    name: "ChangesInfo",
                    len: 3,
                },
                Token::Str("file"),
                Token::String("file2"),
                Token::Str("insertions"),
                Token::U32(0),
                Token::Str("deletions"),
                Token::U32(5),
                Token::StructEnd,
                //
                Token::SeqEnd,
                //
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn serialize_git_diff_outcome_with_only_required_fields() {
        // Create a deterministic change_map using a BTreeMap.
        let mut change_map = HashMap::new();
        // change_map.insert(BString::from("file1"), (10, 2));
        // change_map.insert(BString::from("file2"), (0, 5));

        let outcome = GitDiffOutcome {
            commit: gix::ObjectId::from_str("35f39037f97d1a0da12a383506c83b1a58492917").unwrap(),
            parent: None,
            total_number_of_files_changed: 0,
            total_number_of_insertions: 0,
            total_number_of_deletions: 0,
            committer: None,
            author: None,
            change_map,
        };
        // GitDiffOutcome::new()

        assert_ser_tokens(
            &outcome,
            &[
                Token::Struct {
                    name: "GitDiffOutcome",
                    len: 8,
                },
                // Field "commit"
                Token::Str("commit"),
                Token::String("35f39037f97d1a0da12a383506c83b1a58492917"),
                // Field "parent"
                Token::Str("parent"),
                Token::None,
                // Field "total_number_of_files_changed"
                Token::Str("total_number_of_files_changed"),
                Token::U64(0),
                // Field "total_number_of_insertions"
                Token::Str("total_number_of_insertions"),
                Token::U32(0),
                // Field "total_number_of_deletions"
                Token::Str("total_number_of_deletions"),
                Token::U32(0),
                // Field "committer"
                Token::Str("committer"),
                Token::None,
                // Field "author"
                Token::Str("author"),
                Token::None,
                // Field "changes" as a sequence of ChangesInfo structs.
                Token::Str("changes"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
