use crate::converter::*;
use crate::lexer::Token;
use std::collections::HashMap;

pub struct Interpreter {
    tokens: Vec<Token>,
    tacks: HashMap<String, Vec<String>>,
}

impl Interpreter {
    pub fn interpret(mut self) {
        for (index, current_token) in self.tokens.iter().enumerate() {
            match current_token {
                _ => panic!(),
            }
        }
    }

    pub fn new_interpreter(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            tacks: HashMap::new(),
        }
    }
}
