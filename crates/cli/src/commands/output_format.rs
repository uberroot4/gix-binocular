#[derive(clap::ValueEnum, Debug, PartialEq, Clone)]
/// Represent the different type of available formats
pub enum OutputFormat {
    /// Render the output as table
    Render,
    /// Print the output in json format
    JSON,
    /// Print the output in csv format
    CSV,
}