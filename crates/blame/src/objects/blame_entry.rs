use std::num::NonZeroU32;

#[derive(Debug)]
pub(crate) struct BlameEntry /*(blame::BlameEntry);*/ {
    pub start_in_blamed_file: u32,
    pub start_in_source_file: u32,
    pub len: NonZeroU32,
    pub commit_id: String,
}

impl From<gix::blame::BlameEntry> for BlameEntry {
    fn from(value: gix::blame::BlameEntry) -> Self {
        Self {
            start_in_blamed_file: value.start_in_blamed_file,
            start_in_source_file: value.start_in_source_file,
            len: value.len,
            commit_id: value.commit_id.to_string(),
        }
    }
}