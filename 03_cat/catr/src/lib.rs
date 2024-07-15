use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .bin_name("catr")
        .version("0.1.0")
        .author("Kanahiro Iguchi")
        .about("hogehoge")
        .args([
            Arg::new("filename")
                .value_name("FILE")
                .help("filename to cat")
                .default_values(["-"])
                .num_args(1..),
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("Number lines")
                .conflicts_with("number_nonblank_lines")
                .action(ArgAction::SetTrue),
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("Number no-blank lines")
                .action(ArgAction::SetTrue),
        ])
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("filename")
            .unwrap()
            .into_iter()
            .map(|f| f.to_string())
            .collect(),
        number_lines: *matches.get_one("number_lines").unwrap(),
        number_nonblank_lines: *matches.get_one("number_nonblank_lines").unwrap(),
    })
}

pub fn run(cfg: Config) -> MyResult<()> {
    for filename in cfg.files {
        match open(&filename) {
            Err(e) => eprintln!("Error: {}: {}", filename, e),
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut line_num = 0;
                for line in reader.lines() {
                    let line = line?;
                    let prefix = {
                        if cfg.number_lines {
                            line_num += 1;
                            format!("     {}	", line_num)
                        } else if cfg.number_nonblank_lines {
                            if line.is_empty() {
                                "".to_string()
                            } else {
                                line_num += 1;
                                format!("     {}	", line_num)
                            }
                        } else {
                            "".to_string()
                        }
                    };
                    println!("{}{}", prefix, line);
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
