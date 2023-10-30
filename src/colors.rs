use std::fmt::Display;

///
/// this trait do nothing, it's purpose solely just to
/// provide blanket for Regular, Bold, Background and Special
///
pub trait Formatter: Display {
    fn s(&self) -> String {
        self.to_string()
    }
}

impl Formatter for Regular {}
impl Formatter for Bold {}
impl Formatter for Background {}
impl Formatter for Underline {}
impl Formatter for Special {}

/// this enum provide regular color format for terminal
pub enum Regular { BLACK, RED, GREEN, YELLOW, BLUE, PURPLE, GRBLUE, GRAY, WHITE }
impl Display for Regular {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s;
        match self {
            Self::BLACK => s = "\x1B[30m",
            Self::RED => s = "\x1B[31m",
            Self::GREEN => s = "\x1B[32m",
            Self::YELLOW => s = "\x1B[33m",
            Self::BLUE => s = "\x1B[34m",
            Self::PURPLE => s = "\x1B[35m",
            Self::GRBLUE => s = "\x1B[36m",
            Self::GRAY => s = "\x1B[37m",
            Self::WHITE => s = "\x1B[38m"
        }
        write!(f, "{s}")
    }
}
impl<T: AsRef<str>> From<T> for Regular {
    fn from(value: T) -> Self {
        match value.as_ref().to_lowercase().as_str() {
            "red" => Self::RED,
            "green" => Self::GREEN,
            "yellow" => Self::YELLOW,
            "blue" => Self::BLUE,
            "purple" => Self::PURPLE,
            "grblue" => Self::GRBLUE,
            "gray" => Self::GRAY,
            "white" => Self::WHITE,
            _ => Self::BLACK
        }
    }
}

/// this enum provide color+bold format for terminal
pub enum Bold { BLACK, RED, GREEN, YELLOW, BLUE, PURPLE, GRBLUE, GRAY, WHITE }
impl Display for Bold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s;
        match self {
            Self::BLACK => s = "\x1B[1;30m",
            Self::RED => s = "\x1B[1;31m",
            Self::GREEN => s = "\x1B[1;32m",
            Self::YELLOW => s = "\x1B[1;33m",
            Self::BLUE => s = "\x1B[1;34m",
            Self::PURPLE => s = "\x1B[1;35m",
            Self::GRBLUE => s = "\x1B[1;36m",
            Self::GRAY => s = "\x1B[1;37m",
            Self::WHITE => s = "\x1B[1;38m"
        }
        write!(f, "{s}")
    }
}
impl<T: AsRef<str>> From<T> for Bold {
    fn from(value: T) -> Self {
        match value.as_ref().to_lowercase().as_str() {
            "red" => Self::RED,
            "green" => Self::GREEN,
            "yellow" => Self::YELLOW,
            "blue" => Self::BLUE,
            "purple" => Self::PURPLE,
            "grblue" => Self::GRBLUE,
            "gray" => Self::GRAY,
            "white" => Self::WHITE,
            _ => Self::BLACK
        }
    }
}

/// this enum provide color+underline format for terminal
pub enum Underline { BLACK, RED, GREEN, YELLOW, BLUE, PURPLE, GRBLUE, GRAY, WHITE }
impl Display for Underline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s;
        match self {
            Self::BLACK => s = "\x1B[4;30m",
            Self::RED => s = "\x1B[4;31m",
            Self::GREEN => s = "\x1B[4;32m",
            Self::YELLOW => s = "\x1B[4;33m",
            Self::BLUE => s = "\x1B[4;34m",
            Self::PURPLE => s = "\x1B[4;35m",
            Self::GRBLUE => s = "\x1B[4;36m",
            Self::GRAY => s = "\x1B[4;37m",
            Self::WHITE => s = "\x1B[4;38m"
        }
        write!(f, "{s}")
    }
}
impl<T: AsRef<str>> From<T> for Underline {
    fn from(value: T) -> Self {
        match value.as_ref().to_lowercase().as_str() {
            "red" => Self::RED,
            "green" => Self::GREEN,
            "yellow" => Self::YELLOW,
            "blue" => Self::BLUE,
            "purple" => Self::PURPLE,
            "grblue" => Self::GRBLUE,
            "gray" => Self::GRAY,
            "white" => Self::WHITE,
            _ => Self::BLACK
        }
    }
}

/// this enum provide background color format for terminal
/// also adjust font color to black when background color is bright
pub enum Background { BLACK, RED, GREEN, YELLOW, BLUE, PURPLE, GRBLUE, GRAY, WHITE }
impl Display for Background {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s;
        match self {
            Self::BLACK => s = "\x1B[40m",
            Self::RED => s = "\x1B[30;41m",
            Self::GREEN => s = "\x1B[30;42m",
            Self::YELLOW => s = "\x1B[30;43m",
            Self::BLUE => s = "\x1B[30;44m",
            Self::PURPLE => s = "\x1B[30;45m",
            Self::GRBLUE => s = "\x1B[30;46m",
            Self::GRAY => s = "\x1B[30;47m",
            Self::WHITE => s = "\x1B[48m"
        }
        write!(f, "{s}")
    }
}
impl<T: AsRef<str>> From<T> for Background {
    fn from(value: T) -> Self {
        match value.as_ref().to_lowercase().as_str() {
            "red" => Self::RED,
            "green" => Self::GREEN,
            "yellow" => Self::YELLOW,
            "blue" => Self::BLUE,
            "purple" => Self::PURPLE,
            "grblue" => Self::GRBLUE,
            "gray" => Self::GRAY,
            "white" => Self::WHITE,
            _ => Self::BLACK
        }
    }
}

/// this enum provide other font format
pub enum Special { BOLD, DIM, ITALIC, UNDERLINE, BLINK, DEF }
impl Display for Special {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s;
        match self {
            Self::BOLD => s = "\x1B[1m",
            Self::DIM => s = "\x1B[2m",
            Self::ITALIC => s = "\x1B[3m",
            Self::UNDERLINE => s = "\x1B[4m",
            Self::BLINK => s = "\x1B[5m",
            Self::DEF => s = "\x1B[0m",
        }
        write!(f, "{s}")
    }
}
impl<T: AsRef<str>> From<T> for Special {
    fn from(value: T) -> Self {
        match value.as_ref().to_lowercase().as_str() {
            "bold" => Self::BOLD,
            "dim" => Self::DIM,
            "italic" => Self::ITALIC,
            "underline" => Self::UNDERLINE,
            "BLINK" => Self::BLINK,
            _ => Self::DEF,
        }
    }
}

