pub(crate) mod types;

pub use crate::lexer::types::*;
#[derive(Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    literal: String,
    pub values: Vec<Value>,
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
        let mut if_loop_indexes: u32 = 0; // since you don't have curly brackets I have to put an index to that
        let mut loop_indexes: u32 = 0; // for normal loop; same as above

        loop {
            let mut values: Vec<(String, bool)> = Vec::new();
            let literal = self.get_whole_literal();
            let mut bare_literal = String::new(); //like !++ instead of for example !+10+&1

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

            let (token_kind, value_kind) = match bare_literal.as_str() {
                "!+" => (TokenKind::NewTack, vec![ValueKind::Bin]),
                "!++" => (TokenKind::NewItem, vec![ValueKind::Bin, ValueKind::Hex]),
                "!-" => (TokenKind::DelItem, vec![ValueKind::Bin]),
                "+" => (TokenKind::Increase, vec![ValueKind::Bin]),
                "-" => (TokenKind::Decrease, vec![ValueKind::Bin]),
                ",>" => (TokenKind::Output, vec![ValueKind::Bin]),
                ",<" => (TokenKind::Input, vec![ValueKind::Bin]),
                "?:" => {
                    if_loop_indexes += 1;
                    (
                        TokenKind::BeginIf(if_loop_indexes - 1),
                        vec![ValueKind::Bin, ValueKind::Bin],
                    )
                }
                ":?" => (TokenKind::Else(if_loop_indexes - 1), vec![]),
                "?|" => {
                    if_loop_indexes -= 1;
                    (TokenKind::EndIf(if_loop_indexes), vec![])
                }
                ">" => {
                    loop_indexes += 1;
                    (TokenKind::StartLoop(loop_indexes - 1), vec![])
                }
                "<" => {
                    if values.is_empty() {
                        loop_indexes -= 1;
                        (TokenKind::BreakLoop(Some(loop_indexes)), vec![])
                    } else {
                        (TokenKind::BreakLoop(None), vec![ValueKind::Dec])
                    }
                }
                _ => (TokenKind::Comment, vec![]),
            };
            let literal = literal.into_iter().collect();

            let mut values_in_right_type: Vec<Value> = Vec::new();
            for (index, current_value_kind) in value_kind.into_iter().enumerate() {
                let is_reference = values.get(index).unwrap().1;
                let value = values.get(index).unwrap().0.as_str().to_string();
                values_in_right_type.push(Value::new_value(
                    current_value_kind,
                    is_reference,
                    value,
                ));
            }

            tokens.push(Token {
                token_kind,
                literal,
                values: values_in_right_type,
            });

            if self.position_counter == self.contents.len().try_into().unwrap() {
                break;
            }
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
        let mut is_command = false;
        loop {
            let current_char = self.current_char();
            self.position_counter += 1;
            if current_char == '`' {
                is_command = !is_command;
                if !is_command {
                    return Vec::new();
                }
            } else if current_char != '\\' && current_char != '\n' && current_char != ' ' {
                literal.push(current_char);
            } else if current_char == '\\' {
                literal.push(current_char);
                return literal;
            } else if current_char == '\n' {
                return Vec::new();
            }
        }
    }
}
