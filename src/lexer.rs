use crate::{naive_grammer::{Prefix, Suffix, Word}, token::Token};

pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { input, position: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.position += 1;
        }
    }

    pub fn into_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(char) = self.peek() {
            let char = match char {
                b' ' => {
                    tokens.push(Token::Whitespace);
                    self.advance();
                    continue;
                }
                b'.' => {
                    tokens.push(Token::Period);
                    self.advance();
                    continue;
                }
                b',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                    continue;
                }
                b'!' => {
                    tokens.push(Token::ExclamationMark);
                    self.advance();
                    continue;
                }
                b'?' => {
                    tokens.push(Token::QuestionMark);
                    self.advance();
                    continue;
                }
                x => x,
            };

            let mut text = Vec::new();
            text.push(char);
            self.advance();

            while let Some(char) = self.peek() {
                match char {
                    b' ' => break,
                    b'.' => break,
                    b',' => break,
                    b'!' => break,
                    b'?' => break,
                    _ => {}
                }

                self.advance();

                text.push(char);
            }

            let text = String::from_utf8_lossy(&text);

            match text.parse::<Prefix>() {
                Ok(x) => {
                    tokens.push(Token::Prefix(x));
                    continue;
                }
                Err(_) => {
                    if let Ok(x) = text.parse::<Suffix>() {
                        tokens.push(Token::Suffix(x));
                        continue;
                    } else if let Ok(x) = text.parse::<Word>() {
                        tokens.push(Token::Word(x));
                    }
                }
            }
        }

        tokens
    }
}

