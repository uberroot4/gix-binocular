use super::base::OutputPrinter;
use polars::frame::DataFrame;
use polars::io::json;
use polars::prelude::SerWriter;
use serde::Serialize;
use std::fs::File;
use std::path::PathBuf;

pub struct JSONPrinter {
    pub file_path: Option<PathBuf>,
}

impl OutputPrinter for JSONPrinter {
    fn print<T: Serialize>(&self, object: &T) {
        unimplemented!()
    }

    fn print_df(&self, df: &mut DataFrame) {
        use std::io::prelude::*;

        let mut ffile: Box<dyn std::io::Write> = match &self.file_path {
            None => {
                Box::from(std::io::stdout())
            },
            Some(fp) => {
                Box::from(File::create(fp).unwrap())
            }
        };
        json::JsonWriter::new(&mut ffile)
            .with_json_format(json::JsonFormat::Json)
            .finish(df)
            .unwrap();
    }
}

#[cfg(test)]
mod tests {}
