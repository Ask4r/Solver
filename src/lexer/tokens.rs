#[derive(Debug)]
pub enum TokenType<'a> {
    Space,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Number(f64),
    Word(&'a str),
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenType<'a>,
    pub pos: usize,
}
