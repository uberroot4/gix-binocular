use crate::printer::base::OutputPrinter;
use polars::prelude::*;
use std::fs::File;
use std::path::PathBuf;

pub struct CSVPrinter {
    pub file_path: Option<PathBuf>,
}

impl OutputPrinter for CSVPrinter {
    fn print_df(&self, df: &mut DataFrame) {
        //use std::io::prelude::*;

        let mut ffile: Box<dyn std::io::Write> = match &self.file_path {
            None => Box::from(std::io::stdout()),
            Some(fp) => Box::from(File::create(fp).unwrap()),
        };

        CsvWriter::new(&mut ffile).finish(df).unwrap();
    }
}
