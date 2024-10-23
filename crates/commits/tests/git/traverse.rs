use crate::git::traverse::util::{get_demo_repo, get_demo_repo_merges};
use commits::traverse::traverse_commit_graph;
use shared::logging;
#[ctor::ctor]
fn init() {
    logging::init_logging(None);
}

#[test]
fn check_correct_number_of_results_no_branches() {
    let local_repo = get_demo_repo();
    let result = traverse_commit_graph(
        local_repo,
        vec![],
        false,
    ).unwrap();
    assert_eq!(result.iter().clone().count(), 0);
}

#[test]
fn check_correct_number_of_results_main_branch() {
    let local_repo = get_demo_repo();
    let result = traverse_commit_graph(
        local_repo,
        vec!["main".to_string()],
        false,
    ).unwrap();
    assert_eq!(result.iter().clone().count(), 21);
}

#[test]
fn check_correct_number_of_results_main_branch_merges() {
    let local_repo = get_demo_repo_merges();
    let result = traverse_commit_graph(
        local_repo,
        vec!["main".to_string()],
        false,
    ).unwrap();
    assert_eq!(result.iter().clone().count(), 35);
}
#[test]
fn check_correct_number_of_results_main_branch_no_merges() {
    let local_repo = get_demo_repo_merges();
    let result = traverse_commit_graph(
        local_repo,
        vec!["main".to_string()],
        true,
    ).unwrap();
    assert_eq!(result.iter().clone().count(), 32);
}

#[test]
fn check_correct_number_of_results_dev_branch_unknown() {
    let local_repo = get_demo_repo();
    let result = traverse_commit_graph(
        local_repo,
        vec!["dev".to_string()],
        false,
    ).unwrap();
    assert_eq!(result.iter().clone().count(), 0);
}

mod util {
    use std::path::PathBuf;
    use log::{debug, trace};

    pub fn get_demo_repo() -> gix::Repository {
        let repo_workdir_pathbuf = repo_workdir("make_diff_for_rewrites_repo.sh").unwrap();

        trace!("get_demo_repo {:?}", repo_workdir_pathbuf.as_path());
        gix::discover(repo_workdir_pathbuf.as_path()).unwrap()
    }

    pub fn get_demo_repo_merges() -> gix::Repository {
        let repo_workdir_pathbuf = repo_workdir("make_blame_repo.sh").unwrap();

        trace!("get_demo_repo_merges {:?}", repo_workdir_pathbuf.as_path());
        gix::discover(repo_workdir_pathbuf.as_path()).unwrap()
    }

    fn repo_workdir(script_name: &str) -> gix_testtools::Result<PathBuf> {
        gix_testtools::scripted_fixture_read_only_standalone(script_name)
    }
}