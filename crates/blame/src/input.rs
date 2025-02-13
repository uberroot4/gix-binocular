use log::error;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

#[derive(Deserialize)]
struct RawInputObject {
    commit: String,
    files: Vec<String>,
}

pub(crate) struct InputObject {
    pub commit: gix::ObjectId,
    pub files: Vec<String>,
}

pub(crate) fn read_json_content<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<InputObject>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Vec<InputObject>`.
    let raws: Vec<RawInputObject> = serde_json::from_reader(reader)?;

    // Return the `Vector`.
    Ok(raws
        .into_iter()
        .map(|raw| match gix::ObjectId::from_str(&raw.commit) {
            Ok(oid) => InputObject {
                commit: oid,
                files: raw.files,
            },
            Err(e) => {
                let msg = format!("Could not convert {} to gix::ObjectId", raw.commit);
                error!("{}",msg);
                panic!("{}, {}", msg, e)
            }
        })
        .collect())
}
