use clap::{App, Arg};
use colored::Colorize;

fn main() {
    let matches = App::new("echo")
        .version("0.1.0")
        .about("echo UNIX command")
        .arg(
            Arg::with_name("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
            .short("n")
            .help("Do not print newline")
            .takes_value(false)
    ).get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    print!("{}{}", text.join(" ").green(), if omit_newline {""} else {"\n"});
}