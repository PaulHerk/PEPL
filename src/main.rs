use std::fs;
mod converter;
mod interpreter;
mod lexer;

use crate::{interpreter::Interpreter, lexer::Lexer};

fn main() {
    let file_content: String = fs::read_to_string("main.pepl").expect("No valid file");
    let mut lexer: Lexer = Lexer::new_lexer(file_content);
    let lexer = lexer.lex();
    match lexer {
        Ok(lexer) => {
            println!("{:?}", lexer);
            let interpreter = Interpreter::new_interpreter(lexer);
            match interpreter.interpret() {
                Err(error) => println!("{:?}", error),
                _ => (),
            }
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }
}
