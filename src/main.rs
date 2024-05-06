use lexer::Lexer;

mod naive_grammer;
mod prelude;
mod lexer;
mod token;

fn main() {
    let test: &[u8] = b"hey! how are you?";
    let mut lexer = Lexer::new(test);

    let tokens = lexer.into_tokens();

    dbg!(tokens);
}

