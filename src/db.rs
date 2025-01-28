/// A struct of named strings which refer to filenames of SQL queries in the program
pub struct QueryFiles {
    query1: &'static str,
}

/// The Query files themselves
static QUERY_FILES: QueryFiles = QueryFiles {
    query1: "src/sql/query1.sql",
};

// TODO: write a test which iterates over the queries and tests that each of the files is present
