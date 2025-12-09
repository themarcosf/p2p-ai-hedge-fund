use std::error::Error;
use std::{env, fs, process};

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Application requires args[1] to be a query and args[2] to be a filepath.");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Query: {}", config.query);
    println!("File path: {}", config.file_path);

    let contents: String = fs::read_to_string(config.file_path)?;

    println!("Contents: {contents}");

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
