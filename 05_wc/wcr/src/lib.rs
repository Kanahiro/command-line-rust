use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    chars: bool,
    bytes: bool,
    words: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .bin_name("wcr")
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
                .short('l')
                .long("lines")
                .action(ArgAction::SetTrue)
                .help("show lines"),
            Arg::new("chars")
                .value_name("CHARS")
                .short('m')
                .long("chars")
                .action(ArgAction::SetTrue)
                .conflicts_with("bytes")
                .help("show character count"),
            Arg::new("bytes")
                .value_name("BYTES")
                .short('c')
                .long("bytes")
                .action(ArgAction::SetTrue)
                .help("show bytes"),
            Arg::new("words")
                .value_name("WORDS")
                .short('w')
                .long("words")
                .action(ArgAction::SetTrue)
                .help("show word count"),
        ])
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("filename")
            .unwrap()
            .map(|s| s.to_string())
            .collect(),
        lines: *matches.get_one("lines").unwrap(),
        chars: *matches.get_one("chars").unwrap(),
        bytes: *matches.get_one("bytes").unwrap(),
        words: *matches.get_one("words").unwrap(),
    })
}

pub fn run(cfg: Config) -> MyResult<()> {
    let multiple = cfg.files.len() > 1;
    let mut sum_lines = 0;
    let mut sum_chars = 0;
    let mut sum_bytes = 0;
    let mut sum_words = 0;

    for filename in cfg.files {
        match open(&filename) {
            Err(e) => eprintln!("Error: {}: {}", filename, e),
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut s = String::new();
                reader.read_to_string(&mut s)?;

                let lines = s.lines().count();
                let chars = s.chars().count();
                let bytes = s.len();
                let words = s.split_whitespace().count();

                sum_lines += lines;
                sum_chars += chars;
                sum_bytes += bytes;
                sum_words += words;

                let prefix = {
                    let mut s = String::new();
                    if !cfg.lines && !cfg.chars && !cfg.bytes && !cfg.words {
                        s.push_str(&format!("{:8}{:8}{:8}", lines, words, bytes));
                    } else {
                        if cfg.lines {
                            s.push_str(&format!("{:8}", lines));
                        }
                        if cfg.words {
                            s.push_str(&format!("{:8}", words));
                        }
                        if cfg.bytes {
                            s.push_str(&format!("{:8}", bytes));
                        }
                        if cfg.chars {
                            s.push_str(&format!("{:8}", chars));
                        }
                    }
                    s
                };

                let suffix = match filename.as_str() {
                    "-" => String::new(),
                    _ => format!(" {}", filename),
                };

                println!("{}{}", prefix, suffix);
            }
        }
    }
    if multiple {
        let sum = {
            let mut s = String::new();
            if !cfg.lines && !cfg.chars && !cfg.bytes && !cfg.words {
                s.push_str(&format!("{:8}{:8}{:8}", sum_lines, sum_words, sum_bytes));
            } else {
                if cfg.lines {
                    s.push_str(&format!("{:8}", sum_lines));
                }
                if cfg.words {
                    s.push_str(&format!("{:8}", sum_words));
                }
                if cfg.bytes {
                    s.push_str(&format!("{:8}", sum_bytes));
                }
                if cfg.chars {
                    s.push_str(&format!("{:8}", sum_chars));
                }
            }
            s
        };
        println!("{} total", sum);
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
