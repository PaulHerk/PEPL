use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     pub fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }
mod converter;
mod interpreter;
mod lexer;
mod options;
use self::lexer::errors::ErrorOnLexer;
use self::options::Options;

// use options::Options;

use crate::{interpreter::{Interpreter, error::ErrorOnInterpreter}, lexer::Lexer};

#[wasm_bindgen]
pub struct Error{
    pub error_on_lexer: Option<ErrorOnLexer>,
    pub error_on_interpreter: Option<ErrorOnInterpreter>,
    pub error_kind: bool, // true for lexer, false for interpreter
}

impl Error {
    pub fn new_lexer_error(error_on_lexer: ErrorOnLexer) -> Self {
        Self { error_on_lexer: Some(error_on_lexer), error_on_interpreter: None , error_kind: true }
    }
    pub fn new_interpreter_error(error_on_interpreter: ErrorOnInterpreter) -> Self {
        Self { error_on_lexer: None, error_on_interpreter: Some(error_on_interpreter) , error_kind: false }
    }
}


#[wasm_bindgen]
pub fn main(content: String) -> Result<String, Error> {
        let mut lexer = Lexer::new_lexer(content.chars()); // using Chars bc they are slightly faster :)
        let lexer = lexer.lex();

        match lexer {
            Ok(lexer) => {
                let interpreter = Interpreter::new_interpreter(lexer);
                match interpreter.interpret(Options { debug: false }) {
                        Err(error_on_interpreter) => Err(Error::new_interpreter_error(error_on_interpreter)),
                        Ok(output) => Ok(output),
                    }
                }
                Err(error_on_lexer) => {
                    Err(Error::new_lexer_error(error_on_lexer))
            }
        }
}
