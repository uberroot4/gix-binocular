use alloc::string::{String, ToString};
use core::fmt::{Debug};
use alloc::sync::Arc;
use alloc::vec;
use vfs::error::VfsErrorKind;
use vfs::VfsResult;
use wasm_bindgen::__rt::std::collections::HashMap;
use super::{WebDir, WebFile};

type WebFsHandle = Arc<WebFsImpl>;

pub trait OriginPrivateFileSystem<D, F> {
    fn set_root(&self, dir: D);
    fn read_dir(&self, path: &str); /*-> VfsResult<Box<dyn Iterator<Item=String> + Send>>*/
    fn create_dir(&self, path: &str); /*-> VfsResult<Box<dyn Iterator<Item=String> + Send>>*/
}
// #[derive(Debug)]
pub struct WebFS {
    handle: WebFsHandle,
}

impl core::fmt::Debug for WebFS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("In-Browser File System")
    }
}

impl WebFS {
    /// Create a new web-based filesystem
    pub fn new() -> Self {
        WebFS {
            handle: WebFsImpl::new()
        }
    }

    fn ensure_has_parent(&self, path: &str) -> VfsResult<()> {
        let separator = path.rfind('/');
        if let Some(index) = separator {
            if self.exists(&path[..index])? {
                return Ok(());
            }
        }
        Err(VfsErrorKind::Other("Parent path does not exist".into()).into())
    }
}

impl Default for WebFS {
    fn default() -> Self {
        Self::new()
    }
}

impl OriginPrivateFileSystem<WebDir, WebFile> for WebFS {
    fn set_root(&self, web_dir: WebDir) {
        todo!("{:?}", web_dir)
    }
    fn read_dir(&self, path: &str) {
        todo!("{}", path)
    }

    fn create_dir(&self, path: &str) {
        todo!("{}", path)
    }
}

// impl Debug for WebFS {
//     fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
//         f.write_str("In-Browser File System")
//     }
// }

#[derive(Debug)]
struct WebFsImpl {
    files: HashMap<String, WebFile>,
}

impl WebFsImpl {
    pub fn new() -> Self {
        let mut files = HashMap::new();
        // Add root directory
        // files.insert(
        //     "".to_string(),
        //     WebFile {
        //         content: vec![],
        //
        //     },
        // );
        Self { files }
    }
}