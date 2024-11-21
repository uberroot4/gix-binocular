use web_fs::Metadata;

pub trait Answer {
    type AnswerType;
}

pub enum AnswerResult {
    Success(bool),
    Error(std::io::Error),
    Metadata(crate::Metadata), // Assuming `crate::Metadata` exists
    // Metadata(String), // Assuming `crate::Metadata` exists
    FileContents(Vec<u8>),
    DirectoryContents(Vec<String>),
}

// impl Answer for AnswerResult {
//     fn as_debug(&self) -> String {
//         match self {
//             AnswerResult::Success(result) => format!("Success: {}", result),
//             AnswerResult::Error(err) => format!("Error: {}", err),
//             AnswerResult::Metadata(metadata) => format!("Metadata: {:?}", metadata),
//             AnswerResult::FileContents(contents) => format!("File Contents: {} bytes", contents.len()),
//             AnswerResult::DirectoryContents(entries) => format!("Directory Contents: {} entries", entries.len()),
//         }
//     }
// }


pub struct MetadataAnswer {
    // value: crate::Metadata,
    value: String,


}

impl MetadataAnswer {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Answer for MetadataAnswer {
    type AnswerType = String;
}

pub struct BooleanAnswer;
impl Answer for BooleanAnswer {
    type AnswerType = bool; // Example: Start and Stop return a boolean indicating success
}