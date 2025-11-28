pub fn red(text: &str) -> String {
    format!("\x1b[31m{text}\x1b[0m")
}

pub fn blue(text: &str) -> String {
    format!("\x1b[34m{text}\x1b[0m")
}

pub fn green(text: &str) -> String {
    format!("\x1b[32m{text}\x1b[0m")
}

pub fn yellow(text: &str) -> String {
    format!("\x1b[33m{text}\x1b[0m")
}

pub fn magenta(text: &str) -> String {
    format!("\x1b[35m{text}\x1b[0m")
}
