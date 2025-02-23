use crate::JSONPrinter;
use serde::Serialize;

/// A trait for printing serializable objects.
pub trait OutputPrinter {
    fn print<T: Serialize>(&self, object: &T);
}

pub struct VoidPrinter {}

/// An enum that wraps both printer types.
pub enum Printer {
    Json(JSONPrinter),
    Void(VoidPrinter), // Csv(CSVPrinter),
}

/// Implement the trait for our enum, delegating to the contained printer.
impl OutputPrinter for Printer {
    fn print<T: Serialize>(&self, object: &T) {
        match self {
            Printer::Json(p) => p.print(object),
            Printer::Void(_) => {}
            // Printer::Csv(p) => p.print(object),
        }
    }
}
