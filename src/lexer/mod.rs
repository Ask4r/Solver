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
            let (kind, size) = match self.next_token() {
                Ok(t) => t,
                Err(LexerError::EOF) => return Ok(tokens),
                Err(LexerError::Unknown(ch)) => {
                    return Err(UnknownSymbolError {
                        source: self.text,
                        pos: self.pos,
                        symbol: ch,
                    })
                }
            };
            if !matches!(kind, TokenType::Space) {
                tokens.push(Token {
                    kind,
                    pos: self.pos,
                });
            }
            self.pos += size;
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
