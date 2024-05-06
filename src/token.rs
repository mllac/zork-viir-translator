use crate::naive_grammer::{Prefix, Suffix, Word};

#[derive(Debug, Clone)]
pub enum Symbol {
    ExclamationMark,
    QuestionMark,
    Apostrophe,
    Whitespace,
    Period,
    Comma,
}

#[derive(Debug, Clone)]
pub enum Token {
    Symbol(Symbol),
    Suffix(Suffix),
    Prefix(Prefix),
    None(String),
    Word(Word),
}

#[derive(Debug, Clone)]
pub enum IntermediateToken {
    Symbol(Symbol),
    Text(String),
}
