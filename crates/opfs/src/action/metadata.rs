use std::io::Error;
use std::path::Path;
use shared::{debug, trace};
use crate::action::{ActionHandler, Answer, AnswerResult};
use crate::thread;

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
    type Output = crate::Metadata;

    async fn handle(&self) -> std::io::Result<Box<dyn Answer>> {
        trace!("handle({:?})", self);
        let file = (&*self.file).clone().to_string();
        // format!("Metadata: {}", self.file)

        match web_fs::metadata::<&Path>(file.as_ref()).await {
            Ok(metadata) => {
                debug!("metadata: {:?}", metadata);
                Ok(Box::new(AnswerResult::Metadata(metadata)))
                // Box::new(AnswerResult::Metadata(metadata))
            }
            Err(e) => {
                Err(
                    // Box::new(AnswerResult::Error(
                    Error::new(e.kind(), format!("Action::Metadata({})", e))
                    // ))
                )
            }
        }
    }
}