use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .bin_name("echor")
        .version("0.1.0")
        .author("Kanahiro Iguchi")
        .about("hogehoge")
        .args([
            Arg::new("text")
                .value_name("TEXT")
                .help("text to echo")
                .required(true)
                .num_args(1..),
            Arg::new("omit_newline")
                .short('n')
                .help("do not output the trailing newline")
                .action(ArgAction::SetTrue),
        ])
        .get_matches();

    let texts = match matches.get_many::<String>("text") {
        Some(texts) => texts
            .into_iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>(),
        _ => {
            eprintln!("error");
            std::process::exit(1);
        }
    };

    let omit_newline: bool = match matches.get_one::<bool>("omit_newline") {
        Some(omit_newline) => *omit_newline,
        _ => {
            eprintln!("error");
            std::process::exit(1);
        }
    };
    print!(
        "{}{}",
        texts.join(" "),
        if omit_newline { "" } else { "\n" }
    );
}
