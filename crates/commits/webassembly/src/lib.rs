#![no_std]
mod log;
mod web;
mod external;
//mod source;
//mod asset;

use alloc::boxed::Box;
use alloc::vec::Vec;
//use std::env;
//use std::path::PathBuf;
//use futures::FutureExt;
use alloc::string::ToString;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys, FileSystemDirectoryHandle, FileSystemGetDirectoryOptions, FileSystemGetFileOptions, FileSystemRemoveOptions};
use web_sys::js_sys::{JsString};
use crate::log::{console_err, console_log};
use crate::web::{WebDir};

extern crate console_error_panic_hook;
extern crate core;
extern crate alloc;

#[wasm_bindgen(start)]
fn run() {
    console_log!("run");
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub async fn clear_dir() -> Result<JsValue, JsValue> {
    console_log!("clear_dir");

    let options: FileSystemRemoveOptions = FileSystemRemoveOptions::new();
    options.set_recursive(true);

    let opfs_root = get_root_storage().await;
    let promise = opfs_root.remove_entry_with_options(WEB_REPO_ROOT, &options);
    //let promise = opfs_root.remove_entry(WEB_REPO_ROOT);
    return JsFuture::from(promise).await;
}

#[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[wasm_bindgen]
pub fn show_open_file_picker_wasm() -> Result<js_sys::Promise, JsValue> {
    web_sys::window().unwrap().show_open_file_picker()
}
#[wasm_bindgen]
pub async fn show_directory_picker_wasm() -> Result<web::WebDir, JsValue> {
    //web_sys::window().unwrap().show_directory_picker()
    let dir = web::WebDir::choose().await;
    dir
}

const WEB_REPO_ROOT: &'static str = "web_repo";

#[wasm_bindgen]
pub async fn exec() {
    use wasm_bindgen_futures::spawn_local;

    let opfs_root = get_root_storage().await;
    console_log!("opfsRoot: {:?}", opfs_root);
    let options: FileSystemGetDirectoryOptions = FileSystemGetDirectoryOptions::new();
    options.set_create(true);

    let root_dir = JsFuture::from(opfs_root.get_directory_handle_with_options(WEB_REPO_ROOT, &options)).await
        .unwrap()
        .dyn_into::<web_sys::FileSystemDirectoryHandle>()
        .unwrap();

    let root_web_dir = WebDir::from(root_dir.clone());

    let f_options: FileSystemGetFileOptions = FileSystemGetFileOptions::new();
    f_options.set_create(true);

    return spawn_local(async move {
        match WebDir::choose().await {
            Ok(native_dir) => {
                match native_dir.get_dir_handle(".git").await {
                    Ok(git_native_path) => {
                        console_log!("git_path: {:?}", native_dir);
                        match root_web_dir.create_sub_dir(Box::from(".git"), Some(options)).await {
                            Ok(git_web_dir) => {
                                //JsFuture::from(git_web_dir.clone().as_file_system_handle().get_file_handle_with_options("test2.txt", &f_options)).await.unwrap();
                                let start = instant::Instant::now();
                                console_log!(">>> init git_path {:?}", start);
                                init_git(git_native_path, git_web_dir.clone()).await;
                                console_log!("<<< git_path initialized {:?}", instant::Instant::now() - start);
                                //let path_buf: PathBuf = git_web_dir.into();
                                //console_log!("path_buf: {:?}", PathBuf::from("/web_repo").join(path_buf.clone()).exists());
                                //console_log!("env: {:?}", env::join_paths([path_buf.clone()].iter()));
                            }
                            Err(err) => {
                                console_err!("Failed to create .git: {:?}", serde_wasm_bindgen::Error::from(err));
                            }
                        }
                    }
                    Err(err) => {
                        console_err!("Failed to get .git: {:?}", serde_wasm_bindgen::Error::from(err));
                    }
                }
            }
            Err(err) => {
                console_err!("Failed to open directory: {:?}", serde_wasm_bindgen::Error::from(err));
            }
        };
    });
}

async fn get_root_storage() -> FileSystemDirectoryHandle {
    let navigator = web_sys::window().unwrap().navigator();
    let opfs_root = JsFuture::from(navigator.storage().get_directory())
        .await
        .unwrap()
        .dyn_into::<web_sys::FileSystemDirectoryHandle>()
        .unwrap();
    opfs_root
}

pub async fn init_git(native_dir: WebDir, opfs_dir: WebDir) {
    let directory_name = native_dir.directory_name();
    console_log!(">>>> init_git: {:?}", directory_name);
    console_log!("+++++++++++++++++++++++ FILES {:?} +++++++++++++++++++++++", directory_name);
    let files = native_dir.clone().files().await;
    console_log!("count_files: {:?}", files.len());
    for f in native_dir.clone().files().await {
        // console_log!("handle {:?}", f.name());
        match opfs_dir.clone().create_file(f, None).await {
            Ok(web_file) => {
                //opfs_dir.clone().resolve(web_file).await;
                //console_log!("path_buf {:?}", opfs_dir.clone().resolve(web_file).await.as_path_buf());
            }
            Err(err) => {
                console_err!("Failed create file: {:?}", serde_wasm_bindgen::Error::from(err));
            }
        }
    }
    console_log!("++++++++++++++++++++ DIRECTORIES {:?} ++++++++++++++++++++", directory_name);
    let sub_dirs: Vec<WebDir> = native_dir.clone().directories().await;
    console_log!("count_dirs: {:?}", sub_dirs.len());
    for sub_native_dir in native_dir.clone().directories().await {
        //console_log!("handle {:?}", sub_native_dir.directory_name());
        let sub_web_dir = opfs_dir.clone().create_sub_dir(Box::from(sub_native_dir.directory_name()), None).await.unwrap();
        Box::pin(init_git(sub_native_dir, sub_web_dir)).await
    }
}

#[wasm_bindgen]
pub fn get_location() -> JsString {
    web_sys::window().unwrap().location().to_string()
}
