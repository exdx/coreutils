pub mod colors {
    use colored::Colorize;
    use colored::{Color, ColoredString};

    // TODO: refactor to use Color::FromStr()
    pub fn get_color(color: &str) -> Option<Color> {
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

    pub fn build_colored_string(string: &str, color: Color) -> ColoredString {
        Colorize::color(string, color)
    }
}
