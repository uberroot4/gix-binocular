mod git {
    pub(crate) mod commit;
    pub mod metrics;
    pub(crate) mod object;
    pub mod traverse;
}
pub(crate) use git::object;

mod utils {
    pub mod git_helper;
    pub mod structs;
}

pub mod traversal {
    use log::{debug, info, trace};
    use shared::object::{CartographyObject, Group, Row};
    use crate::git::object::GitDiffOutcome;
    // use std::cmp::min_by;

    pub fn main(
        repo: &gix::Repository,
        commitlist: Vec<String>,
        max_threads: usize,
        skip_merges: bool,
        diff_algorithm: Option<gix::diff::blob::Algorithm>,
        breadth_first: bool,
        follow: bool,
        limit: Option<usize>,
    ) -> anyhow::Result<Vec<GitDiffOutcome>> {
        let cl = crate::git::commit::prepare_commit_list(
            repo,
            commitlist,
            skip_merges,
            breadth_first,
            follow,
            limit,
        )?;
        info!("Processing {} commit(s)", cl.iter().count());
        let num_threads = num_threads(max_threads);
        trace!("threads used: {:?}", num_threads);
        let diffs =
            crate::git::traverse::traverse_commit_graph(repo, cl, num_threads, diff_algorithm)?;

        Ok(diffs)
    }

    fn num_threads(max_threads: usize) -> usize {
        std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(1)
            .min(max_threads)
    }

    #[cfg(test)]
    mod tests {
    }
}

pub use crate::git::metrics;
pub use crate::git::traverse;
pub use crate::utils::git_helper;
pub use crate::utils::structs;
use render::Renderable;
use shared::object::Value;

pub struct GitDiffMetricsVector {
    value_vector: Vec<git::object::GitDiffOutcome>,
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
            // "blames_json".to_string(),
        ]
    }
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        for val in &self.value_vector {
            // println!("change_map\t{:?}", val.change_map.iter().map(|cm| cm.0.to_string()).collect::<Vec<String>>());

            let changes_info_vec = val
                .change_map
                .clone()
                .iter()
                .map(|cm| git::object::ChangesInfo {
                    file: cm.0.to_string(),
                    insertions: cm.1 .0,
                    deletions: cm.1 .1,
                })
                .collect::<Vec<git::object::ChangesInfo>>();

            let changes_info_list: Vec<_> = changes_info_vec
                .iter()
                .map(|ci| serde_json::to_vec(ci).unwrap())
                .map(|ci| String::from_utf8(ci).unwrap())
                .map(Value::Str)
                .collect();

            values.push(Value::List(vec![
                //val.branch.clone().unwrap_or(render::const_values::NULL.clone()),
                Value::Str(val.commit.to_string()),
                match val.parent {
                    None => Value::Str(render::const_values::NULL.clone()),
                    Some(prnt) => Value::Str(prnt.to_string()),
                },
                Value::Str(val.total_number_of_files_changed.to_string()),
                Value::Str(val.total_number_of_insertions.to_string()),
                Value::Str(val.total_number_of_deletions.to_string()),
                Value::List(changes_info_list),
                // Value::List(blames_info_list),
            ]));
        }
        values
    }
}

#[cfg(test)]
mod tests {
    use crate::traversal::main;
}
