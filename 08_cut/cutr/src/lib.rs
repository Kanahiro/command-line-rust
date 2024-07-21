use crate::Extract::{Bytes, Chars, Fields};
use clap::ArgGroup;
use clap::{builder::PossibleValue, Arg, ArgAction, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::{error::Error, ops::Range};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
}

type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("cutr")
        .bin_name("cutr")
        .version("0.1.0")
        .author("Kanahiro Iguchi")
        .about("hogehoge")
        .args([
            Arg::new("file")
                .value_name("FILE")
                .help("dir to find")
                .default_value("-")
                .num_args(1..),
            Arg::new("delimiter")
                .short('d')
                .long("delimiter")
                .value_name("DELIM"),
            Arg::new("fields")
                .short('f')
                .long("fields")
                .value_name("FIELDS")
                .conflicts_with_all(["bytes", "chars"]),
            Arg::new("bytes")
                .short('b')
                .long("bytes")
                .value_name("BYTES")
                .conflicts_with_all(["fields", "chars"]),
            Arg::new("chars")
                .short('c')
                .long("chars")
                .value_name("CHARS")
                .conflicts_with_all(["fields", "bytes"]),
        ])
        .group(
            ArgGroup::new("extract")
                .args(["fields", "bytes", "chars"])
                .required(true),
        )
        .get_matches();

    Ok(Config {
        files: match matches.get_many::<String>("file") {
            Some(files) => files.map(|s| s.to_string()).collect(),
            None => vec![],
        },
        delimiter: match matches.get_one::<String>("delimiter") {
            Some(delim) => {
                if delim.len() != 1 {
                    return Err(format!("--delim \"{}\" must be a single byte", delim).into());
                }
                delim.as_bytes()[0]
            }
            None => b'\t', // default delimiter
        },
        extract: match matches.get_one::<String>("fields") {
            Some(fields) => {
                let positions = parse_pos(fields.to_string());
                match positions {
                    Ok(positions) => Fields(positions),
                    Err(e) => return Err(e),
                }
            }
            None => match matches.get_one::<String>("bytes") {
                Some(bytes) => {
                    let positions = parse_pos(bytes.to_string());
                    match positions {
                        Ok(positions) => Bytes(positions),
                        Err(e) => return Err(e),
                    }
                }
                None => match matches.get_one::<String>("chars") {
                    Some(chars) => {
                        let positions = parse_pos(chars.to_string());
                        match positions {
                            Ok(positions) => Chars(positions),
                            Err(e) => return Err(e),
                        }
                    }
                    None => {
                        return Err("You must specify one of --fields, --bytes, or --chars".into())
                    }
                },
            },
        },
    })
}

pub fn run(cfg: Config) -> MyResult<()> {
    for file in cfg.files {
        match open(&file) {
            Ok(reader) => {
                let reader = BufReader::new(reader);
                for line in reader.lines() {
                    let line = line?;

                    match cfg.extract {
                        Fields(ref positions) => {
                            let segments: Vec<&str> = line.split(cfg.delimiter as char).collect();
                            for pos in positions {
                                let sliced = &segments[pos.clone()];
                                print!("{}", sliced.join(&(cfg.delimiter as char).to_string()));
                            }
                        }
                        Bytes(ref positions) => {
                            for pos in positions {
                                let segment = &line.as_bytes()[pos.clone()];
                                print!("{}", String::from_utf8_lossy(segment));
                            }
                        }
                        Chars(ref positions) => {
                            for pos in positions {
                                let start_idx = line.char_indices().nth(pos.start);
                                let end_idx = line.char_indices().nth(pos.end);
                                if start_idx.is_none() || end_idx.is_none() {
                                    eprintln!("{}: invalid range", file);
                                    continue;
                                };
                                print!("{}", &line[start_idx.unwrap().0..end_idx.unwrap().0]);
                            }
                        }
                    }
                    println!();
                }
            }
            Err(e) => eprintln!("{}: {}", file, e),
        }
    }

    Ok(())
}

fn parse_pos(range: String) -> MyResult<PositionList> {
    let positions = range
        .split(',')
        .map(|s| {
            if s.contains("+") {
                return Err(format!("illegal list value: {:?}", s));
            }

            let range: Vec<&str> = s.split('-').collect();
            if range.len() > 2 {
                return Err(format!("illegal list value: {:?}", s));
            }

            let start = match range[0].parse() {
                Ok(0) => return Err(format!("illegal list value: \"0\"")),
                Ok(n) => n,
                Err(_) => return Err(format!("illegal list value: {:?}", s)),
            };
            let end = if range.len() == 1 {
                start
            } else {
                match range[1].parse::<usize>() {
                    Ok(n) => {
                        if n <= start {
                            return Err(format!(
                                "First number in range ({}) must be lower than second number ({})",
                                start, n
                            ));
                        }
                        n
                    }
                    Err(_) => return Err(format!("illegal list value: {:?}", s)),
                }
            };
            Ok(start - 1..end)
        })
        .collect::<Result<PositionList, _>>()?;
    Ok(positions)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[cfg(test)]
mod unit_tests {
    use super::parse_pos;

    #[test]
    fn test_parse_pos() {
        // The empty string is an error
        assert!(parse_pos("".to_string()).is_err());

        // Zero is an error
        let res = parse_pos("0".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0""#);

        let res = parse_pos("0-1".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0""#);

        // A leading "+" is an error
        let res = parse_pos("+1".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "+1""#,);

        let res = parse_pos("+1-2".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "+1-2""#,
        );

        let res = parse_pos("1-+2".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "1-+2""#,
        );

        // Any non-number is an error
        let res = parse_pos("a".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#);

        let res = parse_pos("1,a".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#);

        let res = parse_pos("1-a".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "1-a""#,);

        let res = parse_pos("a-1".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a-1""#,);

        // Wonky ranges
        let res = parse_pos("-".to_string());
        assert!(res.is_err());

        let res = parse_pos(",".to_string());
        assert!(res.is_err());

        let res = parse_pos("1,".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-1-1".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-1-a".to_string());
        assert!(res.is_err());

        // First number must be less than second
        let res = parse_pos("1-1".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );

        let res = parse_pos("2-1".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );

        // All the following are acceptable
        let res = parse_pos("1".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("01".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("1,3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001,0003".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("1-3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("0001-03".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("1,7,3-5".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15,19-20".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }
}
