use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cat")
        .version("0.1.0")
        .about("cat UNIX command")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .default_value("-")
                .help("File to read from"),
        )
        .arg(
            Arg::with_name("number-lines")
                .short("n")
                .takes_value(false)
                .help("Print line numbers"),
        )
        .arg(
            Arg::with_name("number-nonblank-lines")
                .short("b")
                .takes_value(false)
                .help("Print only non-blank line numbers"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number-lines"),
        number_nonblank_lines: matches.is_present("number-nonblank-lines"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}
