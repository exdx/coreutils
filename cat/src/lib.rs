use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;
use utils;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
    input_color: String,
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
                .long("number")
                .takes_value(false)
                .help("Print line numbers"),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .long("number-nonblank")
                .takes_value(false)
                .help("Print only non-blank line numbers"),
        )
        .arg(
            Arg::with_name("color")
                .short("c")
                .help("Color formatting. Use lowercase, for example green")
                .default_value(""),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
        input_color: matches.value_of("color").unwrap().to_string(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let color = match utils::colors::get_color(&config.input_color) {
        None => {
            eprintln!("Error: unrecognized color selected: {}", config.input_color);
            process::exit(1);
        }
        Some(color) => color,
    };

    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    let line = utils::colors::build_colored_string(&line, color);
                    if config.number_lines {
                        println! {"{:>6}\t{}", line_num + 1, line}
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println! {"{:>6}\t{}", last_num, line}
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line)
                    }
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
