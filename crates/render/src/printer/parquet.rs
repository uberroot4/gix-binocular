use std::path::PathBuf;
use polars::{frame::DataFrame, prelude::ParquetWriter};
use crate::printer::base::OutputPrinter;

pub struct ParquetPrinter {
    pub file_path: Option<PathBuf>,
}


impl OutputPrinter for ParquetPrinter {
    fn print_df(&self, df: &mut DataFrame) {
        let writer = crate::utils::get_writer(self.file_path.to_owned());
        ParquetWriter::new(writer).finish(df).unwrap();
    }
}