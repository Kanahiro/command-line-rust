use clap::{builder::PossibleValue, Arg, ArgAction, Command};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::{error::Error, ops::Range};

#[derive(Debug)]
pub struct Config {
    file: String,
    lines: usize,
    bytes: usize,
    quiet: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("tailr")
        .bin_name("tailr")
        .version("0.1.0")
        .author("Kanahiro Iguchi")
        .about("hogehoge")
        .args([
            Arg::new("file")
                .value_name("FILE")
                .help("file to grep")
                .default_value("-"),
            Arg::new("lines").short('n').default_value("10"),
            Arg::new("bytes").short('c').conflicts_with("lines"),
            Arg::new("quiet").short('q').action(ArgAction::SetTrue),
        ])
        .get_matches();

    Ok(Config {
        file: matches.get_one::<String>("file").unwrap().to_string(),
        lines: match matches.get_one::<String>("lines") {
            Some(n) => n.parse().unwrap(),
            None => 0,
        },
        bytes: match matches.get_one::<String>("bytes") {
            Some(n) => n.parse().unwrap(),
            None => 0,
        },
        quiet: *matches.get_one::<bool>("quiet").unwrap(),
    })
}

pub fn run(cfg: Config) -> MyResult<()> {
    let reader = open(&cfg.file)?;

    if cfg.bytes > 0 {
        let bytes = reader.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>();
        if bytes.len() == 0 {
            return Ok(());
        }

        let start: usize = if bytes.len() > cfg.bytes {
            bytes.len() - cfg.bytes
        } else {
            0
        };
        let slice = &bytes[start..];
        let s = String::from_utf8_lossy(slice);
        print!("{}", s);
    } else {
        let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
        let start = if lines.len() > cfg.lines {
            lines.len() - cfg.lines
        } else {
            0
        };
        for n in start..lines.len() {
            println!("{}", lines[n]);
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
