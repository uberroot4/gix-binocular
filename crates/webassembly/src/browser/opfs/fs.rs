#[allow(unused_imports)]
use futures::FutureExt;
use std::{
    string::ToString,
    vec::Vec,
    boxed::Box,
    string::String,
    format,
};

use wasm_bindgen::JsValue;
use web_sys::FileSystemDirectoryHandle;
use super::OpfsDir;
use crate::external::showDirectoryPicker;
use crate::log::{console_err, console_log};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecursionType {
    BreadthFirst,
    DepthFirst,
}

pub async fn choose() -> Result<OpfsDir, JsValue> {
    let dir_handle_promise = showDirectoryPicker();

    let dir_handle = wasm_bindgen_futures::JsFuture::from(dir_handle_promise).await?;
    console_log!("choose#dir_handle {:?}", dir_handle);

    // Convert the result into a `FileSystemDirectoryHandle`
    let dir_handle = FileSystemDirectoryHandle::from(dir_handle);

    // Wrap the `FileSystemDirectoryHandle` in `WebDir`
    Ok(OpfsDir::from(dir_handle))
}

#[allow(dead_code)]
pub async fn read_dir(path: OpfsDir, recursive: Option<RecursionType>) { //-> io::Result<std::fs::DirEntry> {
    for f in <OpfsDir as Clone>::clone(&path).files().await {
        console_log!("f\t{}/{}", path.name(),f.name());
    }
    let directories: Vec<OpfsDir> = <OpfsDir as Clone>::clone(&path).directories().await;
    if let Some(recursion_type) = recursive {
        match recursion_type {
            RecursionType::BreadthFirst => {
                directories.iter().for_each(|d| console_log!("d\t{}/{}", path.name(),d.name()));
                for d in directories {
                    let _ = Box::pin(self::read_dir(d.clone(), recursive)).await;
                }
            }
            RecursionType::DepthFirst => {
                for d in directories.clone() {
                    let _ = Box::pin(self::read_dir(d.clone(), recursive)).await;
                }
                directories.iter().for_each(|d| console_log!("d\t{}/{}", path.name(),d.name()))
            }
        }
    } else {
        directories.iter().for_each(|d| console_log!("d\t{}/{}", path.name(),d.name()))
    }
}

pub async fn load_into_browser(parent_path: String, path: OpfsDir) {
    use super::super::application::{create_file, write_all, create_dir};
    for opfs_file in <OpfsDir as Clone>::clone(&path).files().await {
        // console_log!("f\t{}/{}", path.name(),f.name());
        match create_file(parent_path.clone(), opfs_file.clone()).await {
            Ok(_file) => {
                // console_log!("file created: {}", opfs_file);
                write_all(opfs_file, _file).await;
            }
            _ => {
                console_err!("Error creating File {:?}", opfs_file)
            }
        }
    }
    // Directories
    let directories: Vec<OpfsDir> = <OpfsDir as Clone>::clone(&path).directories().await;
    for opfs_dir in directories {
        match create_dir(parent_path.clone(), opfs_dir.clone()).await {
            Ok(_) => {
                // console_log!("directory created: {}", opfs_dir);
            }
            Err(_) => {
                console_err!("Error creating directory {:?}", opfs_dir)
            }
        }
        let new_parent: String = format!("{}/{}", parent_path, opfs_dir.name());
        // console_log!("new_parent: {}", new_parent);
        let _ = Box::pin(self::load_into_browser(new_parent, opfs_dir.clone())).await;
    }
}