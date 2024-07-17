use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process::exit;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .bin_name("headr")
        .version("0.1.0")
        .author("Kanahiro Iguchi")
        .about("hogehoge")
        .args([
            Arg::new("filename")
                .value_name("FILE")
                .help("filename to head")
                .default_values(["-"])
                .num_args(1..),
            Arg::new("lines")
                .value_name("LINES")
                .short('n')
                .long("lines")
                .default_value("10")
                .conflicts_with("bytes")
                .help("Number lines"),
            Arg::new("bytes")
                .value_name("BYTES")
                .short('c')
                .long("bytes")
                .help("Number bytes"),
        ])
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("filename")
            .unwrap()
            .map(|s| s.to_string())
            .collect(),
        lines: match matches.get_one::<String>("lines") {
            Some(lines) => match parse_positive_int(lines) {
                Ok(n) => n,
                Err(e) => {
                    eprintln!(
                        "error: {} for '--lines <LINES>': invalid digit found in string",
                        e
                    );
                    std::process::exit(1);
                }
            },
            _ => 10,
        },
        bytes: match matches.get_one::<String>("bytes") {
            Some(bytes) => match parse_positive_int(bytes) {
                Ok(n) => Some(n),
                Err(e) => {
                    eprintln!("{} for '--bytes <BYTES>': invalid digit found in string", e);
                    std::process::exit(1);
                }
            },
            _ => None,
        },
    })
}

pub fn run(cfg: Config) -> MyResult<()> {
    let multiple = cfg.files.len() > 1;
    let mut counter = 0;
    for file in cfg.files {
        if counter > 0 {
            println!();
        };
        if multiple {
            println!("==> {} <==", file);
        };
        match open(&file) {
            Ok(reader) => {
                let mut buf = BufReader::new(reader);

                match cfg.bytes {
                    Some(n) => {
                        let mut s = vec![];
                        buf.take(n as u64).read_to_end(&mut s)?;
                        print!("{}", String::from_utf8_lossy(&mut s));
                    }
                    None => {
                        for _ in 0..cfg.lines {
                            let mut s = vec![];
                            buf.read_until(b'\n', &mut s)?;
                            print!("{}", String::from_utf8_lossy(&mut s));
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("headr: {}: {}", file, e);
            }
        }
        counter += 1;
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(format!("invalid value '{}'", val).into()),
    }
}

#[test]
fn test_parse_positive_int() {
    assert_eq!(parse_positive_int("10").unwrap(), 10);
    assert!(parse_positive_int("a").is_err());
    assert!(parse_positive_int("0").is_err());
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
