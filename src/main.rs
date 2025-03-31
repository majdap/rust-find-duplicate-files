extern crate walkdir;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use walkdir::WalkDir;

const DEFAULTS: [&str; 3] = ["index.ts", "routes.ts", "types.ts"];
enum FLAGS {
    Help,
    Ignore,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: rust_duplicate_files <path>");
        return;
    }
    let path = args[1].clone();
    let flags = &args[2..];
    let mut ignore_defaults = false;
    for flag in flags {
        match flag.as_str() {
            "--help" | "-h" => {
                println!("Ask sham lol");
                return;
            }
            "--ignore-defaults" | "-i" => {
                ignore_defaults = true;
            }
            _ => {}
        }
    }
    let mut files_hash: HashMap<String, Vec<String>> = HashMap::new();
    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        let file_name = file.path().to_string_lossy().to_string();
        let parsed_name = parse_file_name(&file_name);
        match parsed_name {
            Some(name) => {
                if DEFAULTS.contains(&name) && ignore_defaults {
                    continue;
                }
                let key = name.to_string();
                let entry = files_hash.entry(key).or_insert_with(Vec::new);
                entry.push(file_name);
            }
            None => {}
        }
    }

    for (key, value) in &files_hash {
        if value.len() > 1 {
            println!("{}: ", key);
            for filepath in value {
                println!("{}, ", filepath);
            }
            print!("\n");
        }
    }
}

fn parse_file_name(file_name: &str) -> Option<&str> {
    let re = Regex::new(r"([^/\\]*$)").unwrap();
    if let Some(caps) = re.captures(file_name) {
        // Return the first capture group
        let capture: &str = caps.get(1).map_or("", |m| m.as_str());
        if !capture.contains(".") {
            return None;
        }
        Some(capture)
    } else {
        // If no match, return an empty string
        return None;
    }
}

fn read_lines(file_name: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::{self, BufRead};
    let file = File::open(file_name).expect("Unable to open file");
    let reader = io::BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Unable to read line"))
        .collect()
}
