use crate::action::ActionHandler;
use crate::thread;
use crate::Answer;
use async_std::io::ReadExt;
use shared::{debug, trace};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{ArrayBuffer, Uint8Array};
use web_sys::{FileSystemDirectoryHandle};

#[derive(Debug)]
pub struct ReadFileAction {
    file: String,
}

impl ReadFileAction {
    pub fn new(file: String) -> Self {
        Self { file }
    }
}

impl ActionHandler for ReadFileAction {
    async fn handle(&self) -> std::io::Result<Box<dyn Answer>> {
        trace!("handle({:?})", self);

        // let storage = web_sys::js_sys::global()
        //     .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
        //     .expect("worker global scope is not available")
        //     .navigator()
        //     .storage();
        // let storage_root = JsFuture::from(storage.get_directory()).await;
        // trace!(
        //     "storage_root = {:?} (if you see this, storage did not fucked it up)",
        //     storage_root
        // );
        // let root = storage_root
        //     .expect("Getting root directory failed")
        //     .dyn_into::<FileSystemDirectoryHandle>()
        //     .expect("DYN_INTO_ERROR");
        // trace!(
        //     "root = {:?} (if you see this, storage did not fucked it up)",
        //     root
        // );
        // let file_handle = JsFuture::from(root.get_file_handle(&*self.file)).await;
        // trace!("file_handle: {:?}", file_handle);
        let f = web_fs::get_file(PathBuf::from(&self.file).as_path(), false).await;
        trace!("f: {:?}", f);

        let file = JsFuture::from(f?.get_file())
            .await
            .unwrap()
            .dyn_into::<web_sys::File>()
            .unwrap();


        let array_buffer = JsFuture::from(file.array_buffer())
            .await
            .unwrap()
            .dyn_into::<ArrayBuffer>()
            .unwrap();

        let buffer = Uint8Array::new(&array_buffer).to_vec();
        trace!("buffer = {:?}", String::from_utf8(buffer.clone()));

        Ok(Box::new(buffer))
    }
}
