use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct ChangesInfo {
    pub file: String,
    pub insertions: u32,
    pub deletions: u32,
}