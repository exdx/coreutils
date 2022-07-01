use clap::{App, Arg};
use std::process;
use utils;

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
                .takes_value(false),
        )
        .arg(
            Arg::with_name("color")
                .short("c")
                .help("Color formatting. Use lowercase, for example green")
                .default_value(""),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    let input_color = matches.value_of("color").unwrap();

    let color = match utils::colors::get_color(input_color) {
        None => {
            // TODO: write to stderr
            println!("Error: unrecognized color selected: {}", input_color);
            process::exit(1);
        }
        Some(color) => color,
    };

    let output = format!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
    let string = utils::colors::build_colored_string(output.as_str(), color);
    print!("{}", string);
}
