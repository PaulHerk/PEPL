use crate::converter::*;
use crate::lexer::*;
use std::collections::HashMap;
use std::io::stdin;

pub struct Interpreter {
    tokens: Vec<Token>,
    tacks: HashMap<u32, Vec<u32>>,
    position_counter: u32,
}

impl Interpreter {
    pub fn interpret(mut self) {
        loop {
            if self.position_counter == self.tokens.len() as u32 {
                break;
            }
            let current_token = self.tokens.get(self.position_counter as usize).unwrap();
            // TODO: index for errors

            match current_token.token_kind {
                TokenKind::NewTack => {
                    let key = self.get_item(current_token, 0);

                    self.tacks.insert(key, Vec::new());
                }
                TokenKind::NewItem => {
                    let key = self.get_item(current_token, 0);
                    let value = self.get_item(current_token, 1);

                    if let Some(tack) = self.tacks.get_mut(&key) {
                        tack.push(value);
                    } else {
                        self.tacks.insert(key, Vec::new());
                        let tack = self.tacks.get_mut(&key).unwrap();
                        tack.push(value);
                    }
                }
                TokenKind::DelItem => {
                    let key = self.get_item(current_token, 0);

                    if let Some(tack) = self.tacks.get_mut(&key) {
                        tack.pop();
                    }
                }
                TokenKind::Increase => {
                    let key = self.get_item(current_token, 0);

                    if let Some(tack) = self.tacks.get_mut(&key) {
                        *tack.last_mut().unwrap() += 1;
                    }
                }
                TokenKind::Decrease => {
                    let key = self.get_item(current_token, 0);

                    if let Some(tack) = self.tacks.get_mut(&key) {
                        *tack.last_mut().unwrap() -= 1;
                    }
                }
                TokenKind::Output => {
                    let key = self.get_item(current_token, 0);
                    if let Some(tack) = self.tacks.get(&key) {
                        print!("{}", char::from_u32(*tack.last().unwrap()).unwrap());
                    }
                }
                TokenKind::Input => {
                    let mut input = String::new();
                    stdin().read_line(&mut input).unwrap();
                    let input_unicode = input.chars().nth(0).unwrap() as u32; // this decodes a char nto unicode

                    let key = self.get_item(current_token, 0);

                    if let Some(tack) = self.tacks.get_mut(&key) {
                        tack.push(input_unicode);
                    }
                }
                TokenKind::BeginIf(if_loop_indexes) => {
                    let value1 = self
                        .tacks
                        .get(&self.get_item(current_token, 0))
                        .unwrap()
                        .last()
                        .unwrap();
                    let value2 = self
                        .tacks
                        .get(&self.get_item(current_token, 1))
                        .unwrap()
                        .last()
                        .unwrap();

                    let index_of_endif_or_else = if value1 == value2 {
                        self.position_counter as usize
                    } else {
                        self.tokens
                            .iter()
                            .position(|token| {
                                token.token_kind == TokenKind::EndIf(if_loop_indexes)
                                    || token.token_kind == TokenKind::Else(if_loop_indexes)
                            })
                            .unwrap()
                    };
                    self.position_counter = index_of_endif_or_else as u32;
                }
                TokenKind::Else(if_loop_indexes) => {
                    let index_of_endif = self
                        .tokens
                        .iter()
                        .position(|token| token.token_kind == TokenKind::EndIf(if_loop_indexes))
                        .unwrap();

                    self.position_counter = index_of_endif as u32;
                }
                TokenKind::BreakLoop(loop_index) => {
                    if let Some(loop_index) = loop_index {
                        let index_of_begin_loop = self
                            .tokens
                            .iter()
                            .position(|token| token.token_kind == TokenKind::StartLoop(loop_index))
                            .unwrap();
                        self.position_counter = index_of_begin_loop as u32;
                    } else if let None = loop_index {
                        let item = self.get_item(current_token, 0);
                        let index_of_end_loop = self
                            .tokens
                            .iter()
                            .position(|token| token.token_kind == TokenKind::BreakLoop(Some(item)))
                            .unwrap();
                        self.position_counter = index_of_end_loop as u32;
                    }
                }
                _ => (),
                // dont need IfEnd since it doesnt do anything to the Tack (neither read or write)
            }
            self.position_counter += 1;
        }
    }

    fn get_item(&self, token: &Token, value_index: u8) -> u32 {
        let token_value = &token.values[value_index as usize];
        let item: u32 = match token_value.is_reference {
            true => {
                let tack_reference =
                    &decode(token_value.value.as_str().to_string(), "01".to_string());
                *self.tacks.get(tack_reference).unwrap().last().unwrap()
            } //                                  ^ TODO: Tack not existing
            false => match token_value.value_kind {
                ValueKind::Hex => decode(
                    token_value.value.as_str().to_string(),
                    "0123456789abcdef".to_string(),
                ),
                ValueKind::Dec => token_value.value.as_str().parse().unwrap(),
                ValueKind::Bin => decode(token_value.value.as_str().to_string(), "01".to_string()),
            },
        };
        item
    }

    pub fn new_interpreter(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            tacks: HashMap::new(),
            position_counter: 0,
        }
    }
}
