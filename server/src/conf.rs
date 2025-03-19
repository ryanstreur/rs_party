//! Module for storing configuration information about the running instance

use std::env;
use std::process::exit;

extern crate getopts;

use getopts::Options;

const DEFAULT_PG_HOST: &str = "localhost";
const DEFAULT_PG_USER: &str = "postgres";
const DEFAULT_PG_PASSWORD: &str = "postgres";
const DEFAULT_PG_DB: &str = "rs_party";
const DEFAULT_PG_PORT: &str = "5432";

/// Configuration information for current running program
pub struct Configuration {
    pub verbose: bool,
    pub pghost: String,
    pub pguser: String,
    pub pgpass: String,
    pub pgdatabase: String,
    pub pgport: i16,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            verbose: false,
            pghost: "localhost".to_string(),
            pguser: "postgres".to_string(),
            pgpass: "postgres".to_string(),
            pgport: 5432,
            pgdatabase: "postgres".to_string(),
        }
    }
}

/// Parse options and return configuration struct
///
/// <https://docs.rs/getopts/latest/getopts/>
pub fn build_config() -> Configuration {
    let mut opts = Options::new();

    opts.optflag("v", "verbose", "print verbose output");
    opts.optflag("h", "help", "Print this help text");

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} FILE [options]", program);
        print!("{}", opts.usage(&brief));
        exit(0);
    }

    Configuration {
        verbose: matches.opt_present("v"),
        ..Configuration::default()
    }
}

/// Retrieve DB connection string from environment variables, defaulting to
/// the default Postgresql connection parameters
pub fn get_db_connection_string() -> String {
    let pg_user = env::var("POSTGRES_USER").unwrap_or(DEFAULT_PG_USER.to_string());
    let pg_pw = env::var("POSTGRES_PASSWORD").unwrap_or(DEFAULT_PG_PASSWORD.to_string());
    let pg_host = env::var("POSTGRES_HOST").unwrap_or(DEFAULT_PG_HOST.to_string());
    let pg_port = env::var("POSTGRES_PORT").unwrap_or(DEFAULT_PG_PORT.to_string());
    let pg_db = env::var("POSTGRES_DB").unwrap_or(DEFAULT_PG_DB.to_string());

    format!(
        "postgres://{}:{}@{}:{}/{}",
        pg_user, pg_pw, pg_host, pg_port, pg_db
    )
}
