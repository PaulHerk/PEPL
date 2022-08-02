use std::fs;
// mod interpreter;
mod lexer;
// use interpreter::Interpreter;

pub use crate::lexer::Lexer;

fn main() {
    let file_content: String = fs::read_to_string("main.pepl").expect("No valid file");

    let mut lexer: Lexer = Lexer::new_lexer(file_content);
    let lexer = lexer.lex();
    println!("{:?}", lexer);
    // let mut interpreter = Interpreter::new_interpreter(lexer);
    // interpreter.interpret();
    // not yet :)
}
