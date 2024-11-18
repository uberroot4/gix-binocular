use web_fs;
use futures::AsyncWriteExt;
use std::{string::String, format, io};

use crate::browser::opfs::{OpfsDir, OpfsFile};

pub async fn create_dir(parent_path: String, path: OpfsDir) -> io::Result<()> {
    web_fs::create_dir(format!("{}/{}", parent_path, path.name())).await.into()
}

pub async fn create_file(parent_path: String, file: OpfsFile) -> io::Result<web_fs::File> {
    web_fs::File::create(format!("{}/{}", parent_path, file.name())).await.into()
}

pub async fn write_all(source: OpfsFile, mut target: web_fs::File) -> web_fs::File {
    let content = source.read_bytes().await.expect("Could not read file for whatever");
    target.write_all(&*content).await.expect("Could not write file for whatever");

    target
}