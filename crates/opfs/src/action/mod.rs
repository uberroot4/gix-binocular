mod metadata;
mod read_dir;

use std::any::Any;
use std::fmt;
use std::fmt::{Formatter};
use serde::{Deserialize, Serialize};
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
    type Output;

    async fn handle(&self) -> std::io::Result<Box<dyn Answer>>;
}

pub trait Answer: Send + Sync
{
    fn inner(&self) -> Box<dyn Any + '_>;
    fn inner_debug(&self) -> String; // A helper for `Debug` implementation
}

pub enum AnswerResult {
    // Success(bool),
    // Error(std::io::Error),
    Metadata(crate::Metadata), // Assuming `crate::Metadata` exists
    // Metadata(String), // Assuming `crate::Metadata` exists
    // FileContents(Vec<u8>),
    DirectoryContents(ReadDir),
}

impl Answer for AnswerResult {
    fn inner(&self) -> Box<dyn Any + '_> {
        match self {
            AnswerResult::Metadata(val) => Box::new(val),
            AnswerResult::DirectoryContents(val) => Box::new(val),
        }
    }
    fn inner_debug(&self) -> String {
        match self {
            AnswerResult::Metadata(metadata) => format!("{:?}", metadata),
            AnswerResult::DirectoryContents(val) => format!("{:?}", val),
        }
    }
}

impl fmt::Debug for dyn Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner_debug())
    }
}
//
// impl fmt::Debug for AnswerResult {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match self {
//             AnswerResult::Metadata(_) => {
//                 // f.write_str("AnswerResult::Metadata fmt")
//                 write!(f, "AnswerResult::Metadata fmt")
//             }
//             AnswerResult::DirectoryContents(_) => {
//                 // f.write_str("AnswerResult::DirectoryContents fmt")
//                 write!(f, "AnswerResult::DirectoryContents fmt")
//             }
//             // _ => {
//             //     write!(f, "Debug is not implemented!")
//             // }
//         }
//     }
// }

pub use metadata::MetadataAction;
pub use read_dir::ReadDirAction;
use crate::action::read_dir::ReadDir;