use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
                .multiple(true)
                .help("File to read from"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .takes_value(false)
                .help("Print line numbers"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .takes_value(false)
                .help("Print only non-blank line numbers"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(file) => {
                // let line_start = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if config.number_lines {
                        if line.len() != 0 {
                            println! {"{}\t{}", line_num + 1, line}
                            continue;
                        }
                    }
                    if config.number_nonblank_lines {
                        println! {"{} \t{}", line_num + 1, line}
                        continue;
                    }
                    println!("{}", line)
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
