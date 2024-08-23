use std::string::ToString;
use csv::Writer;
pub use cli::output_format::OutputFormat;


pub mod const_values {
    use lazy_static::lazy_static;
    lazy_static! {
        pub static ref NULL: String = "NULL".to_string();
    }
    // pub  const NULL: String = String::new("NULL");
}

pub trait Renderable {
    // fn render_object(format: OutputFormat);
    fn headers() -> Vec<String>;
    fn values(&self) -> Vec<Vec<String>>;

    fn format(&self, format: OutputFormat) -> String {
        match format {
            OutputFormat::Render => {
                let mut table = comfy_table::Table::new();

                // Setup table style
                table.load_preset(comfy_table::presets::UTF8_FULL);
                table.apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS);
                table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

                // Setup table headers
                let header_color = comfy_table::Color::Green;
                let mut table_headers = vec![];
                let titles = Self::headers();
                for key in &titles {
                    table_headers.push(comfy_table::Cell::new(key).fg(header_color));
                }
                table.set_header(table_headers);

                for row in self.values() {
                    let mut table_row: Vec<comfy_table::Cell> = vec![];

                    for val in row {
                        table_row.push(comfy_table::Cell::new(val));
                    }

                    table.add_row(table_row);
                }
                format!("{table}")
            }
            OutputFormat::CSV => {
                let mut writer = Writer::from_writer(vec![]);
                let row_len = Self::headers().len();
                match writer.write_record(Self::headers().clone()) {
                    Ok(_) => {}
                    Err(_) => {
                        eprintln!("Error writing header");
                    }
                }

                for row in self.values() {
                    let mut values_row: Vec<String> = Vec::with_capacity(row_len);
                    for val in row {
                        values_row.push(val.to_string());
                    }
                    match writer.write_record(values_row) {
                        Ok(_) => {}
                        Err(_) => {
                            eprintln!("Error writing record");
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
            OutputFormat::JSON => {
                todo!("Not yet implemented")
            }
        }
    }
    fn render(&self, format: OutputFormat) {
        println!("{}", self.format(format));
    }
}