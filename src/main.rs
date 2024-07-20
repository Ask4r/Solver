use colors::bold_red;
use lexer::Lexer;

pub mod colors;
mod lexer;

fn main() {
    let text = "2 * (4 - log(5 * x + x ^ 2))";

    let mut lexer = Lexer::new(text);
    let tokens = match lexer.analyse() {
        Ok(tokens) => tokens,
        Err(e) => return println!("{}: {}", bold_red("Error"), e),
    };

    println!("Text: `{}`", text);
    println!("Tokens: {:?}", tokens);
}
