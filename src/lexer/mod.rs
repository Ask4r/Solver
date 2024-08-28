use crate::tokens::Token;
use errors::AnalyseError;
use std::f64;

pub mod errors;

#[cfg(test)]
mod tests;

pub fn analyse<'src>(source: &'src str) -> LexerIterator<'src> {
    LexerIterator {
        source,
        pos: 0,
        ch: source.as_bytes()[0],
        prev_tok: None,
    }
}

struct LexerIterator<'src> {
    source: &'src str,
    pos: usize,
    ch: u8,
    prev_tok: Option<Token<'src>>,
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
}

impl<'src> Iterator for LexerIterator<'src> {
    type Item = Result<Token<'src>, AnalyseError<'src>>;

    fn next(&mut self) -> Option<Self::Item> {
        use AnalyseError::*;
        use Token::*;
        self.skip_whitespace();

        let tok = match self.ch {
            b'+' => Add { pos: self.pos },
            b'-' => match self.prev_tok {
                Some(tok) if tok.is_operand() || matches!(tok, RParen { .. }) => {
                    Sub { pos: self.pos }
                }
                _ => UM { pos: self.pos },
            },
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
                    Err(_) => return Some(Err(WrongNumber { text, pos })),
                };
                self.prev_tok = Some(tok);
                return Some(Ok(tok));
            }
            b'A'..=b'Z' | b'a'..=b'z' | b'_' => {
                let pos = self.pos;
                while self.ch.is_ascii_alphabetic() || self.ch.is_ascii_digit() || self.ch == b'_' {
                    self.read_ch();
                }
                let text = &self.source[pos..self.pos];
                let tok = match Token::from_text(text, pos) {
                    Some(tok) => tok,
                    None => return Some(Err(UnknownIdent { text, pos })),
                };
                self.prev_tok = Some(tok);
                return Some(Ok(tok));
            }
            0 => return None,
            _ => {
                return Some(Err(UnknownSymbol {
                    text: &self.source[self.pos..self.pos + 1],
                    pos: self.pos,
                }))
            }
        };

        self.read_ch();
        self.prev_tok = Some(tok);
        return Some(Ok(tok));
    }
}
