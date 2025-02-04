use crate::Value;

pub(crate) fn render_tabular(headers: Vec<String>, values: Vec<Value>) -> String {
    let mut table = comfy_table::Table::new();
    let row_len = headers.len();

    // Setup table style
    table.load_preset(comfy_table::presets::UTF8_FULL);
    table.apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS);
    table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

    // Setup table headers
    let header_color = comfy_table::Color::Green;
    let mut table_headers = vec![];
    let titles = headers;
    for key in &titles {
        table_headers.push(comfy_table::Cell::new(key).fg(header_color));
    }
    table.set_header(table_headers);

    for row in values {
        match row {
            Value::List(inner) => {
                assert_eq!(
                    row_len,
                    inner.len(),
                    "row_len = {}, inner.len() = {}",
                    row_len,
                    inner.len()
                );
                let mut values_row: Vec<comfy_table::Cell> = Vec::with_capacity(row_len);

                for val in inner {
                    values_row.push(comfy_table::Cell::new(crate::format_value(&val)));
                }

                table.add_row(values_row);
            }
            // If the row isn’t a list, we print an error message.
            other => {
                eprintln!("Expected row to be a list of strings, got: {:?}", other);
            }
        }
    }
    format!("{table}")
}
