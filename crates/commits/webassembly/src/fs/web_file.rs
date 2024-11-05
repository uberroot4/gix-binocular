use alloc::string::String;
use alloc::vec::Vec;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::{js_sys, JsFuture};
use web_sys::FileSystemFileHandle;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct WebFile(web_sys::FileSystemFileHandle);

impl WebFile {
    pub fn name(&self) -> String {
        self.0.name()
    }

    pub fn file_system_file_handle(self) -> web_sys::FileSystemFileHandle {
        self.0
    }

    // pub fn as_path_buf(self) -> PathBuf {
    //     self.into()
    // }

    pub async fn read_bytes(&self) -> Result<Vec<u8>, JsValue> {
        let file = self.get_file().await?;

        let array_buffer = JsFuture::from(file.array_buffer())
            .await?
            .dyn_into::<js_sys::ArrayBuffer>()?;

        Ok(js_sys::Uint8Array::new(&array_buffer).to_vec())
    }

    async fn get_file(&self) -> Result<web_sys::File, JsValue> {
        JsFuture::from(self.0.get_file())
            .await?
            .dyn_into::<web_sys::File>()
    }
}

impl From<FileSystemFileHandle> for WebFile {
    fn from(value: FileSystemFileHandle) -> Self {
        Self(value)
    }
}