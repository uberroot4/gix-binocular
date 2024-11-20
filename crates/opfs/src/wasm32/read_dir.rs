use std::{fmt, io};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use futures::StreamExt;

use crate::terminate_self;
use crate::thread;

#[derive(Debug, Clone)]
pub(crate) struct InnerReadDir {
    pub(crate) dirp: BTreeSet<crate::DirEntry>,
    pub(crate) root: PathBuf,
}

pub struct ReadDir {
    inner: Arc<InnerReadDir>,
    iter: std::collections::btree_set::IntoIter<crate::DirEntry>,
}


impl ReadDir {
    pub fn new(inner: InnerReadDir) -> Self {
        let iter = inner.dirp.clone().into_iter(); // Consumes the BTreeSet to create the iterator
        Self {
            inner: Arc::new(inner),
            iter,
        }
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // This will only be called from std::fs::ReadDir, which will add a "ReadDir()" frame.
        // Thus the result will be e g 'ReadDir("/home")'
        fmt::Debug::fmt(&*self.inner.root, f)
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<crate::DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        // Use the stored iterator to maintain state between calls
        self.iter.next().map(Ok) // Iterate and wrap in Ok
    }
}

pub fn readdir<P: AsRef<Path>>(path: P) -> std::io::Result<crate::ReadDir> {
    shared::trace!("read_dir_sync(path={:?})", path.as_ref());
    let (tx, rx) = futures_channel::oneshot::channel();
    let dir_ = std::path::PathBuf::from(path.as_ref());
    shared::debug!("dir_: {:?}", dir_);
    let _wasm_thread_handle = thread::spawn(|| {
        wasm_bindgen_futures::spawn_local(async move {
            shared::debug!("dir_: {:?}", dir_);
            let mut read_dir_stream = web_fs::read_dir(dir_.as_path()).await.unwrap();
            shared::trace!("read_dir_stream returned");
            let mut results: Vec<crate::DirEntry> = std::vec![];
            while let Some(path) = read_dir_stream.next().await {
                match path {
                    Ok(entry) => {
                        results.push(entry);
                    }
                    Err(_) => {}
                }
            }
            drop(tx.send(results));
            terminate_self(); // not working rn, as web-fs/open_options has spawn_local
        });
    }).join_async();

    match futures::executor::block_on(rx) {
        Ok(_data) => {
            shared::debug!("received _data");
            let root = path.as_ref().to_path_buf();
            let dirp = BTreeSet::from_iter(_data);
            let inner = self::InnerReadDir {
                dirp,
                root,
            };
            crate::terminate_self();
            Ok(
                crate::ReadDir::new(inner)
            )
        }
        Err(e) => {
            crate::terminate_self();
            use std::io::{Error, ErrorKind};
            shared::error!("Error within threads: {:?}", e);
            Err(Error::new(ErrorKind::Interrupted, e.to_string()))
        }
    }
}