pub mod lexer;

use lexer::*;

fn main() {
    let mut lexer = Lexer::new("()()()()");
    loop {
        match lexer.next_token() {
            Ok(TokenType::EOF) => break,
            Ok(tok) => println!("{0:?}", tok),
            Err(err) => println!("{0:?}", err),
        }
    }
}
