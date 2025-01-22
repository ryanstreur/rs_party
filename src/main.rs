//! Main module

pub mod conf;
pub mod logging;
pub mod db;

use std::fs;

use leptos::prelude::*;

use logging::{Logger, LogLevel};

/// A struct of named strings which refer to filenames of SQL queries in the program
pub struct QueryFiles {
    query1: &'static str,
}

/// The Query files themselves
static QUERY_FILES: QueryFiles = QueryFiles {
    query1: "src/sql/query1.sql",
};

// TODO: write a test which iterates over the queries and tests that each of the files is present

/// Main Function for program
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> })
}

/// Return the contents of a file as a string, printing a helpful error message
/// if opening the file fails.
fn file_text(file_path: &str) -> String {
    let error_message = format!("Error opening file: {}", file_path);
    fs::read_to_string(file_path).expect(&error_message)
}
