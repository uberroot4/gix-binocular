use alloc::string::String;
use alloc::vec::Vec;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::{js_sys, JsFuture};
use web_sys::FileSystemFileHandle;


#[derive(Debug, Clone)]

pub struct WebFile {
    name: String,
    readonly: bool,
    content: Vec<u8>,
}

impl WebFile {
    pub fn name(self) -> String {
        self.name
    }

    // pub fn file_system_file_handle(self) -> web_sys::FileSystemFileHandle {
    //     self.origin
    // }

    // pub fn as_path_buf(self) -> PathBuf {
    //     self.into()
    // }


    // async fn get_file(&self) -> Result<web_sys::File, JsValue> {
    //     JsFuture::from(self.origin.get_file())
    //         .await?
    //         .dyn_into::<web_sys::File>()
    // }
}

