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
            Arg::with_name("line_count")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .default_value("10")
                .help("How many lines to display"),
        )
        .arg(
            Arg::with_name("chars")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .help("How many characters to display")
                .conflicts_with("lines"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .default_value("-")
                .multiple(true)
                .help("File(s) to read from"),
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

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse::<usize>() {
        Ok(u) if u > 0 => Ok(u),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string())
}
