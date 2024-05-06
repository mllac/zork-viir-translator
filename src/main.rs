use lexer::Lexer;

mod lexer;
mod naive_grammer;
mod prelude;
mod token;

fn main() {
    let test: &[u8] = b"crazy weather we've been having, right?";
    let mut lexer = Lexer::new(test);

    let tokens = lexer.into_intermediate_tokens();
    let tokens = Lexer::into_final_tokens(tokens);

    dbg!(tokens);
}
