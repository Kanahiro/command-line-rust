use clap::{builder::PossibleValue, Arg, ArgAction, Command};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::{error::Error, ops::Range};

#[derive(Debug)]
pub struct Config {
    left: String,
    right: String,
    suppress_1: bool,
    suppress_2: bool,
    suppress_3: bool,
    insensitive: bool,
    delimiter: String,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("commr")
        .bin_name("commr")
        .version("0.1.0")
        .author("Kanahiro Iguchi")
        .about("hogehoge")
        .args([
            Arg::new("left")
                .value_name("LEFT")
                .help("file to grep")
                .default_value("-"),
            Arg::new("right")
                .value_name("RIGHT")
                .help("file to grep")
                .default_value("-"),
            Arg::new("suppress_1")
                .short('1')
                .long("suppress-1")
                .action(ArgAction::SetTrue),
            Arg::new("suppress_2")
                .short('2')
                .long("suppress-2")
                .action(ArgAction::SetTrue),
            Arg::new("suppress_3")
                .short('3')
                .long("suppress-3")
                .action(ArgAction::SetTrue),
            Arg::new("insensitive")
                .short('i')
                .long("insensitive")
                .action(ArgAction::SetTrue),
            Arg::new("delimiter")
                .short('d')
                .long("delimiter")
                .value_name("DELIMITER")
                .default_value("\t"),
        ])
        .get_matches();

    Ok(Config {
        left: matches.get_one::<String>("left").unwrap().to_string(),
        right: matches.get_one::<String>("right").unwrap().to_string(),
        suppress_1: *matches.get_one("suppress_1").unwrap(),
        suppress_2: *matches.get_one("suppress_2").unwrap(),
        suppress_3: *matches.get_one("suppress_3").unwrap(),
        insensitive: *matches.get_one("insensitive").unwrap(),
        delimiter: matches.get_one::<String>("delimiter").unwrap().to_string(),
    })
}

pub fn run(cfg: Config) -> MyResult<()> {
    let left_lines: Vec<String> = open(&cfg.left)?.lines().map(|l| l.unwrap()).collect();
    let right_lines: Vec<String> = open(&cfg.right)?.lines().map(|l| l.unwrap()).collect();

    let count = if left_lines.len() > right_lines.len() {
        left_lines.len()
    } else {
        right_lines.len()
    };

    let mut count_map: HashMap<String, usize> = HashMap::new();

    for line in left_lines.iter() {
        let key = line.to_string();
        let count = count_map.entry(key).or_insert(0);
    }

    for line in right_lines.iter() {
        let key = line.to_string();
        let count = count_map.entry(key).or_insert(1);
        *count += 1;
    }
    // left=0, right=2, both=1

    let left_only_lines = count_map
        .iter()
        .filter(|(_, v)| **v == 0)
        .map(|(k, _)| k)
        .collect::<Vec<&String>>();

    let right_only_lines = count_map
        .iter()
        .filter(|(_, v)| **v == 2)
        .map(|(k, _)| k)
        .collect::<Vec<&String>>();

    let both_lines = count_map
        .iter()
        .filter(|(_, v)| **v == 1)
        .map(|(k, _)| k)
        .collect::<Vec<&String>>();

    for n in 0..count {
        let mut cols: Vec<String> = vec![];
        match left_only_lines.get(n) {
            Some(l) => {
                if !cfg.suppress_1 {
                    cols.push(l.to_string());
                }
            }
            None => {}
        };
        match right_only_lines.get(n) {
            Some(r) => {
                if !cfg.suppress_2 {
                    cols.push(r.to_string());
                }
            }
            None => {}
        };

        match both_lines.get(n) {
            Some(b) => {
                if !cfg.suppress_3 {
                    cols.push(b.to_string());
                }
            }
            None => {}
        };

        println!("{}", cols.join(&cfg.delimiter));
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
