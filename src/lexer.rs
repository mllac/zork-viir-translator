use crate::{
    naive_grammer::{Prefix, Suffix, Word},
    token::{IntermediateToken, Symbol, Token},
};

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

    pub fn into_intermediate_tokens(&mut self) -> Vec<IntermediateToken> {
        let mut tokens = Vec::new();

        while let Some(char) = self.peek() {
            let char = match char {
                b' ' => {
                    tokens.push(IntermediateToken::Symbol(Symbol::Whitespace));
                    self.advance();
                    continue;
                }
                b'.' => {
                    tokens.push(IntermediateToken::Symbol(Symbol::Period));
                    self.advance();
                    continue;
                }
                b',' => {
                    tokens.push(IntermediateToken::Symbol(Symbol::Comma));
                    self.advance();
                    continue;
                }
                b'!' => {
                    tokens.push(IntermediateToken::Symbol(Symbol::ExclamationMark));
                    self.advance();
                    continue;
                }
                b'?' => {
                    tokens.push(IntermediateToken::Symbol(Symbol::QuestionMark));
                    self.advance();
                    continue;
                }
                b'\'' => {
                    tokens.push(IntermediateToken::Symbol(Symbol::Apostrophe));
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
                    b'\'' => break,
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
            tokens.push(IntermediateToken::Text(text.to_string()));
        }

        tokens
    }

    pub fn into_final_tokens(tokens: Vec<IntermediateToken>) -> Vec<Token> {
        let mut new_tokens = Vec::new();

        fn check_prefix(x: &str) -> Option<Prefix> {
            x.parse::<Prefix>().map_or(None, Some)
        }

        fn check_suffix(x: &str) -> Option<Suffix> {
            x.parse::<Suffix>().map_or(None, Some)
        }

        fn check_word(x: &str) -> Option<Word> {
            x.parse::<Word>().map_or(None, Some)
        }

        for (i, token) in tokens.iter().enumerate() {
            let text = if let IntermediateToken::Text(txt) = token {
                txt
            } else if let IntermediateToken::Symbol(s) = token {
                match s {
                    Symbol::Apostrophe => {
                        if let Some(x) = tokens.get(i - 1) {
                            if let IntermediateToken::Text(x) = x {
                                if let Ok(x) = x.parse::<Word>() {
                                    match x {
                                        Word::Fop => continue,
                                        _ => {}
                                    }
                                }
                            }
                        }

                        new_tokens.push(Token::Symbol(Symbol::Apostrophe));
                    }
                    s => new_tokens.push(Token::Symbol(s.clone())),
                };

                continue;
            } else {
                continue;
            };

            let word = match (check_prefix(&text), check_suffix(&text), check_word(&text)) {
                (None, None, None) => {
                    new_tokens.push(Token::None(text.to_string()));
                    continue;
                }
                (Some(x), None, Some(z)) => {
                    new_tokens.push(Token::Prefix(x));
                    new_tokens.push(Token::Word(z));
                    continue;
                }
                (None, Some(x), Some(y)) => {
                    new_tokens.push(Token::Word(y));
                    new_tokens.push(Token::Suffix(x));
                    continue;
                }
                (Some(x), None, None) => {
                    new_tokens.push(Token::Prefix(x));
                    continue;
                }
                (None, Some(x), None) => {
                    new_tokens.push(Token::Suffix(x));
                    continue;
                }
                (None, None, Some(x)) => x,
                _ => continue,
            };

            match word {
                Word::Ror => {
                    if let Some(x) = tokens.get(i + 1) {
                        match x {
                            IntermediateToken::Symbol(Symbol::QuestionMark) => {
                                new_tokens.push(Token::Word(Word::Zorp));
                            }
                            _ => {
                                new_tokens.push(Token::Word(Word::Ror));
                            }
                        }
                    } else {
                        new_tokens.push(Token::Word(Word::Ror));
                    }
                }
                _ => new_tokens.push(Token::Word(word)),
            }
        }

        new_tokens
    }
}
