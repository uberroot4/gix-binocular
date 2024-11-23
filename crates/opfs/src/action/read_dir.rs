use crate::action::{ActionHandler, Answer};
use async_std::stream::StreamExt;
use std::collections::BTreeSet;
use std::fmt;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct InnerReadDir {
    pub(crate) dirp: BTreeSet<crate::DirEntry>,
    pub(crate) root: PathBuf,
}

#[derive(Clone)]
pub struct ReadDir {
    inner: Arc<InnerReadDir>,
}

impl IntoIterator for ReadDir {
    type Item = crate::DirEntry;
    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.dirp.clone().into_iter()
    }
}

impl ReadDir {
    pub fn new(inner: InnerReadDir) -> Self {
        // let iter = inner.dirp.clone().into_iter(); // Consumes the BTreeSet to create the iterator
        Self {
            inner: Arc::new(inner),
            // iter,
        }
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // This will only be called from std::fs::ReadDir, which will add a "ReadDir()" frame.
        // Thus the result will be e g 'ReadDir("/home")'
        // fmt::Debug::fmt(&*self.inner.root, f)
        write!(f, "ReadDir({:?})", &*self.inner.root)
    }
}

#[derive(Debug)]
pub struct ReadDirAction {
    dir: String,
}

impl ReadDirAction {
    pub fn new(dir: String) -> Self {
        Self { dir }
    }
}

impl ActionHandler for ReadDirAction {
    async fn handle(&self) -> std::io::Result<Box<dyn Answer>> {
        let mut entries = vec![];
        let p = std::path::PathBuf::from(&self.dir);
        if let Ok(mut read_dir) = web_fs::read_dir::<&Path>(p.clone().as_path()).await {
            while let Some(d) = read_dir.next().await {
                // info!("d = {:?}", d);
                entries.push(d?);
            }
            let root = p.clone();
            let dirp = BTreeSet::from_iter(entries);
            let inner = InnerReadDir { dirp, root };
            Ok(Box::new(self::ReadDir::new(inner)))
        } else {
            Err(Error::new(
                ErrorKind::Other,
                format!("handle({})#web_fs::read_dir", self.dir),
            ))
        }
    }
}
