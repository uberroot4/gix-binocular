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

        assert_eq!(metrics.change_map, change_map);
    }
}