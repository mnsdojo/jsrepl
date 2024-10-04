//color.rs

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const ITALIC: &str = "\x1b[3m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";

pub trait Colorize {
    fn green(&self) -> String;
    fn yellow(&self) -> String;
    fn bold(&self) -> String;
    fn italic(&self) -> String;
}

impl Colorize for &str {
    fn green(&self) -> String {
        format!("{}{}{}", GREEN, self, RESET)
    }

    fn yellow(&self) -> String {
        format!("{}{}{}", YELLOW, self, RESET)
    }

    fn bold(&self) -> String {
        format!("{}{}{}", BOLD, self, RESET)
    }

    fn italic(&self) -> String {
        format!("{}{}{}", ITALIC, self, RESET)
    }
}

impl Colorize for String {
    fn green(&self) -> String {
        self.as_str().green()
    }

    fn yellow(&self) -> String {
        self.as_str().yellow()
    }

    fn bold(&self) -> String {
        self.as_str().bold()
    }

    fn italic(&self) -> String {
        self.as_str().italic()
    }
}
