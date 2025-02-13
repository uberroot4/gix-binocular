use super::base::OutputPrinter;
use crate::Value;
use serde::Serialize;
use shared::object::CartographyObject;
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

#[deprecated]
pub(crate) fn render_json(headers: Vec<String>, values: Vec<Value>) -> String {
    let mut elements: Vec<serde_json::Value> = vec![];
    let row_len = headers.len();

    for header in headers {
        let mut object = serde_json::Map::new();
        object.insert(header, serde_json::Value::String(String::from("JSON")));
        elements.push(serde_json::Value::Object(object));
    }

    for row in values {
        let mut object = serde_json::Map::new();
        // We expect each row to be a Value::List variant.
        match row {
            Value::List(inner) => {
                assert_eq!(
                    row_len,
                    inner.len(),
                    "row_len = {}, inner.len() = {}",
                    row_len,
                    inner.len()
                );
                // Preallocate the vector with the number of inner elements.
                let mut values_row: Vec<serde_json::Value> = Vec::with_capacity(row_len);
                // Iterate over the inner values.
                // For each element in the row, format it accordingly.
                for val in inner {
                    values_row.push(serde_json::Value::String(crate::format_value(&val)));
                }
                // Write the record.
                // object.insert(
                //     String::from("header"),
                //     serde_json::Value::Array(values_row),
                // );
                // elements.push(serde_json::Value::Array(values_row))
                elements.extend(values_row);
            }
            // If the row isn’t a list, we print an error message.
            other => {
                eprintln!("Expected row to be a list of strings, got: {:?}", other);
            }
        }
        // elements.push(serde_json::Value::Object(object));
    }

    if let Ok(json_str) = serde_json::to_string(&serde_json::Value::Array(elements)) {
        format!("{}", json_str)
    } else {
        panic!("Could not get JSON")
    }
}
