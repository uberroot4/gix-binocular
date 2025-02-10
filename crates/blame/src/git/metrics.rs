use gix_blame::{BlameEntry, Statistics};

#[derive(Debug)]
pub struct GitBlameMetric {
    pub entries: Vec<BlameEntry>,
    pub statistics: Statistics,
}
