use colored::*;
use std::process;

fn get_default_path() -> Option<String> {
    if cfg!(target_os = "linux") {
        return Some(String::from("/"));
    } else if cfg!(target_os = "windows") {
        return Some(String::from("C:\\"));
    } else {
        return None;
    }
}

pub struct Config {
    pub path: String,
    pub file_name: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
        let mut args_iter = args.into_iter();

        let pathname;
        let file_name;

        args_iter.next(); // Advance the first argument to ignore the file call

        match args_iter.next() {
            Some(file) => file_name = file,
            None => {
                eprintln!("File not specified");
                process::exit(1)
            }
        }

        match args_iter.next() {
            Some(arg) => {
                pathname = arg;
            }
            None => {
                let path = get_default_path();
                match path {
                    Some(path) => {
                        println!(
                            "{}",
                            "Path not specified. Searching your entire computer.".yellow()
                        );
                        pathname = path;
                    }
                    None => {
                        eprintln!("{}", "Could not determine root directory for your OS".red());
                        process::exit(1);
                    }
                }
            }
        }

        return Self {
            path: pathname,
            file_name,
        };
    }
}
