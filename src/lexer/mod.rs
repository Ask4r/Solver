use errors::{LexerError, UnknownSymbolError};
use once_cell::sync::Lazy;
use regex::Regex;
use tokens::{Token, TokenType};

mod errors;
mod tokens;

static NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d*\.)?\d+").unwrap());
static WORD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_]+").unwrap());

pub struct Lexer<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text, pos: 0 }
    }

    pub fn analyse(&mut self) -> Result<Vec<Token>, UnknownSymbolError> {
        let mut tokens = Vec::new();
        loop {
            match self.next_token() {
                Ok((kind, size)) => {
                    if !matches!(kind, TokenType::Space) {
                        tokens.push(Token {
                            kind,
                            pos: self.pos,
                        })
                    }
                    self.pos += size;
                }
                Err(LexerError::EOF) => return Ok(tokens),
                Err(LexerError::Unknown(ch)) => {
                    return Err(UnknownSymbolError {
                        source: self.text,
                        pos: self.pos,
                        symbol: ch,
                    })
                }
            }
        }
    }

    fn next_token(&self) -> Result<(TokenType<'a>, usize), LexerError> {
        let text = &self.text[self.pos..];

        if text.len() == 0 {
            return Err(LexerError::EOF);
        }

        if let Some(mat) = NUMBER_REGEX.find(text) {
            let kind = TokenType::Number(mat.as_str().parse::<f64>().unwrap());
            return Ok((kind, mat.len()));
        }

        if let Some(mat) = WORD_REGEX.find(text) {
            let kind = TokenType::Word(mat.as_str());
            return Ok((kind, mat.len()));
        }

        let kind = match text.chars().next().unwrap() {
            ch if ch.is_whitespace() => TokenType::Space,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Star,
            '/' => TokenType::Slash,
            '^' => TokenType::Caret,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            ch => return Err(LexerError::Unknown(ch)),
        };

        Ok((kind, 1))
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
            Token {
                kind: TokenType::Word("sin"),
                pos: 0,
            },
            Token {
                kind: TokenType::LParen,
                pos: 3,
            },
            Token {
                kind: TokenType::Word("x"),
                pos: 4,
            },
            Token {
                kind: TokenType::Caret,
                pos: 6,
            },
            Token {
                kind: TokenType::Number(2.0),
                pos: 8,
            },
            Token {
                kind: TokenType::RParen,
                pos: 9,
            },
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

        assert!(tok1.iter().eq(&vec![Token {
            kind: TokenType::Number(2.0),
            pos: 0,
        }]));
        assert!(tok2.iter().eq(&vec![Token {
            kind: TokenType::Number(0.14),
            pos: 0,
        }]));
        assert!(tok3.iter().eq(&vec![
            Token {
                kind: TokenType::Minus,
                pos: 0,
            },
            Token {
                kind: TokenType::Number(0.1),
                pos: 1,
            },
        ]));
    }
}
