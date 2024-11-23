mod metadata;
mod read_dir;

use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fmt;
use std::fmt::Formatter;
use tsify_next::Tsify;

#[derive(Debug, Tsify, Serialize, Deserialize, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[tsify(namespace)]
pub enum Action {
    Stop,
    Start,
    Metadata(String),
    ReadDir(String),
    OpenFile(String),
    ReadFile(String),
}

pub trait ActionHandler {
    async fn handle(&self) -> std::io::Result<Box<dyn Answer>>;
}

pub trait Answer: Send + Sync + Any {
    fn inner_debug(&self) -> String; // A helper for `Debug` implementation
    fn as_any(&self) -> &dyn Any;
}

impl Answer for anyhow::Error {
    fn inner_debug(&self) -> String {
        format!("{:?}", self)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Answer for bool {
    fn inner_debug(&self) -> String {
        format!("{:?}", self)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Answer for crate::Metadata {
    fn inner_debug(&self) -> String {
        format!("{:?}", self)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Answer for ReadDir {
    fn inner_debug(&self) -> String {
        format!("{:?}", self)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Debug for dyn Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner_debug())
    }
}

pub use metadata::MetadataAction;
pub use read_dir::{ReadDir, ReadDirAction};
