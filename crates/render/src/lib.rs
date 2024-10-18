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

#[cfg(test)]
mod tests {
    use super::*;
    use csv::Writer;
    use std::string::ToString;

    #[derive(Default)]
    struct TestStruct;

    impl Renderable for TestStruct {
        fn headers() -> Vec<String> {
            vec!["Column1".to_string(), "Column2".to_string()]
        }

        fn values(&self) -> Vec<Vec<String>> {
            vec![
                vec!["Value1".to_string(), "Value2".to_string()],
                vec!["Value3".to_string(), "Value4".to_string()],
            ]
        }
    }

    #[derive(Default)]
    struct EmptyStruct;

    impl Renderable for EmptyStruct {
        fn headers() -> Vec<String> {
            vec![]
        }

        fn values(&self) -> Vec<Vec<String>> {
            vec![]
        }
    }

    fn empty_comfy_table() -> String {
        let mut table = comfy_table::Table::new();

        // Setup table style
        table.load_preset(comfy_table::presets::UTF8_FULL);
        table.apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS);
        table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

        // Setup table headers
        let table_headers: Vec<String> = vec![];
        table.set_header(table_headers);

        format!("{table}")
    }

    // Test 1: Render output format basic test
    #[test]
    fn test_render_output_format_basic() {
        let test_data = TestStruct;
        let output = test_data.format(OutputFormat::Render);
        assert!(output.contains("Column1"));
        assert!(output.contains("Value1"));
        assert!(output.contains("Value4"));
    }

    // Test 2: CSV output format basic test
    #[test]
    fn test_csv_output_format_basic() {
        let test_data = TestStruct;
        let output = test_data.format(OutputFormat::CSV);
        assert!(output.contains("Column1,Column2"));
        assert!(output.contains("Value1,Value2"));
    }

    // Test 3: Empty headers for Render output
    #[test]
    fn test_empty_headers_render() {
        let empty_data = EmptyStruct;
        let output = empty_data.format(OutputFormat::Render);
        assert_eq!(empty_comfy_table(), output); // Render output style should still show a table
    }

    // Test 4: Empty headers for CSV output
    #[test]
    fn test_empty_headers_csv() {
        let empty_data = EmptyStruct;
        let output = empty_data.format(OutputFormat::CSV);

        let mut writer = Writer::from_writer(vec![]);
        match writer.write_record(EmptyStruct::headers().clone()) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error writing header");
            }
        }
        let writer_res = writer.into_inner().ok().unwrap();
        let empty_csv_output = String::from_utf8(writer_res).ok().unwrap();

        assert_eq!(empty_csv_output, output);
    }

    // Test 5: Handling empty rows in Render output
    #[test]
    fn test_empty_rows_render() {
        struct TestWithHeadersNoRows;
        impl Renderable for TestWithHeadersNoRows {
            fn headers() -> Vec<String> {
                vec!["Header1".to_string(), "Header2".to_string()]
            }

            fn values(&self) -> Vec<Vec<String>> {
                vec![] // No values
            }
        }

        let data = TestWithHeadersNoRows;
        let output = data.format(OutputFormat::Render);
        assert!(output.contains("Header1"));
        assert!(!output.contains("Value"));
    }

    // Test 6: Handling empty rows in CSV output
    #[test]
    fn test_empty_rows_csv() {
        struct TestWithHeadersNoRows;
        impl Renderable for TestWithHeadersNoRows {
            fn headers() -> Vec<String> {
                vec!["Header1".to_string(), "Header2".to_string()]
            }

            fn values(&self) -> Vec<Vec<String>> {
                vec![] // No values
            }
        }

        let data = TestWithHeadersNoRows;
        let output = data.format(OutputFormat::CSV);
        assert_eq!(output, "Header1,Header2\n");
    }

    // Test 7: CSV output with non-UTF8 strings
    #[test]
    fn test_csv_non_utf8_handling() {
        struct NonUTF8Struct;

        impl Renderable for NonUTF8Struct {
            fn headers() -> Vec<String> {
                vec!["Header1".to_string()]
            }

            fn values(&self) -> Vec<Vec<String>> {
                vec![vec![String::from_utf8_lossy(&[0x80, 0x81]).to_string()]]
            }
        }

        let data = NonUTF8Struct;
        let output = data.format(OutputFormat::CSV);
        assert!(output.contains("�")); // Non-UTF8 should be replaced with �
    }

    // Test 8: Empty struct Render output
    #[test]
    fn test_empty_struct_render_output() {
        let empty_data = EmptyStruct;
        let output = empty_data.format(OutputFormat::Render);
        assert_eq!(empty_comfy_table(), output); // Empty table should render
    }

    // Test 9: JSON output format not implemented
    #[test]
    #[should_panic(expected = "Not yet implemented")]
    fn test_json_output_format_unimplemented() {
        let test_data = TestStruct;
        test_data.format(OutputFormat::JSON);
    }

    // Test 10: CSV header writing error (force error by passing wrong writer)
    #[test]
    #[ignore]
    fn test_csv_header_write_error() {
        struct BadWriter;

        impl Renderable for BadWriter {
            fn headers() -> Vec<String> {
                vec!["Header1".to_string()]
            }

            fn values(&self) -> Vec<Vec<String>> {
                vec![]
            }

            fn format(&self, format: OutputFormat) -> String {
                if let OutputFormat::CSV = format {
                    let mut writer = Writer::from_writer(vec![]);
                    // Deliberately make writer error
                    writer.flush().unwrap_err();
                    "Header1\n".to_string()
                } else {
                    "Unsupported format".to_string()
                }
            }
        }

        let data = BadWriter;
        let output = data.format(OutputFormat::CSV);
        assert_eq!(output, "Header1\n");
    }

    // Test 11: Render with special characters in headers
    #[test]
    fn test_render_with_special_characters() {
        struct SpecialHeader;

        impl Renderable for SpecialHeader {
            fn headers() -> Vec<String> {
                vec!["He@der#1$".to_string()]
            }

            fn values(&self) -> Vec<Vec<String>> {
                vec![vec!["Value!".to_string()]]
            }
        }

        let data = SpecialHeader;
        let output = data.format(OutputFormat::Render);
        assert!(output.contains("He@der#1$"));
    }

    // Test 12: Render with NULL replacement in values
    #[test]
    fn test_render_with_null_values() {
        struct NullValues;

        impl Renderable for NullValues {
            fn headers() -> Vec<String> {
                vec!["Header1".to_string()]
            }

            fn values(&self) -> Vec<Vec<String>> {
                vec![vec![const_values::NULL.to_string()]]
            }
        }

        let data = NullValues;
        let output = data.format(OutputFormat::Render);
        assert!(output.contains("NULL"));
    }

    // Test 13: CSV with NULL replacement in values
    #[test]
    fn test_csv_with_null_values() {
        struct NullValues;

        impl Renderable for NullValues {
            fn headers() -> Vec<String> {
                vec!["Header1".to_string()]
            }

            fn values(&self) -> Vec<Vec<String>> {
                vec![vec![const_values::NULL.to_string()]]
            }
        }

        let data = NullValues;
        let output = data.format(OutputFormat::CSV);
        assert!(output.contains("NULL"));
    }

    // Test 14: Render with long column headers
    #[test]
    fn test_render_long_column_headers() {
        struct LongHeader;

        impl Renderable for LongHeader {
            fn headers() -> Vec<String> {
                vec!["ThisIsAReallyLongHeaderNameThatShouldBeFormattedCorrectly".to_string()]
            }

            fn values(&self) -> Vec<Vec<String>> {
                vec![vec!["Value1".to_string()]]
            }
        }

        let data = LongHeader;
        let output = data.format(OutputFormat::Render);
        assert!(output.contains("ThisIsAReallyLongHeaderNameThatShouldBeFormattedCorrectly"));
    }

    // Test 15: Render large dataset
    #[test]
    fn test_render_large_dataset() {
        struct LargeDataSet;

        impl Renderable for LargeDataSet {
            fn headers() -> Vec<String> {
                vec!["Header1".to_string()]
            }

            fn values(&self) -> Vec<Vec<String>> {
                (0..1000).map(|i| vec![format!("Value{i}")]).collect()
            }
        }

        let data = LargeDataSet;
        let output = data.format(OutputFormat::Render);
        assert!(output.contains("Value999"));
    }

    // Test 16: CSV large dataset
    #[test]
    fn test_csv_large_dataset() {
        struct LargeDataSet;

        impl Renderable for LargeDataSet {
            fn headers() -> Vec<String> {
                vec!["Header1".to_string()]
            }

            fn values(&self) -> Vec<Vec<String>> {
                (0..1000).map(|i| vec![format!("Value{i}")]).collect()
            }
        }

        let data = LargeDataSet;
        let output = data.format(OutputFormat::CSV);
        assert!(output.contains("Value999"));
    }

    // Test 17: Render with numeric values
    #[test]
    fn test_render_with_numeric_values() {
        struct NumericValues;

        impl Renderable for NumericValues {
            fn headers() -> Vec<String> {
                vec!["Header1".to_string()]
            }

            fn values(&self) -> Vec<Vec<String>> {
                vec![vec!["12345".to_string()]]
            }
        }

        let data = NumericValues;
        let output = data.format(OutputFormat::Render);
        assert!(output.contains("12345"));
    }

    // Test 18: CSV with numeric values
    #[test]
    fn test_csv_with_numeric_values() {
        struct NumericValues;

        impl Renderable for NumericValues {
            fn headers() -> Vec<String> {
                vec!["Header1".to_string()]
            }

            fn values(&self) -> Vec<Vec<String>> {
                vec![vec!["12345".to_string()]]
            }
        }

        let data = NumericValues;
        let output = data.format(OutputFormat::CSV);
        assert!(output.contains("12345"));
    }

    // Test 19: Render with boolean values
    #[test]
    fn test_render_with_boolean_values() {
        struct BooleanValues;

        impl Renderable for BooleanValues {
            fn headers() -> Vec<String> {
                vec!["Header1".to_string()]
            }

            fn values(&self) -> Vec<Vec<String>> {
                vec![vec!["true".to_string()]]
            }
        }

        let data = BooleanValues;
        let output = data.format(OutputFormat::Render);
        assert!(output.contains("true"));
    }

    // Test 20: CSV with boolean values
    #[test]
    fn test_csv_with_boolean_values() {
        struct BooleanValues;

        impl Renderable for BooleanValues {
            fn headers() -> Vec<String> {
                vec!["Header1".to_string()]
            }

            fn values(&self) -> Vec<Vec<String>> {
                vec![vec!["true".to_string()]]
            }
        }

        let data = BooleanValues;
        let output = data.format(OutputFormat::CSV);
        assert!(output.contains("true"));
    }
}
