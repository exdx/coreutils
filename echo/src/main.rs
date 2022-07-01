use clap::{App, Arg};
use colored::{Color, Colorize};
use std::io::{stderr, stdin, Read, Result, Write};
use std::process;

fn main() -> Result<()> {
    let matches = App::new("echo")
        .version("0.1.0")
        .about("echo UNIX command")
        .arg(Arg::with_name("text").value_name("TEXT").help("Input text"))
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

    let text = matches.values_of_lossy("text");
    let omit_newline = matches.is_present("omit_newline");
    let input_color = matches.value_of("color").unwrap();

    let mut buffer = String::new();
    let stdin = stdin().read_to_string(&mut buffer);
    match stdin {
        Err(_) => {
            stderr().write(b"Error: reading from stdin")?;
            process::exit(1);
        }
        Ok(_) => (),
    }
    if buffer.len() == 0 && text.is_none() {
        stderr().write(b"Error: no text provided: supply text directly or via stdin")?;
        process::exit(1);
    }

    // prefer stdin  over any provided data
    let input = if buffer.len() != 0 {
        buffer
    } else {
        text.unwrap().join(" ")
    };

    let output = format!("{}{}", input, if omit_newline { "" } else { "\n" });

    let color = match get_color(input_color) {
        None => {
            stderr().write(b"Error: unrecognized color selected")?;
            process::exit(1);
        }
        Some(c) => c,
    };

    let string = Colorize::color(output.as_str(), color);
    print!("{}", string);

    Ok(())
}

// TODO: refactor to use Color::FromStr()
fn get_color(color: &str) -> Option<Color> {
    match color {
        "" => Some(Color::White),
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "purple" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        "bright black" => Some(Color::BrightBlack),
        "bright red" => Some(Color::BrightRed),
        "bright green" => Some(Color::BrightGreen),
        "bright yellow" => Some(Color::BrightYellow),
        "bright blue" => Some(Color::BrightBlue),
        "bright magenta" => Some(Color::BrightMagenta),
        "bright cyan" => Some(Color::BrightCyan),
        "bright white" => Some(Color::BrightWhite),
        _ => None,
    }
}
