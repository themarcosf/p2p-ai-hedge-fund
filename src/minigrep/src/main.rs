use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Application requires args[1] to be a query and args[2] to be a filepath.")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Self { query, file_path }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args);

    println!("Query: {}", config.query);
    println!("File path: {}", config.file_path);

    let contents: String =
        fs::read_to_string(config.file_path).expect("variable `FILE_PATH` should be a valid path");

    println!("Contents: {contents}")
}
