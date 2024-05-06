use crate::naive_grammer::{Prefix, Suffix, Word};

#[derive(Debug)]
pub enum Token {
    ExclamationMark,
    Suffix(Suffix),
    Prefix(Prefix),
    QuestionMark,
    Whitespace,
    Word(Word),
    Period,
    Comma,
}
