use crate::objects::blame_entry::BlameEntry;
use crate::objects::blame_statistics::BlameStatistics;
use serde::ser::SerializeStruct;

#[derive(Debug, Clone)]
pub struct BlameOutcome {
    pub entries: Vec<gix::blame::BlameEntry>,
    pub statistics: gix::blame::Statistics,
    pub file_path: String,
}

impl serde::ser::Serialize for BlameOutcome {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("BlameOutcome", 3)?;
        state.serialize_field("file_path", &self.file_path.to_string())?;
        state.serialize_field("statistics", &BlameStatistics::from(self.statistics))?;
        let entries = &self
            .clone()
            .entries
            .into_iter()
            .map(|e| BlameEntry::from(e))
            .collect::<Vec<_>>();
        state.serialize_field("entries", entries)?;
        state.end()
    }
}
