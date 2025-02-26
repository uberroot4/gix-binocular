#[derive(Debug, Clone)]
pub struct BlameOutcome {
    pub entries: Vec<gix::blame::BlameEntry>,
    pub statistics: gix::blame::Statistics,
    pub file_path: String,
}