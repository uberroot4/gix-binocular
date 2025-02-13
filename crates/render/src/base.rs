use serde::Serialize;
// SOURCE: https://github.com/AmrDeveloper/GQL/blob/master/crates/gitql-cli/src/printer/base.rs
use shared::object::CartographyObject;
use crate::JSONPrinter;

/// A trait for printing serializable objects.
pub trait OutputPrinter {
    fn print<T: Serialize>(&self, object: &T);
}

/// An enum that wraps both printer types.
pub enum Printer {
    Json(JSONPrinter),
    // Csv(CSVPrinter),
}

/// Implement the trait for our enum, delegating to the contained printer.
impl OutputPrinter for Printer {
    fn print<T: Serialize>(&self, object: &T) {
        match self {
            Printer::Json(p) => p.print(object),
            // Printer::Csv(p) => p.print(object),
        }
    }
}