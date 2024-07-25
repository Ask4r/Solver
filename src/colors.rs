use std::fmt::Display;

pub fn green<T: Display>(text: T) -> String {
    format!("\x1B[32m{}\x1B[0m", text)
}
pub fn yellow<T: Display>(text: T) -> String {
    format!("\x1B[32m{}\x1B[0m", text)
}
pub fn cyan<T: Display>(text: T) -> String {
    format!("\x1B[36m{}\x1B[0m", text)
}


pub fn bold_white<T: Display>(text: T) -> String {
    format!("\x1B[1;29m{}\x1B[0m", text)
}
pub fn bold_gray<T: Display>(text: T) -> String {
    format!("\x1B[1;30m{}\x1B[0m", text)
}
pub fn bold_red<T: Display>(text: T) -> String {
    format!("\x1B[1;31m{}\x1B[0m", text)
}
pub fn bold_green<T: Display>(text: T) -> String {
    format!("\x1B[1;32m{}\x1B[0m", text)
}
pub fn bold_yellow<T: Display>(text: T) -> String {
    format!("\x1B[1;33m{}\x1B[0m", text)
}
pub fn bold_blue<T: Display>(text: T) -> String {
    format!("\x1B[1;34m{}\x1B[0m", text)
}
pub fn bold_magenta<T: Display>(text: T) -> String {
    format!("\x1B[1;35m{}\x1B[0m", text)
}
pub fn bold_cyan<T: Display>(text: T) -> String {
    format!("\x1B[1;36m{}\x1B[0m", text)
}

