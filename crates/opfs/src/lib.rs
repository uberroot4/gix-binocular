use std::path::Path;
use web_sys::wasm_bindgen::JsCast;
use wasm_thread as thread;

mod wasm32 {
    pub(crate) mod read_dir;
    pub(crate) mod metadata;
    pub(crate) mod file;
}

use crate::wasm32 as fs_imp;

pub use fs_imp::read_dir::{ReadDir};
pub use fs_imp::file::{ThreadSafeFile};
pub use web_fs::{Metadata, DirEntry, File};


fn terminate_self() {
    match web_sys::js_sys::eval("self")
        .unwrap()
        .dyn_into::<web_sys::DedicatedWorkerGlobalScope>() {
        Ok(worker_scope) => {
            worker_scope.close();
        }
        Err(_) => {}
    }
}

pub fn read_dir<P: AsRef<Path>>(path: P) -> std::io::Result<crate::ReadDir> {
    fs_imp::read_dir::readdir(path)
}

pub fn metadata<P: AsRef<Path>>(path: P) -> std::io::Result<crate::Metadata> {
    fs_imp::metadata::metadata(path)
}

// pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<ThreadSafeFile> {
//     fs_imp::file::open(path)
// }