use crate::tokens::Token;
use errors::UnknownSymbolError;
use std::f64;

pub mod errors;

#[cfg(test)]
mod tests;

pub struct Lexer {
    source: Vec<u8>,
    pos: usize,
    read_pos: usize,
    ch: u8,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lex = Self {
            source: input.into_bytes(),
            pos: 0,
            read_pos: 0,
            ch: 0,
            tokens: Vec::new(),
        };
        lex.read_ch();
        return lex;
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, UnknownSymbolError> {
        let mut token;
        loop {
            token = self.next_token()?;
            if matches!(token, Token::EOF) { break }
            self.tokens.push(token.clone());
        }
        Ok(self.tokens.clone())
    }

    fn next_token(&mut self) -> Result<Token, UnknownSymbolError> {
        use Token::*;
        self.skip_whitespace();

        let tok = match self.ch {
            b'+' => Plus { pos: self.pos },
            b'-' => match self.tokens.last() {
                Some(Number { .. } | RParen { .. } | Var { .. } | Const { .. })
                    => Minus { pos: self.pos },
                _ => UM { pos: self.pos }
            },
            b'*' => Star { pos: self.pos },
            b'/' => Slash { pos: self.pos },
            b'^' => Caret { pos: self.pos },
            b'(' => LParen { pos: self.pos },
            b')' => RParen { pos: self.pos },
            b',' => Comma { pos: self.pos },
            b'0'..=b'9' | b'.' => {
                let text = self.read_number();
                let size = text.len();
                return Ok(match text.parse::<f64>() {
                    Ok(value) => Number { text, value, pos: self.pos - size },
                    Err(_) => return Err(self.error_unknown_symbol(text)),
                });
            }
            b'A'..=b'Z' | b'a'..=b'z' | b'_' => {
                let text = self.read_ident();
                let size = text.len();
                return Ok(match text.as_str() {
                    "x" => Var { pos: self.pos - size },
                    "sin" => Func { text, func: f64::sin, pos: self.pos - size },
                    "cos" => Func { text, func: f64::cos, pos: self.pos - size },
                    "exp" => Func { text, func: f64::exp, pos: self.pos - size },
                    "ln" => Func { text, func: f64::ln, pos: self.pos - size },
                    "sqrt" => Func { text, func: f64::sqrt, pos: self.pos - size },
                    "e" => Const { text, value: std::f64::consts::E, pos: self.pos - size },
                    "pi" => Const { text, value: std::f64::consts::PI, pos: self.pos - size },
                    _ => return Err(self.error_unknown_symbol(text)),
                });
            }
            0 => EOF,
            ch => return Err(self.error_unknown_symbol(ch.to_string())),
        };

        self.read_ch();
        Ok(tok)
    }

    fn read_ch(&mut self) {
        if self.read_pos >= self.source.len() {
            self.ch = 0;
        } else {
            self.ch = self.source[self.read_pos];
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_ch();
        }
    }

    fn read_number(&mut self) -> String {
        let pos = self.pos;
        while self.ch.is_ascii_digit() || self.ch == b'.' {
            self.read_ch();
        }
        String::from_utf8_lossy(&self.source[pos..self.pos]).to_string()
    }

    fn read_ident(&mut self) -> String {
        let pos = self.pos;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_ch();
        }
        String::from_utf8_lossy(&self.source[pos..self.pos]).to_string()
    }

    fn error_unknown_symbol(&self, symbol: String) -> UnknownSymbolError {
        UnknownSymbolError {
            source: String::from_utf8_lossy(&self.source).to_string(),
            pos: self.pos - symbol.len(),
            symbol,
        }
    }
}

