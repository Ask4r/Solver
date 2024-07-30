use crate::tokens::Token;
use errors::AnalyseError;
use std::f64;

pub mod errors;

#[cfg(test)]
mod tests;

pub struct Lexer<'src> {
    source: &'src str,
    pos: usize,
    ch: u8,
    prev_tok: Option<Token<'src>>
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { source, pos: 0, ch: source.as_bytes()[0], prev_tok: None }
    }

    fn read_ch(&mut self) {
        if self.pos + 1 >= self.source.len() {
            self.ch = 0;
        } else {
            self.ch = self.source.as_bytes()[self.pos + 1];
        }
        self.pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_ch();
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Result<Token<'src>, AnalyseError<'src>>;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;
        self.skip_whitespace();

        let tok = match self.ch {
            b'+' => Add { pos: self.pos },
            b'-' => match self.prev_tok {
                Some(Number { .. } | RParen { .. } | Var { .. } | Const { .. })
                => Sub { pos: self.pos },
                _ => UM { pos: self.pos }
            }
            b'*' => Mul { pos: self.pos },
            b'/' => Div { pos: self.pos },
            b'^' => Pow { pos: self.pos },
            b'(' => LParen { pos: self.pos },
            b')' => RParen { pos: self.pos },
            b',' => Comma { pos: self.pos },
            b'0'..=b'9' | b'.' => {
                let pos = self.pos;
                while self.ch.is_ascii_digit() || self.ch == b'.' {
                    self.read_ch();
                }
                let text = &self.source[pos..self.pos];
                let tok = match text.parse::<f64>() {
                    Ok(value) => Number { text, value, pos },
                    Err(_) => return Some(Err(AnalyseError::new(
                        "could not read number", self.source, text, pos))),
                };
                self.prev_tok = Some(tok);
                return Some(Ok(tok));
            }
            b'A'..=b'Z' | b'a'..=b'z' | b'_' => {
                let pos = self.pos;
                while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
                    self.read_ch();
                }
                let text = &self.source[pos..self.pos];
                let tok = match text {
                    "x" => Var { pos },
                    "sin" => Func { text, func: f64::sin, pos },
                    "cos" => Func { text, func: f64::cos, pos },
                    "exp" => Func { text, func: f64::exp, pos },
                    "ln" => Func { text, func: f64::ln, pos },
                    "sqrt" => Func { text, func: f64::sqrt, pos },
                    "e" => Const { text, value: f64::consts::E, pos },
                    "pi" => Const { text, value: f64::consts::PI, pos },
                    _ => return Some(Err(AnalyseError::new(
                        "unknown ident", self.source, text, pos))),
                };
                self.prev_tok = Some(tok);
                return Some(Ok(tok));
            }
            0 => return None,
            _ => return Some(Err(AnalyseError::new(
                "unknown symbol", self.source, &self.source[self.pos..self.pos + 1], self.pos))),
        };

        self.read_ch();
        self.prev_tok = Some(tok);
        return Some(Ok(tok));
    }
}

