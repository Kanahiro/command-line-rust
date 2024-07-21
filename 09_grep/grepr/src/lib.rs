use clap::ArgGroup;
use clap::{builder::PossibleValue, Arg, ArgAction, Command};
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::{error::Error, ops::Range};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    invert_match: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("grepr")
        .bin_name("grepr")
        .version("0.1.0")
        .author("Kanahiro Iguchi")
        .about("hogehoge")
        .args([
            Arg::new("pattern")
                .value_name("pattern")
                .default_value("")
                .required(true)
                .help("pattern to search"),
            Arg::new("file")
                .value_name("FILE")
                .help("file to grep")
                .default_value("-")
                .num_args(1..),
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .action(ArgAction::SetTrue),
            Arg::new("count")
                .short('c')
                .long("count")
                .action(ArgAction::SetTrue),
            Arg::new("invert-match")
                .short('v')
                .long("invert-match")
                .action(ArgAction::SetTrue),
            Arg::new("insensitive")
                .short('i')
                .long("insensitive")
                .action(ArgAction::SetTrue),
        ])
        .get_matches();

    let insensitive = *matches.get_one::<bool>("insensitive").unwrap();

    Ok(Config {
        pattern: match matches.get_one::<String>("pattern") {
            Some(pattern) => match RegexBuilder::new(pattern)
                .case_insensitive(insensitive)
                .build()
            {
                Ok(re) => re,
                Err(_) => {
                    eprintln!("Invalid pattern \"{}\"", pattern);
                    std::process::exit(1);
                }
            },
            None => Regex::new("")?,
        },
        files: match matches.get_many::<String>("file") {
            Some(files) => files.map(|s| s.to_string()).collect(),
            None => vec![],
        },
        recursive: *matches.get_one::<bool>("recursive").unwrap(),
        count: *matches.get_one::<bool>("count").unwrap(),
        invert_match: *matches.get_one::<bool>("invert-match").unwrap(),
    })
}

pub fn run(cfg: Config) -> MyResult<()> {
    let multiple = cfg.files.len() > 1;
    let mut matched: Vec<String> = vec![];
    for file in cfg.files {
        if file == "-" {
            process_file(&mut matched, "-", &cfg.pattern, multiple, cfg.invert_match)?;
            continue;
        }

        for entry in WalkDir::new(file) {
            match entry {
                Ok(entry) => {
                    let path = entry.path().to_string_lossy();
                    if entry.file_type().is_file() {
                        process_file(
                            &mut matched,
                            &path,
                            &cfg.pattern,
                            multiple,
                            cfg.invert_match,
                        )?;
                    } else if entry.file_type().is_dir() && cfg.recursive {
                        if !cfg.recursive {
                            continue;
                        }

                        for file in WalkDir::new(entry.path()) {
                            match file {
                                Ok(file) => {
                                    if !file.path().is_file() {
                                        continue;
                                    }
                                    let path = file.path().to_string_lossy();
                                    if file.file_type().is_file() {
                                        process_file(
                                            &mut matched,
                                            &path,
                                            &cfg.pattern,
                                            true,
                                            cfg.invert_match,
                                        )?;
                                    }
                                }
                                Err(e) => eprintln!("{}", e),
                            }
                        }
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    }

    let has = matched
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    if has.len() == 0 {
        return Ok(());
    }

    if cfg.count {
        if multiple {
            let mut count_hash: HashMap<String, usize> = HashMap::new();
            for line in has {
                let key = line.split(":").next().unwrap().to_string();
                let count = count_hash.entry(key).or_insert(0);
                *count += 1;
            }

            for (key, value) in count_hash.iter() {
                println!("{}:{}", key, value);
            }
        } else {
            println!("{}", has.len());
        }
    } else {
        println!("{}", has.join("\n"));
    }

    Ok(())
}

fn process_file(
    matched: &mut Vec<String>,
    file_path: &str,
    pattern: &Regex,
    multiple: bool,
    is_invert: bool,
) -> MyResult<()> {
    match open(file_path) {
        Ok(reader) => {
            for line in reader.lines() {
                let line = line?;
                if pattern.is_match(&line) ^ is_invert {
                    let prefix = if multiple {
                        format!("{}:", file_path)
                    } else {
                        "".to_string()
                    };
                    matched.push(format!("{}{}", prefix, line));
                }
            }
        }
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
