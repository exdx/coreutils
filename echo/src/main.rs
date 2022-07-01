use std::fmt::Debug;

use clap::{App, Arg};
use colored::{Color, ColoredString, Colorize};

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

    let color = get_color(input_color);
    if color.is_err() {
        println!("Unrecognized color selected: {}", input_color)
    }

    let output = format!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
    let string = Colorize::color(output.as_str(), color.unwrap());
    println!("{}", string);
}

fn get_color(color: &str) -> Result<Color, ()> {
    match color {
        "" => Ok(Color::White),
        "black" => Ok(Color::Black),
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "yellow" => Ok(Color::Yellow),
        "blue" => Ok(Color::Blue),
        "magenta" => Ok(Color::Magenta),
        "purple" => Ok(Color::Magenta),
        "cyan" => Ok(Color::Cyan),
        "white" => Ok(Color::White),
        "bright black" => Ok(Color::BrightBlack),
        "bright red" => Ok(Color::BrightRed),
        "bright green" => Ok(Color::BrightGreen),
        "bright yellow" => Ok(Color::BrightYellow),
        "bright blue" => Ok(Color::BrightBlue),
        "bright magenta" => Ok(Color::BrightMagenta),
        "bright cyan" => Ok(Color::BrightCyan),
        "bright white" => Ok(Color::BrightWhite),
        _ => Err(()),
    }
}
