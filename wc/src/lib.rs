use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wc")
        .version("0.1.0")
        .about("wc UNIX command")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .default_value("-")
                .multiple(true)
                .help("Files to read from"),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .help("Whether to print the line numbers")
                .short("l")
                .long("lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("words")
                .value_name("WORDS")
                .help("Whether to print the number of words")
                .short("w")
                .long("words")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Whether to print the number of bytes")
                .short("c")
                .long("bytes")
                .takes_value(false)
                .conflicts_with("chars"),
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("Whether to print the number of chars")
                .short("m")
                .long("chars")
                .takes_value(false),
        )
        .get_matches();

    let files: Vec<String> = matches.values_of_lossy("files").unwrap();
    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|arg| arg == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut counts: Vec<FileInfo> = Vec::new();
    for filename in &config.files {
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(data) => {
                match count(data) {
                    Err(e) => eprintln!("{}: {}", filename, e),
                    Ok(count) => {
                        // TODO print formatted file information here
                        counts.push(count);
                    }
                }
            }
        }
    }

    // TODO: sum together all counts to get summary

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut words_offset: bool = true;
    // Walk through the file one line at a time
    // Keep a running count of the variables defined above
    for byte in file.bytes() {
        match byte {
            Err(e) => eprintln!("Error reading bytes {}", e),
            Ok(b) => {
                num_bytes += 1;
                num_chars += 1;
                if b == b'\n' {
                    num_lines += 1;
                }
                if b == b' ' {
                    if words_offset {
                        words_offset = false;
                        num_words += 1;
                    }
                    num_words += 1;
                }
            }
        }
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
