pub(crate) mod types;

pub use crate::lexer::types::*;
#[derive(Debug)]
pub struct Token {
    token_kind: TokenKind,
    literal: String,
    values: Vec<ValueKind>,
}

impl Token {
    pub fn new_token(token_kind: TokenKind, literal: String) -> Self {
        Self {
            token_kind,
            literal,
            values: Vec::new(),
        }
    }
}

pub struct Lexer {
    contents: String,
    position_counter: u32,
}

impl Lexer {
    pub fn new_lexer(contents: String) -> Self {
        Self {
            contents,
            position_counter: 0,
        }
    }
    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let mut values: Vec<(String, bool)> = Vec::new();
            let literal = self.get_whole_literal();
            let mut bare_literal = String::new(); //like !++ instead of for example !+10+&1
            let mut token_kind: TokenKind;
            let mut values_with_kind: Vec<ValueKind> = Vec::new();

            for (index, &current_char) in literal.iter().enumerate() {
                if COMMAND_CHARACTERS.contains(&current_char) {
                    bare_literal.push(current_char);
                    let next_char = literal[index + 1];
                    if literal[index + 1] == '&' || POSSIBLE_VALUES.contains(&next_char) {
                        values.push((String::new(), false));
                    }
                } else if current_char == '&' {
                    values.last_mut().unwrap().1 = true;
                } else if POSSIBLE_VALUES.contains(&current_char) {
                    values.last_mut().unwrap().0.push(current_char);
                } else if current_char == '\\' {
                    break;
                }
            }

            (token_kind, values_with_kind) = match bare_literal.as_str() {
                "!+" => (
                    TokenKind::NewTack,
                    vec![ValueKind::Bin {
                        value: values.first().unwrap().0.as_str().to_string(),
                        is_reference: values.first().unwrap().1,
                    }],
                ),
                "!++" => (
                    TokenKind::NewItem,
                    vec![
                        ValueKind::Bin {
                            value: values.first().unwrap().0.as_str().to_string(),
                            is_reference: values.first().unwrap().1,
                        },
                        ValueKind::Hex {
                            value: values.last().unwrap().0.as_str().to_string(),
                            is_reference: values.last().unwrap().1,
                        },
                    ],
                ),
                "!-" => (
                    TokenKind::DelItem,
                    vec![ValueKind::Bin {
                        value: values.first().unwrap().0.as_str().to_string(),
                        is_reference: values.first().unwrap().1,
                    }],
                ),
                "+" => (
                    TokenKind::Increase,
                    vec![ValueKind::Bin {
                        value: values.first().unwrap().0.as_str().to_string(),
                        is_reference: values.first().unwrap().1,
                    }],
                ),
                "-" => (
                    TokenKind::Decrease,
                    vec![ValueKind::Bin {
                        value: values.first().unwrap().0.as_str().to_string(),
                        is_reference: values.first().unwrap().1,
                    }],
                ),
                ",>" => (
                    TokenKind::Output,
                    vec![ValueKind::Bin {
                        value: values.first().unwrap().0.as_str().to_string(),
                        is_reference: values.first().unwrap().1,
                    }],
                ),
                ",<" => (
                    TokenKind::Input,
                    vec![ValueKind::Bin {
                        value: values.first().unwrap().0.as_str().to_string(),
                        is_reference: values.first().unwrap().1,
                    }],
                ),
                "?:" => (
                    TokenKind::BeginIf,
                    vec![
                        ValueKind::Bin {
                            value: values.first().unwrap().0.as_str().to_string(),
                            is_reference: values.first().unwrap().1,
                        },
                        ValueKind::Bin {
                            value: values.last().unwrap().0.as_str().to_string(),
                            is_reference: values.last().unwrap().1,
                        },
                    ],
                ),
                ":?" => (
                    TokenKind::ReverseIf,
                    vec![
                        ValueKind::Bin {
                            value: values.first().unwrap().0.as_str().to_string(),
                            is_reference: values.first().unwrap().1,
                        },
                        ValueKind::Bin {
                            value: values.last().unwrap().0.as_str().to_string(),
                            is_reference: values.last().unwrap().1,
                        },
                    ],
                ),
                "?|" => (TokenKind::EndIf, Vec::new()),
                ">" => (
                    TokenKind::StartLoop,
                    vec![ValueKind::Dec {
                        value: values.first().unwrap().0.as_str().to_string(),
                        is_reference: values.first().unwrap().1,
                    }],
                ),
                "<" => (
                    TokenKind::EndLoop,
                    vec![ValueKind::Dec {
                        value: values.first().unwrap().0.as_str().to_string(),
                        is_reference: values.first().unwrap().1,
                    }],
                ),
                _ => panic!(),
            };
            let literal = literal.into_iter().collect();

            tokens.push(Token {
                token_kind,
                literal,
                values: values_with_kind,
            });

            if self.position_counter == self.contents.len().try_into().unwrap() {
                break;
            }
            // tokens.push(Token::new_token(todo!(), whole_literal.iter().collect()));
        }
        tokens
    }

    fn current_char(&self) -> char {
        self.contents
            .chars()
            .nth(self.position_counter.try_into().unwrap())
            .unwrap()
            .to_ascii_lowercase() //since people write in hex sometimes "abcdef" sometimes "ABCDEF"
    }

    fn get_whole_literal(&mut self) -> Vec<char> {
        let mut literal: Vec<char> = Vec::new();
        loop {
            let current_char = self.current_char();
            self.position_counter += 1;
            if current_char != '\\' && current_char != '\n' {
                literal.push(current_char);
            } else if current_char == '\\' {
                literal.push(current_char);
                return literal;
            }
        }
    }
}
