use std::path::PathBuf;
use web_sys::console;

pub fn fib(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    }
    match n {
        0 => panic!("zero is not a right argument to fib()!"),
        1 | 2 => 1,
        3 => 2,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[allow(dead_code)]
pub async fn worker_discover(p: PathBuf) {
    use gix_discover::{self};
    // let upwards = gix_discover::upwards(WEB_REPO_ROOT.as_ref());
    let worktree = gix_discover::repository::Path::WorkTree {
        0: p.clone(),
    };
    console::log_1(&format!("worktree {:?}", worktree).into());
    let upwards = gix_discover::repository::Path::from_dot_git_dir(p.clone(), gix_discover::repository::Path::WorkTree { 0: p.clone() }.kind(), "./".as_ref());
    console::log_1(&format!("upwards {:?}", upwards).into());

    let is_bare = gix_discover::is_bare(&*p.clone());
    console::log_1(&format!("is_bare {:?}", is_bare).into());

    let is_submodule_git_dir = gix_discover::is_submodule_git_dir(p.clone().as_path());
    console::log_1(&format!("is_submodule_git_dir {:?}", is_submodule_git_dir).into());

    // let metadata = web_fs::metadata(p.clone()).await.expect("TODO metadata");
    // // // let metadata = std::fs::metadata(WEB_REPO_ROOT).expect("lol");
    // console::log_1(&format!("metadata {:?}" , metadata).into());
    // console::log_1(&format!("metadata.is_file {:?}" , metadata.is_file()).into());
    // console::log_1(&format!("metadata.is_dir {:?}" , metadata.is_dir()).into());

    // let cwd = gix_fs::current_dir(false)?;
    // console::log_1(&format!("cwd {:?}" , cwd).into());

    // let dot_git = Path::new(&WEB_REPO_ROOT);
    console::log_1(&format!("dot_git (1) {:?}", p.clone()).into());
    console::log_1(&format!("dot_git (2) {:?}", p.clone()).into());
    // use tokio;
    // use tokio_with_wasm::alias as tokio;
    // let x = tokio::task::spawn(web_fs::metadata(p.clone())).await.expect("TODO");
    // console::log_1(&format!("x {:?}", x).into());
    // while !x.is_finished() {
    //     console::log_1(&"not finished".into());
    // }
    // let x = loop {
    //     // if let Poll::Ready(res) = fs_metadata_future.as_mut().poll(&mut cx) {
    //     if true == x.is_finished() {
    //         break "finished";
    //     }
    // };
    // console::log_1(&format!("x {:?}", x).into());

    // use tokio_with_wasm::alias as tokio;
    // let blocking_join_handle =  tokio::task::spawn_blocking(async || {
    //     web_fs::metadata(p.clone()).await
    // });
    // let result = blocking_join_handle.await.expect("TODOOOO");
    // console::log_1(&format!("result {:?}" , result).into());

    // let is_git = gix_discover::is_git(WEB_REPO_ROOT.as_ref()).await?;
    // console::log_1(&format!("is_git {:?}" , is_git).into());


    // dot_git.join("HEAD").exists()
    // let dot_git = if metadata.is_file() {
    //     let private_git_dir = crate::path::from_gitdir_file(git_dir)?;
    //     Cow::Owned(private_git_dir)
    // } else {
    //     Cow::Borrowed(git_dir)
    // };
    console::log_1(&"end of discover method".into());
}