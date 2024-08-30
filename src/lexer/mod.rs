use crate::tokens::{Token, TokenType};
use errors::{AnalyseError, AnalyseErrorType};
use parsers::{parse_ident, parse_number};

pub mod errors;
mod parsers;

#[cfg(test)]
mod tests;

pub fn analyse<'src>(source: &'src str) -> LexerIterator<'src> {
    LexerIterator {
        source,
        pos: 0,
        ch: source.as_bytes()[0],
    }
}

pub struct LexerIterator<'src> {
    source: &'src str,
    pos: usize,
    ch: u8,
}

impl<'src> LexerIterator<'src> {
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

    fn read_number(&mut self) -> &'src str {
        let pos = self.pos;
        while self.ch.is_ascii_digit() || self.ch == b'.' {
            self.read_ch();
        }
        return &self.source[pos..self.pos];
    }

    fn read_ident(&mut self) -> &'src str {
        let pos = self.pos;
        while self.ch.is_ascii_alphabetic() || self.ch.is_ascii_digit() || self.ch == b'_' {
            self.read_ch();
        }
        return &self.source[pos..self.pos];
    }
}

impl<'src> Iterator for LexerIterator<'src> {
    type Item = Result<Token<'src>, AnalyseError<'src>>;

    fn next(&mut self) -> Option<Self::Item> {
        use AnalyseErrorType::*;
        use TokenType::*;
        self.skip_whitespace();

        let token = match self.ch {
            b'+' => Token::new(self.pos, "+", Add),
            b'-' => Token::new(self.pos, "-", Sub),
            b'*' => Token::new(self.pos, "*", Mul),
            b'/' => Token::new(self.pos, "/", Div),
            b'^' => Token::new(self.pos, "^", Pow),
            b'(' => Token::new(self.pos, "(", LParen),
            b')' => Token::new(self.pos, ")", RParen),
            b',' => Token::new(self.pos, ",", Comma),
            b'0'..=b'9' | b'.' => {
                let text = self.read_number();
                return Some(match parse_number(text) {
                    Some(tok) => Ok(Token::new(self.pos - text.len(), text, tok)),
                    None => Err(AnalyseError::new(self.pos - text.len(), text, WrongNumber)),
                });
            }
            b'A'..=b'Z' | b'a'..=b'z' | b'_' => {
                let text = self.read_ident();
                return Some(match parse_ident(text) {
                    Some(tok) => Ok(Token::new(self.pos - text.len(), text, tok)),
                    None => Err(AnalyseError::new(self.pos - text.len(), text, UnknownIdent)),
                });
            }
            0 => return None,
            _ => {
                return Some(Err(AnalyseError::new(
                    self.pos,
                    &self.source[self.pos..self.pos + 1],
                    UnknownSymbol,
                )));
            }
        };

        self.read_ch();
        return Some(Ok(token));
    }
}
