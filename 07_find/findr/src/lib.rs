use crate::EntryType::*;
use clap::{builder::PossibleValue, Arg, ArgAction, Command};
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    entry_types: Vec<EntryType>,
    names: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("findr")
        .bin_name("findr")
        .version("0.1.0")
        .author("Kanahiro Iguchi")
        .about("hogehoge")
        .args([
            Arg::new("paths")
                .value_name("PATH")
                .help("dir to find")
                .num_args(1..),
            Arg::new("type")
                .value_name("TYPE")
                .long("type")
                .short('t')
                .num_args(0..)
                .value_parser([
                    PossibleValue::new("d"),
                    PossibleValue::new("f"),
                    PossibleValue::new("l"),
                ]),
            Arg::new("name")
                .long("name")
                .short('n')
                .action(ArgAction::Append)
                .num_args(1),
        ])
        .get_matches();

    Ok(Config {
        paths: match matches.get_many::<String>("paths") {
            Some(paths) => paths.map(|s| s.to_string()).collect(),
            None => vec![],
        },
        entry_types: match matches.get_many::<String>("type") {
            Some(types) => types
                .map(|s| match s.as_str() {
                    "d" => Dir,
                    "f" => File,
                    "l" => Link,
                    _ => unreachable!(),
                })
                .collect(),
            None => vec![],
        },
        names: match matches.get_many::<String>("name") {
            Some(names) => names.map(|s| s.to_string()).collect(),
            None => vec![],
        },
    })
}

pub fn run(cfg: Config) -> MyResult<()> {
    let regexes = cfg
        .names
        .iter()
        .map(|name| match Regex::new(&format!("{}", name)) {
            Ok(re) => re,
            Err(_) => {
                eprintln!("error: invalid value '{}'", name);
                std::process::exit(1);
            }
        })
        .collect::<Vec<_>>();

    for path in cfg.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("error: {}", e),
                Ok(entry) => {
                    let path = entry.path().to_string_lossy();

                    if cfg.names.len() > 0 {
                        if !regexes
                            .iter()
                            .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
                        {
                            // 正規表現に合致しなければスキップ
                            continue;
                        }
                    }

                    if cfg.entry_types.len() > 0 {
                        if entry.file_type().is_dir() && cfg.entry_types.contains(&Dir) {
                            println!("{}", path)
                        } else if entry.file_type().is_file() && cfg.entry_types.contains(&File) {
                            println!("{}", path)
                        } else if entry.file_type().is_symlink() && cfg.entry_types.contains(&Link)
                        {
                            println!("{}", path)
                        };
                    } else {
                        println!("{}", path)
                    }
                }
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
