pub(crate) mod utils;

pub use cli::output_format::OutputFormat;

pub mod printer;

pub mod const_values {
    use lazy_static::lazy_static;
    lazy_static! {
        pub static ref NULL: String = "NULL".to_string();
    }
    // pub  const NULL: String = String::new("NULL");
}
