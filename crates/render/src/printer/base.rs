use crate::printer::{JSONPrinter, CSVPrinter, VoidPrinter, ParquetPrinter};

/// A trait for printing serializable objects.
pub trait OutputPrinter {
    fn print_df(&self, df: &mut polars::frame::DataFrame);
}

// trait Writer {
//     fn get_writer(&self) -> Box<dyn std::io::Write>;
// }

/// An enum that wraps both printer types.
pub enum Printer {
    Json(JSONPrinter),
    Void(VoidPrinter),
    Csv(CSVPrinter),
    Parquet(ParquetPrinter),
}
//
// impl Writer for Printer {
//     fn get_writer(&self) -> Box<dyn Write> {
//         let ffile = |file_path: Option<PathBuf>| -> Box<dyn std::io::Write> {
//             match file_path {
//                 None => {
//                     Box::from(std::io::stdout())
//                 }
//                 Some(fp) => {
//                     Box::from(File::create(fp).unwrap())
//                 }
//             }
//         };
//
//         let x = match self {
//             Printer::Json(p) => p.file_path.to_owned(),
//             Printer::Csv(p) => p.file_path.to_owned(),
//             Printer::Parquet(p) => p.file_path.to_owned(),
//             Printer::Void(_) => None
//         };
//         x.map(|writer| ffile(writer))
//     }
// }

/// Implement the trait for our enum, delegating to the contained printer.
impl OutputPrinter for Printer {
    fn print_df(&self, df: &mut polars::frame::DataFrame) {
        match self {
            Printer::Json(p) => p.print_df(df),
            Printer::Csv(p) => p.print_df(df),
            Printer::Parquet(p) => p.print_df(df),
            Printer::Void(_) => {}
        }
    }
}
