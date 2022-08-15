use std::fs;
mod converter;
mod interpreter;
mod lexer;

use crate::{interpreter::Interpreter, lexer::Lexer};

fn main() {
    let file_content = fs::read_to_string("main.pepl").expect("No valid file");
    let mut lexer = Lexer::new_lexer(file_content.chars()); // using Chars bc they are slightly faster :)
    let lexer = lexer.lex();
    match lexer {
        Ok(lexer) => {
            // println!("{:?}", lexer);
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
