mod errors;
pub(crate) mod types;
use std::str::Chars;

pub use crate::lexer::types::*;

use self::errors::*;
#[derive(Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    // literal: String,
    pub values: Vec<Value>,
}

fn format_content(content: Chars) -> String {
    let mut bare_content = String::new();
    let mut is_command = false;
    for char in content {
        if char == '`' {
            is_command = !is_command;
        }
        if !is_command && !char.is_whitespace() {
            bare_content.push(char.to_ascii_lowercase());
        }
    }
    bare_content
}
pub struct Lexer {
    contents: String,
}

impl Lexer {
    pub fn new_lexer(contents: Chars) -> Self {
        let contents = format_content(contents);
        Self { contents }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, ErrorOnLexer> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut if_loop_indexes: u32 = 0; // since you don't have curly brackets I have to put an index to that
        let mut loop_indexes = 0; // for normal loop; same as above
        let literals = self.contents.split('\\');

        for (index, current_literal) in literals.enumerate() {
            let mut values: Vec<(String, bool)> = Vec::new();
            let mut bare_literal = String::new(); //like !++ instead of for example !+10+&1
            let literal = current_literal.chars().collect::<Vec<char>>();

            for (index, &current_char) in literal.iter().enumerate() {
                if COMMAND_CHARACTERS.contains(&current_char) {
                    bare_literal.push(current_char);
                    let &next_char = literal.get(index + 1).unwrap_or(&' ');
                    // if let None = next_char {
                    //     break;
                    // }
                    // let &next_char = next_char.unwrap();
                    if next_char == '&' || next_char.is_alphanumeric() {
                        values.push((String::new(), false));
                    }
                } else if current_char == '&' {
                    values.last_mut().unwrap().1 = true;
                } else if current_char.is_alphanumeric() {
                    values.last_mut().unwrap().0.push(current_char);
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
                ":?" => (TokenKind::Else(if_loop_indexes - 1), Vec::new()),
                "?|" => {
                    if_loop_indexes -= 1;
                    (TokenKind::EndIf(if_loop_indexes), Vec::new())
                }
                ">" => {
                    loop_indexes += 1;
                    (TokenKind::StartLoop(loop_indexes - 1), Vec::new())
                }
                "<" => {
                    if values.is_empty() {
                        loop_indexes -= 1;
                        (TokenKind::BreakLoop(Some(loop_indexes)), Vec::new())
                    } else {
                        (TokenKind::BreakLoop(None), vec![ValueKind::Dec])
                    }
                }
                "" => {
                    return Err(ErrorOnLexer::new_error(
                        ErrorkindOnLexer::InvalidSyntax(
                            InvalidSyntaxKind::BackslashAfterLastCommand,
                        ),
                        index as u32,
                    ));
                }

                _ => {
                    return Err(ErrorOnLexer::new_error(
                        ErrorkindOnLexer::InvalidToken,
                        index as u32,
                    ));
                }
            };

            // let mut literal_as_string = String::new();
            let mut values_in_right_type = Vec::new();
            for (index, current_value_kind) in value_kind.into_iter().enumerate() {
                // literal_as_string = literal.iter().collect();

                let (is_reference, value) = match values.get(index) {
                    Some(values) => (values.1, values.0.as_str().to_string()),
                    None => {
                        return Err(ErrorOnLexer::new_error(
                            ErrorkindOnLexer::InvalidSyntax(InvalidSyntaxKind::NoValuePut),
                            index as u32,
                        ));
                    }
                };

                values_in_right_type.push(Value::new_value(
                    current_value_kind,
                    is_reference,
                    value,
                ));
            }
            tokens.push(Token {
                token_kind,
                // literal: literal_as_string,
                values: values_in_right_type,
            });
        }
        Ok(tokens)
    }
}
