use super::base::OutputPrinter;
use serde::Serialize;
use std::path::PathBuf;

pub struct JSONPrinter {
    pub file_path: Option<PathBuf>,
}

impl OutputPrinter for JSONPrinter {
    fn print<T: Serialize>(&self, object: &T) {
        match serde_json::to_string_pretty(object) {
            Ok(json) => match &self.file_path {
                None => println!("{}", json),
                Some(fp) => {
                    use std::io::prelude::*;
                    let mut file = std::fs::File::create(fp).unwrap();
                    file.write_all(json.as_ref())
                        .expect("TODO: panic message");
                }
            },
            Err(e) => eprintln!("Error serializing object to JSON: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
}