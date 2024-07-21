use errors::{LexerError, UnknownSymbolError};
use once_cell::sync::Lazy;
use regex::Regex;
pub use tokens::TextToken;

pub mod errors;
mod tokens;

static NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d*\.?\d+").unwrap());
static WORD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_]+").unwrap());

pub struct Lexer<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text, pos: 0 }
    }

    pub fn analyse(&mut self) -> Result<Vec<TextToken>, UnknownSymbolError> {
        let mut tokens = Vec::new();
        loop {
            let token = match self.next_token() {
                Ok(token) => token,
                Err(LexerError::EOF) => return Ok(tokens),
                Err(LexerError::Unknown(ch)) => {
                    return Err(UnknownSymbolError {
                        source: self.text,
                        pos: self.pos,
                        symbol: ch,
                    })
                }
            };
            match token {
                TextToken::Space { .. } => self.pos += 1,
                TextToken::Word { text, .. } | TextToken::Number { text, .. } => {
                    tokens.push(token);
                    self.pos += text.len();
                }
                char_token => {
                    tokens.push(char_token);
                    self.pos += 1;
                }
            }
        }
    }

    fn next_token(&self) -> Result<TextToken<'a>, LexerError> {
        let text = &self.text[self.pos..];
        if text.len() == 0 {
            return Err(LexerError::EOF);
        }
        if let Some(mat) = NUMBER_REGEX.find(text) {
            return Ok(TextToken::Number {
                text: mat.as_str(),
                value: mat.as_str().parse::<f64>().unwrap(),
                pos: self.pos,
            });
        }
        if let Some(mat) = WORD_REGEX.find(text) {
            return Ok(TextToken::Word {
                text: mat.as_str(),
                pos: self.pos,
            });
        }
        match text.chars().next().unwrap() {
            ch if ch.is_whitespace() => Ok(TextToken::Space { pos: self.pos }),
            '+' => Ok(TextToken::Plus { pos: self.pos }),
            '-' => Ok(TextToken::Minus { pos: self.pos }),
            '*' => Ok(TextToken::Star { pos: self.pos }),
            '/' => Ok(TextToken::Slash { pos: self.pos }),
            '^' => Ok(TextToken::Caret { pos: self.pos }),
            '(' => Ok(TextToken::LParen { pos: self.pos }),
            ')' => Ok(TextToken::RParen { pos: self.pos }),
            ',' => Ok(TextToken::Comma { pos: self.pos }),
            ch => Err(LexerError::Unknown(ch)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "sin(x ^ 2)";
        let mut lexer = Lexer::new(text);
        let tokens = lexer.analyse().unwrap();
        let expected = vec![
            TextToken::Word {
                text: "sin",
                pos: 0,
            },
            TextToken::LParen { pos: 3 },
            TextToken::Word { text: "x", pos: 4 },
            TextToken::Caret { pos: 6 },
            TextToken::Number {
                text: "2",
                value: 2.0,
                pos: 8,
            },
            TextToken::RParen { pos: 9 },
        ];
        assert!(tokens.iter().eq(&expected));
    }

    #[test]
    fn it_parses_numbers() {
        let num1 = "2";
        let num2 = ".14";
        let num3 = "-0.1";

        let mut lex1 = Lexer::new(num1);
        let mut lex2 = Lexer::new(num2);
        let mut lex3 = Lexer::new(num3);

        let tok1 = lex1.analyse().unwrap();
        let tok2 = lex2.analyse().unwrap();
        let tok3 = lex3.analyse().unwrap();

        assert!(tok1.iter().eq(&vec![TextToken::Number {
            text: "2",
            value: 2.0,
            pos: 0,
        }]));
        assert!(tok2.iter().eq(&vec![TextToken::Number {
            text: ".14",
            value: 0.14,
            pos: 0,
        }]));
        assert!(tok3.iter().eq(&vec![
            TextToken::Minus { pos: 0 },
            TextToken::Number {
                text: "0.1",
                value: 0.1,
                pos: 1,
            },
        ]));
    }
}
