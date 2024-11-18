use std::{
    string::{String, ToString},
    vec::Vec,
};
use core::fmt;
use wasm_bindgen::{
    JsValue,
    JsCast,
};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::{
    stream::JsStream,
    JsFuture,
};
use futures::stream::StreamExt;
use web_sys::{FileSystemDirectoryHandle, FileSystemFileHandle, js_sys, FileSystemHandle};
use crate::{
    browser::opfs::OpfsFile,
    log::{console_log, console_err},
};

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct OpfsDir {
    origin: FileSystemDirectoryHandle,
}

impl OpfsDir {
    pub async fn resolve(self, possible_descendant: &FileSystemHandle) {
        match wasm_bindgen_futures::JsFuture::from(self.origin.resolve(possible_descendant)).await
            .unwrap()
            .dyn_into::<js_sys::Array>() {
            Ok(resolved) => {
                for a in resolved {
                    console_log!("array: {}/{:?}", self.name(), a.as_string());
                }
            }
            Err(err) => {
                console_err!("Failed resolve file: {:?}", serde_wasm_bindgen::Error::from(err));
            }
        }
    }
    pub async fn get_dir_handle(&self, dir_name: &str) -> Result<Self, JsValue> {
        let dir = JsFuture::from(self.origin.get_directory_handle(dir_name))
            .await?
            .dyn_into::<FileSystemDirectoryHandle>()?;
        console_log!("get_dir_handle#dir {:?}", dir);

        Ok(Self {
            origin: dir,
        })
    }

    pub fn name(&self) -> String {
        self.origin.name()
    }

    pub async fn files(self) -> Vec<OpfsFile> {
        let res = self.values().await;
        let file_handles: Vec<_> = res.iter().filter_map(|s|
            s.clone().dyn_into::<FileSystemFileHandle>()
                .ok()
        ).map(|f| OpfsFile::from(f)).collect();
        file_handles
    }

    pub async fn directories(self) -> Vec<OpfsDir> {
        let res = self.values().await;
        let handles: Vec<_> = res.iter().filter_map(|s|
            s.clone()
                .dyn_into::<FileSystemDirectoryHandle>()
                .ok()
        ).map(|f| Self::from(f)).collect();
        handles
    }

    async fn values(self) -> Vec<JsValue> {
        let stream = JsStream::from(self.origin.values());
        let res = stream.collect::<Vec<Result<JsValue, JsValue>>>().await;
        let iter: Vec<_> = res.iter().filter_map(|s| s.clone().ok()).collect();
        iter
    }
}


impl From<FileSystemDirectoryHandle> for OpfsDir {
    fn from(value: FileSystemDirectoryHandle) -> Self {
        Self { origin: value }
    }
}

impl Into<FileSystemHandle> for OpfsDir {
    fn into(self) -> FileSystemHandle {
        self.origin.dyn_into::<FileSystemHandle>().unwrap()
    }
}

impl fmt::Display for OpfsDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Origin private file system Directory name: {}", self.origin.name())
    }
}


