extern crate walkdir;
use clap::{Arg, ArgAction, Command};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use walkdir::WalkDir;

// use oncelock for the compiled regex pattern to avoid recompilation
static FILE_NAME_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_filename_regex() -> &'static Regex {
    FILE_NAME_REGEX.get_or_init(|| Regex::new(r"([^/\\]*$)").unwrap())
}

fn main() {
    // define command line interface
    let matches = Command::new("rust_duplicate_files")
        .version("1.0")
        .author("majdap")
        .about("Find duplicate files in a directory")
        .arg(
            Arg::new("path")
                .help("The directory path to search for duplicates")
                .required(true)
                .index(1)
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("ignore-from-file")
                .short('i')
                .long("ignore-from-file")
                .help("Path to file with items to ignore (one per line)")
                .value_name("FILE")
                .value_parser(clap::value_parser!(PathBuf))
                .action(ArgAction::Set),
        )
        .get_matches();

    // get the directory path to search
    let path = matches.get_one::<PathBuf>("path").unwrap();

    // process ignore file if provided
    let ignored_file_names =
        if let Some(ignore_file) = matches.get_one::<PathBuf>("ignore-from-file") {
            println!("Loading ignore patterns from: {}", ignore_file.display());

            match read_lines(ignore_file) {
                Ok(lines) => lines.flatten().collect::<Vec<String>>(),
                Err(e) => {
                    eprintln!("Error reading ignore file: {}", e);
                    std::process::exit(1);
                }
            }
        } else {
            Vec::new()
        };

    // find and analyze files
    let duplicate_files = find_duplicate_files(path, &ignored_file_names);

    // print results
    print_duplicates(&duplicate_files);
}

fn find_duplicate_files(
    path: &Path,
    ignored_file_names: &[String],
) -> HashMap<String, Vec<String>> {
    let mut files_hash: HashMap<String, Vec<String>> = HashMap::new();

    // walk the directory tree
    for file in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        // only process files, not directories

        let file_path = file.path().to_string_lossy().to_string();

        // skip if the file path is in the ignore list
        if ignored_file_names
            .iter()
            .any(|pattern| file_path.contains(pattern))
        {
            continue;
        }

        if let Some(file_name) = parse_file_name(&file_path) {
            // skip default files if ignore_defaults is true

            files_hash
                .entry(file_name.to_string())
                .or_insert_with(Vec::new)
                .push(file_path);
        }
    }

    // filter out entries with only one file (not duplicates)
    files_hash.retain(|_, paths| paths.len() > 1);

    files_hash
}

fn print_duplicates(duplicates: &HashMap<String, Vec<String>>) {
    if duplicates.is_empty() {
        println!("No duplicate files found.");
        return;
    }

    println!("Found {} sets of duplicate files:", duplicates.len());

    for (file_name, paths) in duplicates {
        println!("File: {}", file_name);
        for path in paths {
            println!("  - {}", path);
        }
        println!();
    }
}

fn parse_file_name(file_path: &str) -> Option<&str> {
    let re = get_filename_regex();

    re.captures(file_path).and_then(|caps| {
        let capture = caps.get(1).map_or("", |m| m.as_str());
        if !capture.contains('.') {
            None
        } else {
            Some(capture)
        }
    })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
