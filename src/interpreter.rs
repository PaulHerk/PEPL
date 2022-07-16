pub use crate::lexer::types::TokenKind;
pub use crate::lexer::Token;
use std::collections::HashMap;

pub struct Interpreter {
    content: Vec<Token>,
    tacks: HashMap<String, Vec<String>>,
}

impl Interpreter {
    pub fn interpret(&mut self) {
        for token in self.content.iter() {
            let token_tack: String = token.tack.clone();
            match token.kind {
                TokenKind::NewTack => {
                    self.tacks.insert(token_tack.to_owned(), Vec::new());
                }
                TokenKind::NewItem => {
                    self.tacks
                        .get_mut(&token_tack)
                        .unwrap_or_else(|| panic!())
                        .push(token.extra.clone());
                }
                TokenKind::DelItem => {
                    self.tacks
                        .get_mut(&token_tack)
                        .unwrap_or_else(|| panic!())
                        .pop();
                }
                TokenKind::Increase => {}
                TokenKind::Decrease => {}
                TokenKind::Output => {}
                TokenKind::Input => {}
                TokenKind::BeginIf => {}
                TokenKind::Else => {}
                TokenKind::EndIf => {}
                TokenKind::StartLoop => {}
                TokenKind::EndLoop => {}
            }
        }
        println!("{:?}", self.tacks);
    }
    fn check_if_reference(&mut self, token: Token) -> String {
        if token.reference {
            "test".to_owned()
        } else {
            let mut_reference_from_tack = self.tacks.get_mut(&token.tack).unwrap();
            mut_reference_from_tack
                .get(mut_reference_from_tack.len())
                .unwrap()
                .to_string()
        }
    }

    pub fn new_interpreter(content: Vec<Token>) -> Self {
        Self {
            content,
            tacks: HashMap::new(),
        }
    }
}
