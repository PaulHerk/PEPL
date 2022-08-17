pub(crate) mod error;
pub use crate::interpreter::error::*;
use crate::lexer::*;

use std::collections::HashMap;
use std::io::stdin;

use self::error::ErrorOnInterpreter;
use crate::options::Options;

pub struct Interpreter {
    tokens: Vec<Token>,
    tacks: HashMap<u32, Vec<u32>>,
    pub position_counter: u32,
}

impl Interpreter {
    pub fn interpret(mut self, options: Options) -> Result<u8, ErrorOnInterpreter> {
        loop {
            if options.debug {
                println!("{:?}", self.tacks)
            }

            if self.position_counter == self.tokens.len() as u32 {
                break;
            }
            let current_token = self.tokens.get(self.position_counter as usize).unwrap();

            let (key, value) = (
                self.get_value(current_token, 0)?,
                self.get_value(current_token, 1)?,
            );

            match current_token.token_kind {
                TokenKind::NewTack => {
                    self.tacks.insert(key, Vec::new());
                }
                TokenKind::NewItem => {
                    if let Some(tack) = self.tacks.get_mut(&key) {
                        tack.push(value);
                    } else {
                        self.tacks.insert(key, Vec::new());
                        let tack = self.get_tack_mut(key)?;
                        tack.push(value);
                    }
                }
                TokenKind::DelItem => {
                    self.get_item(key)?; // if no item in tack
                    self.get_tack_mut(key)?.pop().unwrap();
                }
                TokenKind::Increase => {
                    if *self.get_item_mut(key)? == 4294967295 {
                        // biggest u32 number, TODO: make it flexible
                        *self.get_item_mut(key)? = 0;
                    } else {
                        *self.get_item_mut(key)? += 1;
                    }
                }
                TokenKind::Decrease => {
                    if *self.get_item_mut(key)? == 0 {
                        *self.get_item_mut(key)? = 4294967295;
                    } else {
                        *self.get_item_mut(key)? -= 1;
                    }
                }
                TokenKind::Output => {
                    let item = self.get_item(key)?;
                    let character = match char::from_u32(item) {
                        Some(char) => char,
                        None => {
                            return Err(ErrorOnInterpreter::new_error(
                                ErrorkindOnInterpreter::NoSuchCharacter,
                                self.position_counter,
                            ))
                        }
                    };
                    print!("{}", character);
                }
                TokenKind::Input => {
                    let mut input = String::new();
                    stdin().read_line(&mut input).unwrap(); // TODO: change input
                    let input_unicode = input.chars().nth(0).unwrap() as u32; // this decodes a char into unicode

                    if let Some(tack) = self.tacks.get_mut(&key) {
                        tack.push(input_unicode);
                    }
                }
                TokenKind::BeginIf(if_loop_indexes) => {
                    let value1 = self.get_item(key)?;
                    let value2 = self.get_item(value)?;

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
                TokenKind::BreakLoop(loop_index) => match loop_index {
                    Some(loop_index) => {
                        let index_of_begin_loop = self
                            .tokens
                            .iter()
                            .position(|token| token.token_kind == TokenKind::StartLoop(loop_index))
                            .unwrap();
                        self.position_counter = index_of_begin_loop as u32;
                    }
                    None => {
                        let item = key;
                        let index_of_end_loop = self
                            .tokens
                            .iter()
                            .position(|token| token.token_kind == TokenKind::BreakLoop(Some(item)))
                            .unwrap();
                        self.position_counter = index_of_end_loop as u32;
                    }
                },
                _ => (),
                // don't need IfEnd since it doesn't do anything to the Tack (neither read or write)
            }
            self.position_counter += 1;
        }
        Ok(0)
    }

    fn get_tack_mut(&mut self, key: u32) -> Result<&mut Vec<u32>, ErrorOnInterpreter> {
        let tack_wrapped = self.tacks.get_mut(&key);
        match tack_wrapped {
            Some(tack) => Ok(tack),
            None => {
                return Err(ErrorOnInterpreter::new_error(
                    ErrorkindOnInterpreter::TackNotFound,
                    self.position_counter,
                ))
            }
        }
    }
    fn get_item_mut(&mut self, key: u32) -> Result<&mut u32, ErrorOnInterpreter> {
        let tack_wrapped = self.tacks.get_mut(&key);
        match tack_wrapped {
            Some(tack) => match tack.last_mut() {
                Some(item) => Ok(item),
                None => {
                    return Err(ErrorOnInterpreter::new_error(
                        ErrorkindOnInterpreter::NoItemInTack,
                        self.position_counter,
                    ))
                }
            },
            None => {
                return Err(ErrorOnInterpreter::new_error(
                    ErrorkindOnInterpreter::TackNotFound,
                    self.position_counter,
                ))
            }
        }
    }

    fn get_item(&self, key: u32) -> Result<u32, ErrorOnInterpreter> {
        let tack_wrapped = self.tacks.get(&key);
        match tack_wrapped {
            Some(tack) => match tack.last() {
                Some(&item) => Ok(item),
                None => {
                    return Err(ErrorOnInterpreter::new_error(
                        ErrorkindOnInterpreter::NoItemInTack,
                        self.position_counter,
                    ))
                }
            },
            None => {
                return Err(ErrorOnInterpreter::new_error(
                    ErrorkindOnInterpreter::TackNotFound,
                    self.position_counter,
                ))
            }
        }
    } // TODO: fix DRY

    fn get_value(&self, token: &Token, value_index: u8) -> Result<u32, ErrorOnInterpreter> {
        let token_value = token.values.get(value_index as usize);
        if let None = token_value {
            return Ok(0);
        }
        let token_value = token_value.unwrap();
        let value: Result<u32, ErrorOnInterpreter> = match token_value.is_reference {
            true => {
                let tack_reference =
                    &self.decode(token_value.value.as_str().to_string(), "01".to_string())?;
                return self.get_item(*tack_reference);
            }
            false => match token_value.value_kind {
                ValueKind::Hex => self.decode(
                    token_value.value.as_str().to_string(),
                    "0123456789abcdef".to_string(),
                ),
                ValueKind::Dec => self.decode(
                    token_value.value.as_str().to_string(),
                    "0123456789".to_string(),
                ), // decoding decimal to decimal here bc I don't have to do error handling twice
                ValueKind::Bin => {
                    self.decode(token_value.value.as_str().to_string(), "01".to_string())
                }
            },
        };
        value
    }

    pub fn new_interpreter(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            tacks: HashMap::new(),
            position_counter: 0,
        }
    }
}
