use crate::converter::*;
pub(crate) mod error;
pub use crate::interpreter::error::*;
use crate::lexer::*;

use std::collections::HashMap;
use std::io::stdin;

use self::error::ErrorOnInterpreter;

pub struct Interpreter {
    tokens: Vec<Token>,
    tacks: HashMap<u32, Vec<u32>>,
    position_counter: u32,
}

impl Interpreter {
    pub fn interpret(mut self) -> Result<u8, ErrorOnInterpreter> {
        let mut errors: Vec<ErrorOnInterpreter> = Vec::new();
        'break_if_error: loop {
            if self.position_counter == self.tokens.len() as u32 {
                break;
            }
            let current_token = self.tokens.get(self.position_counter as usize).unwrap();

            let (key_wrapped, value_wrapped) = (
                self.get_item(current_token, 0),
                self.get_item(current_token, 1),
            );
            let (mut key, mut value);
            if let Err(kind) = key_wrapped {
                errors.push(ErrorOnInterpreter::new_error(kind, self.position_counter));
                break 'break_if_error;
            }
            key = key_wrapped.unwrap();

            if let Err(kind) = value_wrapped {
                errors.push(ErrorOnInterpreter::new_error(kind, self.position_counter));
                break 'break_if_error;
            }
            value = value_wrapped.unwrap();

            match current_token.token_kind {
                TokenKind::NewTack => {
                    self.tacks.insert(key, Vec::new());
                }
                TokenKind::NewItem => {
                    if let Some(tack) = self.tacks.get_mut(&key) {
                        tack.push(value);
                    } else {
                        self.tacks.insert(key, Vec::new());
                        let tack = self.tacks.get_mut(&key).unwrap();
                        tack.push(value);
                    }
                }
                TokenKind::DelItem => {
                    if let Some(tack) = self.tacks.get_mut(&key) {
                        tack.pop();
                    }
                }
                TokenKind::Increase => {
                    if let Some(tack) = self.tacks.get_mut(&key) {
                        *tack.last_mut().unwrap() += 1;
                    }
                }
                TokenKind::Decrease => {
                    if let Some(tack) = self.tacks.get_mut(&key) {
                        *tack.last_mut().unwrap() -= 1;
                    }
                }
                TokenKind::Output => {
                    if let Some(tack) = self.tacks.get(&key) {
                        print!("{}", char::from_u32(*tack.last().unwrap()).unwrap());
                    }
                }
                TokenKind::Input => {
                    let mut input = String::new();
                    stdin().read_line(&mut input).unwrap();
                    let input_unicode = input.chars().nth(0).unwrap() as u32; // this decodes a char into unicode

                    if let Some(tack) = self.tacks.get_mut(&key) {
                        tack.push(input_unicode);
                    }
                }
                TokenKind::BeginIf(if_loop_indexes) => {
                    let value1 = self.tacks.get(&key).unwrap().last().unwrap();
                    let value2 = self.tacks.get(&value).unwrap().last().unwrap();

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
                        let item = key;
                        let index_of_end_loop = self
                            .tokens
                            .iter()
                            .position(|token| token.token_kind == TokenKind::BreakLoop(Some(item)))
                            .unwrap();
                        self.position_counter = index_of_end_loop as u32;
                    }
                }
                _ => (),
                // don't need IfEnd since it doesn't do anything to the Tack (neither read or write)
            }
            self.position_counter += 1;
        }
        if errors.is_empty() {
            Ok(0)
        } else {
            Err(*errors.first().unwrap())
        }
    }

    fn get_item(&self, token: &Token, value_index: u8) -> Result<u32, ErrorkindOnInterpreter> {
        let token_value = token.values.get(value_index as usize);
        if let None = token_value {
            return Ok(0);
        }
        let token_value = token_value.unwrap();
        let item: Result<u32, ErrorkindOnInterpreter> = match token_value.is_reference {
            true => {
                let tack_reference =
                    &decode(token_value.value.as_str().to_string(), "01".to_string())?;
                let tack_wrapped = self.tacks.get(tack_reference);
                match tack_wrapped {
                    Some(tack) => match tack.last() {
                        Some(&item) => Ok(item),
                        None => return Err(ErrorkindOnInterpreter::NoItemInTack),
                    },
                    None => return Err(ErrorkindOnInterpreter::TackNotFound),
                }
            }
            false => match token_value.value_kind {
                ValueKind::Hex => decode(
                    token_value.value.as_str().to_string(),
                    "0123456789abcdef".to_string(),
                ),
                ValueKind::Dec => decode(
                    token_value.value.as_str().to_string(),
                    "0123456789".to_string(),
                ), // decoding decimal to decimal here bc I don't have to do error handling twice
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
