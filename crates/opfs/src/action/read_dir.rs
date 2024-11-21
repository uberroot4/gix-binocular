use std::collections::BTreeSet;
use std::{fmt, io};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use async_std::stream::StreamExt;
use crate::action::{ActionHandler, Answer, AnswerResult};

#[derive(Debug, Clone)]
pub struct InnerReadDir {
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
        // fmt::Debug::fmt(&*self.inner.root, f)
        write!(f, "ReadDir({:?})", &*self.inner.root)
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<crate::DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        // Use the stored iterator to maintain state between calls
        self.iter.next().map(Ok) // Iterate and wrap in Ok
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

// #[derive(Debug)]
// pub struct ReadDirActionResultWrapper {
//     inner: self::ReadDir,
// }
// impl ActionResultWrapper for ReadDirActionResultWrapper {
//     fn inner(self) -> Box<self::ReadDir> {
//         Box::new(self.inner)
//     }
// }

impl ActionHandler for ReadDirAction {
    type Output = self::ReadDir;

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
            let inner = InnerReadDir {
                dirp,
                root,
            };
            Ok(
                // Box::new(ReadDirActionResultWrapper { inner: self::ReadDir::new(inner) })
                Box::new(AnswerResult::DirectoryContents(self::ReadDir::new(inner)))
            )
        } else {
            Err(
                Error::new(ErrorKind::Other, format!("handle({})#web_fs::read_dir", self.dir))
            )
        }
    }
}