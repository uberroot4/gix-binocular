use alloc::string::{String, ToString};
use core::fmt;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::stream::JsStream;
use futures::stream::StreamExt;
use web_sys::{FileSystemDirectoryHandle};
use crate::log::{console_log};

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct WebDir {
    origin: FileSystemDirectoryHandle,
}

impl WebDir {
    pub fn as_file_system_handle(self) -> FileSystemDirectoryHandle {
        self.origin
    }

    // pub async fn resolve(self, file: WebFile) {
    //     // match wasm_bindgen_futures::JsFuture::from(self.origin.resolve(&*file.file_system_file_handle())).await
    //     //     .unwrap()
    //     //     .dyn_into::<js_sys::Array>() {
    //     //     Ok(resolved) => {
    //     //         for a in resolved {
    //     //             console_log!("array: {}/{:?}", self.directory_name(), a.as_string());
    //     //         }
    //     //     }
    //     //     Err(err) => {
    //     //         console_err!("Failed resolve file: {:?}", serde_wasm_bindgen::Error::from(err));
    //     //     }
    //     // }
    // }

    pub fn directory_name(&self) -> String {
        self.origin.name()
    }

    // pub fn as_path_buf(self) -> PathBuf {
    //     self.into()
    // }

    // pub async fn create_sub_dir(self, dir_name: Box<str>, options: Option<FileSystemGetDirectoryOptions>) -> Result<WebDir, JsValue> {
    //     let options = options.unwrap_or({
    //         let options: FileSystemGetDirectoryOptions = FileSystemGetDirectoryOptions::new();
    //         options.set_create(true);
    //         options
    //     });
    //
    //     let handle = wasm_bindgen_futures::JsFuture::from(self.origin.get_directory_handle_with_options(&*dir_name, &options)).await?
    //         .dyn_into::<FileSystemDirectoryHandle>()?;
    //     Ok(Self { origin: handle })
    // }

    // pub async fn create_file(self, file: WebFile, options: Option<FileSystemGetFileOptions>) -> Result<WebFile, JsValue> {
    //     let options = options.unwrap_or({
    //         let options: FileSystemGetFileOptions = FileSystemGetFileOptions::new();
    //         options.set_create(true);
    //         options
    //     });
    //
    //     let handle = wasm_bindgen_futures::JsFuture::from(self.origin.get_file_handle_with_options(file.name().as_str(), &options)).await?
    //         .dyn_into::<FileSystemFileHandle>()?;
    //
    //     let writable = wasm_bindgen_futures::JsFuture::from(handle.create_writable()).await?
    //         .dyn_into::<FileSystemWritableFileStream>()?;
    //     let content = file.read_bytes().await?;
    //
    //     //console_log!("content: {:?}", content.clone());
    //
    //     match wasm_bindgen_futures::JsFuture::from(writable.write_with_u8_array(&*content)?).await {
    //         Ok(res) => {
    //             if !res.is_undefined() {
    //                 console_err!("Failed write file (no content)");
    //             }
    //         }
    //         Err(err) => {
    //             console_err!("Failed write file: {:?}", serde_wasm_bindgen::Error::from(err));
    //         }
    //     }
    //     wasm_bindgen_futures::JsFuture::from(writable.close()).await?;
    //
    //     Ok(WebFile::from(handle))
    // }

    // pub async fn count_files(self) -> usize {
    //     let file_handles = self.files().await;
    //
    //     file_handles.len()
    // }
    //
    // pub async fn count_dirs(self) -> usize {
    //     let handles = self.directories().await;
    //
    //     handles.len()
    // }
    //
    // pub async fn files(self) -> Vec<WebFile> {
    //     let res = self.values().await;
    //     let file_handles: Vec<_> = res.iter().filter_map(|s|
    //         s.clone().dyn_into::<FileSystemFileHandle>()
    //             .ok()
    //     ).map(|f| WebFile::from(f)).collect();
    //     file_handles
    // }
    //
    // pub async fn directories(self) -> Vec<WebDir> {
    //     let res = self.values().await;
    //     let file_handles: Vec<_> = res.iter().filter_map(|s|
    //         s.clone()
    //             .dyn_into::<FileSystemDirectoryHandle>()
    //             .ok()
    //     ).map(|f| WebDir::from(f)).collect();
    //     file_handles
    // }
    //
    // async fn values(self) -> Vec<JsValue> {
    //     let stream = JsStream::from(self.origin.values());
    //     let res = stream.collect::<Vec<Result<JsValue, JsValue>>>().await;
    //     let iter: Vec<_> = res.iter().filter_map(|s| s.clone().ok()).collect();
    //     iter
    // }
}

impl fmt::Display for WebDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WebDir.name: {}", self.origin.name())
    }
}
