use std::fs;
mod lexer;
pub use crate::lexer::Lexer;

fn main() {
    let file_content: String = fs::read_to_string("main.pepl").expect("No valid file");

    let mut lexer: Lexer = Lexer::new_lexer(file_content);
    lexer.lex();
}
