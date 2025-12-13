use minigrep::{search_case_insensitive, search_case_sensitive};
use std::env::Args;
use std::error::Error;
use std::{env, fs, process};

////////////////////////////////////////////////////////////////////////////////
// implementation
////////////////////////////////////////////////////////////////////////////////
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string is missing."),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("File path is missing."),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    for line in results {
        println!("Search line: {line}")
    }

    Ok(())
}

fn main() {
    let args: Args = env::args();

    let config: Config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
