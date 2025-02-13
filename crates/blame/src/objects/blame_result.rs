use serde::ser::SerializeStruct;
use crate::git::objects::BlameOutcome;

#[derive(Debug, Clone)]
pub struct BlameResult {
    pub blames: Vec<BlameOutcome>,
    pub commit_oid: gix::ObjectId,
}


impl serde::ser::Serialize for BlameResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut state = serializer.serialize_struct("BlameResult", 2)?;
        state.serialize_field("commit", &self.commit_oid.to_string())?;
        state.serialize_field("blames", &self.blames)?;
        state.end()
    }
}