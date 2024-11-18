use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::{
    FileSystemFileHandle,
    js_sys::{ArrayBuffer, Uint8Array},
};
use std::{
    string::String,
    vec::Vec,
    format
};
use core::fmt::{Debug, Display, Formatter};
use wasm_bindgen_futures::JsFuture;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct OpfsFile {
    inner: FileSystemFileHandle,
}

impl OpfsFile {
    pub fn name(self) -> String {
        self.inner.name()
    }

    pub async fn read_bytes(self) -> Result<Vec<u8>, JsValue> {
        let file = JsFuture::from(self.inner.get_file()).await?.dyn_into::<web_sys::File>()?;

        let array_buffer = JsFuture::from(file.array_buffer())
            .await?
            .dyn_into::<ArrayBuffer>()?;

        Ok(Uint8Array::new(&array_buffer).to_vec())
    }
}

impl From<FileSystemFileHandle> for OpfsFile {
    fn from(value: FileSystemFileHandle) -> Self {
        Self {
            inner: value
        }
    }
}

impl Display for OpfsFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(format!("{}", self.clone().name()).as_str())
    }
}