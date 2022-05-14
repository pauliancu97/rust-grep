mod regex_parser;
mod utils;
use std::path::PathBuf;
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

fn main() {
    if let Ok(Arguments { paths, pattern }) = get_arguments() {
        for path in &paths {
            if let Ok(lines) = read_lines(path) {
                for line_result in lines {
                    if let Ok(line) = line_result {
                        if pattern.is_match(&line) {
                            println!("{}", line);
                        }
                    }
                }
            }
        }
    }
}
