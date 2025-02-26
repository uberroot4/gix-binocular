use gix::date::time::Sign;
use crate::git::traverse::util::{get_demo_repo, get_demo_repo_merges};
use commits::traversal::main;
use shared::logging;
use assertables::assert_contains;
use pretty_assertions::assert_eq;
// Marks a function or static variable as a library/executable constructor
#[ctor::ctor]
fn init() {
    logging::init_logging(None);
}

// Marks a function as a library/executable destructor
#[ctor::dtor]
fn teardown() {}

#[test]
fn check_correct_number_of_results_no_branches() {
    let local_repo = get_demo_repo();
    let result = main(
        local_repo,
        vec![],
        false,
    ).unwrap();
    assert_eq!(result.iter().clone().count(), 0);
}

#[test]
fn check_correct_number_of_results_main_branch() {
    let local_repo = get_demo_repo();
    let result = main(
        local_repo,
        vec!["main".to_string()],
        false,
    ).unwrap();
    assert_eq!(result.iter().clone().count(), 21);
}

#[test]
fn check_correct_number_of_results_main_branch_merges() {
    let local_repo = get_demo_repo_merges();
    let result = main(
        local_repo,
        vec!["main".to_string()],
        false,
    ).unwrap();
    assert_eq!(result.iter().clone().count(), 35);
}
#[test]
fn check_correct_number_of_results_main_branch_no_merges() {
    let local_repo = get_demo_repo_merges();
    let result = main(
        local_repo,
        vec!["main".to_string()],
        true,
    ).unwrap();
    assert_eq!(result.iter().clone().count(), 32);
}

#[test]
fn check_correct_number_of_results_dev_branch_unknown_merges() {
    let local_repo = get_demo_repo();
    let result = main(
        local_repo,
        vec!["dev".to_string()],
        false,
    ).unwrap();
    assert_eq!(result.iter().clone().count(), 0);
}

#[test]
fn check_correct_number_of_results_dev_branch_unknown_no_merges() {
    let local_repo = get_demo_repo();
    let result = main(
        local_repo,
        vec!["dev".to_string()],
        true,
    ).unwrap();
    assert_eq!(result.iter().clone().count(), 0);
}

#[test]
fn check_correct_number_of_results_branch_one_commit_merges() {
    let local_repo = get_demo_repo_merges();
    let result = main(
        local_repo,
        vec!["branch-that-has-one-commit".to_string()],
        false,
    ).unwrap();
    assert_eq!(result.iter().clone().count(), 34);
}

#[test]
fn check_correct_number_of_results_branch_one_commit_no_merges() {
    let local_repo = get_demo_repo_merges();
    let result = main(
        local_repo,
        vec!["branch-that-has-one-commit".to_string()],
        true,
    ).unwrap();
    assert_eq!(result.iter().clone().count(), 32);
}

#[test]
fn check_correct_order_of_commits() {
    let local_repo = get_demo_repo();
    let result = main(
        local_repo,
        vec!["main".to_string()],
        true,
    ).unwrap();

    let result_0 = result.get(0).unwrap();
    assert_eq!(result_0.commit.to_string(), "0cf7a4fe3ad6c49ae7beb394a1c1df7cc5173ce4");

    let result_1 = result.get(1).unwrap();
    assert_eq!(result_1.commit.to_string(), "a9f4112b75ecad0cb07a45e20e2a363f29729157");

    let result_2 = result.get(2).unwrap();
    assert_eq!(result_2.commit.to_string(), "d78c63c5ea3149040767e4387e7fc743cda118fd");
}

#[test]
fn check_correct_metric_properties() {
    let local_repo = get_demo_repo();
    let result = main(
        local_repo,
        vec!["main".to_string()],
        true,
    ).unwrap();

    let result_0 = result.get(0).unwrap();
    assert_eq!(result_0.commit.to_string(), "0cf7a4fe3ad6c49ae7beb394a1c1df7cc5173ce4");
    assert_ne!(result_0.committer, None);
    assert_ne!(result_0.author, None);
    assert_contains!(result_0.parents, &String::from("a9f4112b75ecad0cb07a45e20e2a363f29729157"));
    let committer = result_0.committer.clone().unwrap();
    assert_eq!(committer.name.to_string(), "committer");
    assert_eq!(committer.email.to_string(), "committer@example.com");
    assert_eq!(committer.time, gix::date::Time {
        seconds: 946771200, // 2.1.2000 00:00:00
        offset: 0,
        sign: Sign::Plus
    });

    let author = result_0.author.clone().unwrap();
    assert_eq!(author.name.to_string(), "author");
    assert_eq!(author.email.to_string(), "author@example.com");
    assert_eq!(author.time, gix::date::Time {
        seconds: 946684800, // 1.1.2000 00:00:00
        offset: 0,
        sign: Sign::Plus
    });

    let branch = result_0.branch.clone().unwrap();
    assert_eq!(branch, "main");

    let msg = result_0.message.clone();
    assert_eq!(msg, "cjMtY2hhbmdl"); // echo -n "r3-change" | base64
}

#[test]
fn check_correct_metric_properties_mailmap() {
    let local_repo = get_demo_repo_merges();
    let result = main(
        local_repo,
        vec!["mailmap-test".to_string()],
        true,
    ).unwrap();

    let result_0 = result.get(0).unwrap();
    assert_eq!(result_0.commit.to_string(), "f1c027f839e1facea509b8efc2ddd9bf2ccc9c7e");
    assert_ne!(result_0.committer, None);
    assert_ne!(result_0.author, None);
    assert_contains!(result_0.parents, &String::from("21995c9faa19ca2c03f4f66e5bf32578b7c3e945"));
    let committer = result_0.committer.clone().unwrap();
    assert_eq!(committer.name.to_string(), "Ronald McDonald");
    assert_eq!(committer.email.to_string(), "ronald@mcdonald.lol");
    assert_eq!(committer.time, gix::date::Time {
        seconds: 946771200, // 2.1.2000 00:00:00
        offset: 0,
        sign: Sign::Plus
    });

    let author = result_0.author.clone().unwrap();
    assert_eq!(author.name.to_string(), "Ronald McDonald");
    assert_eq!(author.email.to_string(), "ronald@mcdonald.lol");
    assert_eq!(author.time, gix::date::Time {
        seconds: 946684800, // 1.1.2000 00:00:00
        offset: 0,
        sign: Sign::Plus
    });

    let branch = result_0.branch.clone().unwrap();
    assert_eq!(branch, "mailmap-test");

    let msg = result_0.message.clone();
    assert_eq!(msg, "bWFpbG1hcC1Kb2VSRGV2ZWxvcGVy"); // echo -n "mailmap-JoeRDeveloper" | base64
}

#[test]
#[ignore]
fn check_correct_metric_properties_no_committer_no_author() {
    let local_repo = get_demo_repo_merges();
    let result = main(
        local_repo,
        vec!["no-committer-no-author-test".to_string()],
        true,
    ).unwrap();

    let result_0 = result.get(0).unwrap();
    assert_eq!(result_0.commit.to_string(), "f1c027f839e1facea509b8efc2ddd9bf2ccc9c7e");
    assert_contains!(result_0.parents, &String::from("21995c9faa19ca2c03f4f66e5bf32578b7c3e945"));
    assert_ne!(result_0.committer, None);
    assert_eq!(result_0.author, None);
    let committer = result_0.committer.clone().unwrap();
    assert_eq!(committer.name.to_string(), "committer");
    assert_eq!(committer.email.to_string(), "committer@example.com");
    assert_eq!(committer.time, gix::date::Time {
        seconds: 946771200, // 2.1.2000 00:00:00
        offset: 0,
        sign: Sign::Plus
    });

    let branch = result_0.branch.clone().unwrap();
    assert_eq!(branch, "no-committer-no-author-test");

    let msg = result_0.message.clone();
    assert_eq!(msg, "dGVzdC1maWxlLnR4dA"); // echo -n "test-file.txt" | base64
}

mod util {
    use std::path::PathBuf;
    use log::trace;

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