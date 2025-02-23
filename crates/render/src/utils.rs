use std::fs::File;
use std::path::PathBuf;

pub(crate) fn get_writer(file_path: Option<PathBuf>) -> Box<dyn std::io::Write> {
    match file_path {
        None => Box::from(std::io::stdout()),
        Some(fp) => Box::from(File::create(fp).unwrap()),
    }
}
