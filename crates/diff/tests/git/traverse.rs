use diff::traverse::traverse_commit_graph;

#[test]
fn it_adds_two() {
    let result = 2+2;
    assert_eq!(result, 4);
}

// mod util {
//     use gix_testtools::Result;
//     use gix::diff::rewrites;
//     // use gix_object::{FindExt, TreeRefIter};
//     use std::convert::Infallible;
//     use std::path::{Path, PathBuf};
//
//     pub fn repo_workdir() -> gix_testtools::Result<PathBuf> {
//         gix_testtools::scripted_fixture_read_only_standalone("make_diff_for_rewrites_repo.sh")
//     }
// }