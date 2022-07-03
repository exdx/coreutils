use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("head")
        .version("0.1.0")
        .about("head UNIX tool")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .default_value("-")
                .multiple(true)
                .help("File(s) to read from"),
        )
        .arg(
            Arg::with_name("line_count")
                .value_name("LINE COUNT")
                .default_value("10")
                .help("How many lines to display"),
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("How many characters to display"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: matches.value_of("line_count").unwrap().parse::<usize>()?,
        bytes: matches
            .value_of("chars")
            .and_then(|s| s.parse::<usize>().ok()),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}
