use csv::Writer;
use crate::Value;

pub(crate) fn render_csv(headers: Vec<String>, values: Vec<Value>) -> String {
    let mut writer = Writer::from_writer(vec![]);
    let row_len = headers.len();
    match writer.write_record(headers.clone()) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Error writing header");
        }
    }

    for row in values {
        // We expect each row to be a Value::List variant.
        match row {
            Value::List(inner) => {
                assert_eq!(row_len, inner.len(), "row_len = {}, inner.len() = {}", row_len, inner.len());
                // Preallocate the vector with the number of inner elements.
                let mut values_row: Vec<String> = Vec::with_capacity(row_len);
                // Iterate over the inner values.
                // For each element in the row, format it accordingly.
                for val in inner {
                    values_row.push(crate::format_value(&val));
                }
                // Write the record.
                if let Err(e) = writer.write_record(values_row) {
                    eprintln!("Error writing record: {}", e);
                }
            }
            // If the row isn’t a list, we print an error message.
            other => {
                eprintln!("Expected row to be a list of strings, got: {:?}", other);
            }
        }
    }

    fn get_result(writer: Writer<Vec<u8>>) -> Option<String> {
        let writer_res = writer.into_inner().ok()?;
        let str = String::from_utf8(writer_res).ok()?;
        Some(str)
    }

    // Print csv
    match get_result(writer) {
        Some(csv) => {
            csv
        }
        None => { panic!("Error writing content"); }
    }
}