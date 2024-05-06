#![allow(dead_code)]

use std::str::FromStr;

#[derive(Debug)]
enum Word {
    Zorp,
    Zork,
    Worp,
    Corp,
    Now,
    Ror,
    Rio,
    T,
    Roop,
    Vlorp,
    Floom,
    Fleem,
    Mow,
    Wok,
    Sqock,
    Oom,
    Baak,
    Yopper,
    Wor,
    Viir,
    Lovia,
    Korop,
    Wompwomp,
    Womp,
    Coor,
    Yorp,
    Eep,
    Loomp,
    Vine,
    Bloo,
    Poropl,
    Perpoler,
    Dooleji,
    Cor,
    Clork,
    Blork,
    Plork,
    Koy,
    Hort,
    Skoo,
    Skorp,
    Jake,
    Bomp,
    Yolo,
    Choor,
    Lond,
    Porson,
    Sodgo,
    Skoop,
    Ajooz,
    K,
    Yorn,
    Jop,
    George,
    Toop,
    Ool,
    Woov,
    Bog,
    Cot,
    Boll,
    Boil,
    Shook,
    Woah,
    Plonk,
    Wower,
    Porf,
    Port,
    Mort,
    Morf,
    Zorf,
    Fi,
    Yop,
    Yopi,
    Tai,
    Fro,
    Fo,
    Fui,
    Fop,
    Hmm,
    Lor,
    Tonk,
    Por,
    Tirp,
    Torp,
    Tarp,
    Tworp,
    Vworp,
    Torz,
    Pol,
    Zol,
    Porc,
    Fot,
    Wolk,
    Conk,
    Holz,
    Tuch,
    Bon,
    Sorf,
    Comf,
    Oa,
    Grr,
    Yoft,
    Awaw,
    Uwu,
    Bron,
    Koor,
    Wot,
    Foor,
    Dorp,
    Voirz,
    Wook,
    Sos,
    Olp,
    Zoop,
    Blonk,
    Nom,
    Brod,
    Hoz,
    Foch,
    Ommog,
    Zoz,
    Po,
    Ni,
    Vol,
    Yot,
    Oot,
    Blorp,
    Som,
    Yiop,
    Oomp,
    Mop,
    Yor,
    Roy,
    Noy,
    Ord,
    Rorp,
    Ow,
    Trop,
    Goo,
    Lee,
    Konekshon,
    Woo,
}

#[derive(Debug)]
enum Modifier {
    Orp,
    Orz,
}

#[derive(Debug)]
enum Prefix {
    Rg,
    Rb,
}

#[derive(Debug)]
enum Suffix {
    Omp,
    Iop,
}

impl FromStr for Word {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "yes" => Ok(Self::Zorp),
            "hell" => Ok(Self::Zork),
            "no" => Ok(Self::Worp),
            "bitchass" => Ok(Self::Corp),
            "stick" => Ok(Self::Now),
            "right" => Ok(Self::Ror),
            "left" => Ok(Self::Rio),
            "side" => Ok(Self::T),
            "in" => Ok(Self::Roop),
            "what" => Ok(Self::Vlorp),
            "lot" => Ok(Self::Floom),
            "little" => Ok(Self::Floom),
            "cat" => Ok(Self::Mow),
            "dog" => Ok(Self::Wok),
            "bird" => Ok(Self::Sqock),
            "cow" => Ok(Self::Oom),
            "chicken" => Ok(Self::Baak),
            "person" => Ok(Self::Yopper),
            "human" => Ok(Self::Yopper),
            "country" => Ok(Self::Wor),
            "language" => Ok(Self::Viir),
            "favorite" => Ok(Self::Lovia),
            "alien" => Ok(Self::Korop),
            "too-bad" => Ok(Self::Wompwomp),
            "dang" => Ok(Self::Womp),
            "color" => Ok(Self::Coor),
            "red" => Ok(Self::Yorp),
            "orange" => Ok(Self::Eep),
            "green" => Ok(Self::Vine),
            "blue" => Ok(Self::Bloo),
            "purple" => Ok(Self::Poropl),
            "voilet" => Ok(Self::Perpoler),
            "semi" => Ok(Self::Dooleji),
            "see-through" => Ok(Self::Cor),
            "seethrough" => Ok(Self::Cor),
            "flower" => Ok(Self::Clork),
            "tree" => Ok(Self::Blork),
            "plant" => Ok(Self::Blork),
            "bush" => Ok(Self::Koy),
            "leaf" => Ok(Self::Hort),
            "sky" => Ok(Self::Skoo),
            "cloud" => Ok(Self::Skorp),
            "weather" => Ok(Self::Jake),
            "opacity" => Ok(Self::Bomp),
            "sun" => Ok(Self::Yolo),
            "awesome" => Ok(Self::Choor),
            "lovely" => Ok(Self::Lond),
            "sad" => Ok(Self::Porson),
            "upset" => Ok(Self::Sodgo),
            "angry" => Ok(Self::Skoop),
            "jealous" => Ok(Self::Skoop),
            "jealousy" => Ok(Self::Skoop),
            "cold" => Ok(Self::K),
            "hot" => Ok(Self::Yorn),
            "rain" => Ok(Self::Yop),
            "thuner" => Ok(Self::George),
            "tornado" => Ok(Self::Toop),
            "flood" => Ok(Self::Ool),
            "tsunami" => Ok(Self::Woov),
            "love" => Ok(Self::Bog),
            "like" => Ok(Self::Cot),
            "volcano" => Ok(Self::Boll),
            "eruption" => Ok(Self::Boil),
            "erupt" => Ok(Self::Boil),
            "explode" => Ok(Self::Boil),
            "boom" => Ok(Self::Boil),
            "earthquake" => Ok(Self::Shook),
            "hurricane" => Ok(Self::Woah),
            "wood" => Ok(Self::Plonk),
            "water" => Ok(Self::Wower),
            "she" => Ok(Self::Porf),
            "her" => Ok(Self::Port),
            "him" => Ok(Self::Mort),
            "he" => Ok(Self::Morf),
            "they" => Ok(Self::Zorf),
            "them" => Ok(Self::Zorf),
            "i" => Ok(Self::Fi),
            "im" => Ok(Self::Fi),
            "my" => Ok(Self::Fi),
            "you" => Ok(Self::Yop),
            "your" => Ok(Self::Yopi),
            "thy" => Ok(Self::Tai),
            "are" => Ok(Self::Fro),
            "is" => Ok(Self::Fo),
            "have" => Ok(Self::Fui),
            "been" => Ok(Self::Fui),
            "our" => Ok(Self::Fop),
            "we" => Ok(Self::Fop),
            "hmm" => Ok(Self::Hmm),
            "how" => Ok(Self::Hmm),
            "if" => Ok(Self::Lor),
            "logic" => Ok(Self::Tonk),
            "so" => Ok(Self::Por),
            "this" => Ok(Self::Tirp),
            "then" => Ok(Self::Torp),
            "that" => Ok(Self::Tarp),
            "there" => Ok(Self::Tworp),
            "where" => Ok(Self::Vworp),
            "here" => Ok(Self::Torz),
            "stop" => Ok(Self::Pol),
            "go" => Ok(Self::Zol),
            "continue" => Ok(Self::Porc),
            "will" => Ok(Self::Fot),
            "leg" => Ok(Self::Wolk),
            "arm" => Ok(Self::Conk),
            "hand" => Ok(Self::Holz),
            "finger" => Ok(Self::Tuch),
            "joints" => Ok(Self::Bon),
            "thigh" => Ok(Self::Sorf),
            "chest" => Ok(Self::Comf),
            "stomach" => Ok(Self::Oa),
            "hungry" => Ok(Self::Grr),
            "soft" => Ok(Self::Yoft),
            "rough" => Ok(Self::Awaw),
            "rigid" => Ok(Self::Uwu),
            "smooth" => Ok(Self::Bron),
            "holey" => Ok(Self::Koor),
            "hole-y" => Ok(Self::Koor),
            "confused" => Ok(Self::Wot),
            "fair" => Ok(Self::Foor),
            "dumb" => Ok(Self::Dorp),
            "odd" => Ok(Self::Voirz),
            "walk" => Ok(Self::Wook),
            "sit" => Ok(Self::Sos),
            "breathe" => Ok(Self::Olp),
            "look" => Ok(Self::Zoop),
            "blink" => Ok(Self::Blonk),
            "eat" => Ok(Self::Nom),
            "ate" => Ok(Self::Nom),
            "bread" => Ok(Self::Brod),
            "hold" => Ok(Self::Hoz),
            "bring" => Ok(Self::Foch),
            "hesitate" => Ok(Self::Ommog),
            "think" => Ok(Self::Ommog),
            "wonder" => Ok(Self::Ommog),
            "sleep" => Ok(Self::Zoz),
            "rest" => Ok(Self::Zoz),
            "do" => Ok(Self::Po),
            "and" => Ok(Self::Ni),
            "also" => Ok(Self::Ni),
            "pet" => Ok(Self::Vol),
            "own" => Ok(Self::Yot),
            "out" => Ok(Self::Oot),
            "time" => Ok(Self::Blorp),
            "some" => Ok(Self::Som),
            "while" => Ok(Self::Yiop),
            "almost" => Ok(Self::Oomp),
            "name" => Ok(Self::Mop),
            "good" => Ok(Self::Yor),
            "bad" => Ok(Self::Roy),
            "not" => Ok(Self::Noy),
            "word" => Ok(Self::Ord),
            "thing" => Ok(Self::Rorp),
            "oh" => Ok(Self::Ow),
            "die" => Ok(Self::Trop),
            "same" => Ok(Self::Goo),
            "need" => Ok(Self::Lee),
            "connection" => Ok(Self::Konekshon),
            "hey" => Ok(Self::Woo),
            "hi" => Ok(Self::Woo),
            "hello" => Ok(Self::Woo),
            "hai" => Ok(Self::Woo),
            _ => Err(()),
        }
    }
}

impl FromStr for Prefix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase().ends_with("rg") {
            Ok(Self::Rg)
        } else if s.ends_with("rb") {
            Ok(Self::Rb)
        } else {
            Err(())
        }
    }
}

impl FromStr for Suffix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase().ends_with("omp") {
            Ok(Self::Omp)
        } else if s.to_lowercase().ends_with("iop") {
            Ok(Self::Iop)
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
enum Token {
    ExclamationMark,
    Suffix(Suffix),
    Prefix(Prefix),
    QuestionMark,
    Whitespace,
    Word(Word),
    Period,
    Comma,
}

struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a [u8]) -> Self {
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

    fn into_tokens(&mut self) -> Vec<Token> {
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

fn main() {
    println!("Before translation: hey! how are you?");
    let c: &[u8] = b"hey! how are you?";

    let mut trs = Lexer::new(c);
    let tokens = trs.into_tokens();

    dbg!(tokens);
}
