mod regex_parser;
mod utils;
use std::{path::{PathBuf, Path}, borrow::Borrow};
use std::fs::read_dir;
use crate::regex_parser::Regex;
use crate::utils::read_lines;
use clap::{Command, Arg};

struct Arguments {
    paths: Vec<PathBuf>,
    pattern: Regex
}

fn get_arguments() -> Result<Arguments, &'static str> {
    let matches = Command::new("rust-grep")
        .version("0.0.1")
        .author("Iancu Paul")
        .about("Simple grep clone for educational purposes")
        .arg(Arg::new("pattern")
                .required(true)
        )
        .arg(Arg::new("files")
                .required(true)
                .multiple_values(true)
                .min_values(1)
                .allow_invalid_utf8(true)
        )
        .get_matches();
    let pattern = Regex::new(matches.value_of("pattern").unwrap())?;
    let paths: Vec<_> = matches.values_of_os("files").unwrap().map(|string| PathBuf::from(string)).collect();
    Ok(Arguments{ paths, pattern })    
}

fn search_dir(path: &Path, pattern: &Regex, is_recursive: bool) {
    match read_dir(path) {
        Ok(entries) => {
            for result_entry in entries {
                match result_entry {
                    Ok(entry) => {
                        if entry.path().is_file() {
                            search_file(&entry.path(), pattern);
                        } else if entry.path().is_dir() && is_recursive {
                            search_dir(&entry.path(), pattern, is_recursive);
                        }
                    },
                    Err(error) => {
                        if let Some(error_code) = error.raw_os_error() {
                            eprintln!("Error {} reading entry from directory {}", error_code, path.display());
                        } else {
                            eprintln!("Error reading entry from directory {}", path.display());
                        }
                    }
                }
            }
        },
        Err(error) => {
            if let Some(error_code) = error.raw_os_error() {
                eprintln!("Error {} reading directory {}", error_code, path.display());
            } else {
                eprintln!("Error reading directory {}", path.display());
            }
        }
    }
}

fn search_file(path: &Path, pattern: &Regex) {
    match read_lines(path) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(string) => {
                        if pattern.is_match(&string) {
                            println!("{}", string);
                        }
                    },
                    Err(error) => {
                        if let Some(error_code) = error.raw_os_error() {
                            eprintln!("Error {} reading line from file {}", error_code, path.display());
                        } else {
                            eprintln!("Error reading line from file {}", path.display());
                        }
                    }
                }
            }
        },
        Err(error) => {
            if let Some(error_code) = error.raw_os_error() {
                eprintln!("Error {} reading file {}", error_code, path.display());
            } else {
                eprintln!("Error reading file {}", path.display());
            }
        }
    }
}

fn search(path: &Path, pattern: &Regex, is_recursive: bool) {
    if path.is_file() {
        search_file(path, pattern);
    } else if path.is_dir() {
        search_dir(path, pattern, is_recursive);
    }
}

fn main() {
    if let Ok(Arguments { paths, pattern }) = get_arguments() {
        for path in &paths {
            search(path, &pattern, false);
        }
    }
}
