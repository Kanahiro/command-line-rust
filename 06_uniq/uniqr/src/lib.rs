use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};

#[derive(Debug)]
pub struct Config {
    input: String,
    output: String,
    count: bool,
    repeated: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("uniqr")
        .bin_name("uniqr")
        .version("0.1.0")
        .author("Kanahiro Iguchi")
        .about("hogehoge")
        .args([
            Arg::new("input")
                .value_name("INPUT")
                .help("filepath to read")
                .default_values(["-"]),
            Arg::new("output")
                .value_name("OUTPUT")
                .help("filepath to output")
                .default_values(["-"]),
            Arg::new("count")
                .value_name("COUNT")
                .short('c')
                .long("count")
                .action(ArgAction::SetTrue)
                .help("show counts"),
            Arg::new("repeated")
                .value_name("REPEATED")
                .short('d')
                .long("repeated")
                .action(ArgAction::SetTrue)
                .help("show only dupilicated lines"),
        ])
        .get_matches();

    Ok(Config {
        input: matches.get_one::<String>("input").unwrap().to_string(),
        output: matches.get_one::<String>("output").unwrap().to_string(),
        count: *matches.get_one::<bool>("count").unwrap(),
        repeated: *matches.get_one::<bool>("repeated").unwrap(),
    })
}

pub fn run(cfg: Config) -> MyResult<()> {
    let mut buf = match open(&cfg.input) {
        Ok(buf) => buf,
        Err(err) => {
            eprintln!("{}: {}", cfg.input, err);
            std::process::exit(1);
        }
    };

    let mut streak: isize = 0;
    let mut prev: String = String::new();

    let file = File::create(cfg.output.clone())?;
    let mut writer = BufWriter::new(file);
    let mut _writefn = |content: &str| {
        writer.write(content.as_bytes());
    };

    loop {
        let mut v = vec![];
        buf.read_until(b'\n', &mut v)?;
        let line = String::from_utf8(v).unwrap();

        if line.is_empty() {
            break;
        }

        if streak == 0 {
            prev = line.clone();
            streak = 1;
            continue;
        }

        if prev.trim_end() == line.trim_end() {
            streak += 1;
        } else {
            let content = if cfg.count {
                format!("{:4} {}", streak, prev)
            } else {
                prev.clone()
            };

            if cfg.output == "-" {
                print!("{}", content);
            } else {
                _writefn(&content);
            }

            streak = 1;
            prev = line.clone();
        }
    }

    if streak > 0 {
        let content = if cfg.count {
            format!("{:4} {}", streak, prev)
        } else {
            prev.clone()
        };

        if cfg.output == "-" {
            print!("{}", content);
        } else {
            _writefn(&content);
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
