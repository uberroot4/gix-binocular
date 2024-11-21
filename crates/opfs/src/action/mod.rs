mod metadata;

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

    async fn handle(&self) -> std::io::Result<Self::Output>;
}

pub use metadata::MetadataAction;