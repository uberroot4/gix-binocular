use crate::action::{ActionHandler, Answer};
use crate::thread;
use shared::{debug, trace};
use std::io::Error;
use std::path::Path;

#[derive(Debug)]
pub struct MetadataAction {
    file: String,
}

impl MetadataAction {
    pub fn new(file: String) -> Self {
        Self { file }
    }
}

impl ActionHandler for MetadataAction {
    async fn handle(&self) -> std::io::Result<Box<dyn Answer>> {
        trace!("handle({:?})", self);
        let file = &*self.file.to_string();
        // format!("Metadata: {}", self.file)

        match web_fs::metadata::<&Path>(file.as_ref()).await {
            Ok(metadata) => {
                debug!("metadata: {:?}", metadata);
                Ok(Box::new(metadata))
            }
            Err(e) => Err(Error::new(e.kind(), format!("Action::Metadata({})", e))),
        }
    }
}
