use rust_interperter::lexer::lex::{Lexer, Token, TokenType};

fn main() {
    let mut lex = Lexer::new("123");
    let expected = Token {
        token_type: TokenType::INT(123),
        literal: "123".into(),
    };

    println!("{:?} {:?}", expected, lex.next_token());
}
