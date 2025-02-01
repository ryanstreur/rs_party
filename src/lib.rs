use std::fs;

pub mod fairings;
pub mod conf;
pub mod db;
pub mod logging;
pub mod model;

/// Return the contents of a file as a string, printing a helpful error message
/// if opening the file fails.
fn file_text(file_path: &str) -> String {
    let error_message = format!("Error opening file: {}", file_path);
    fs::read_to_string(file_path).expect(&error_message)
}
