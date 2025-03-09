//! Module for storing configuration information about the running instance

use std::env;
use std::process::exit;

extern crate getopts;

use getopts::Options;

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
