pub(crate) mod types;

use ascii_converter::{
    binary_to_string, decimals_to_string, hexadecimal_to_decimal, hexadecimal_to_string,
};

pub use crate::lexer::types::{break_items, TokenKind};
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    pub tack: String,
    pub extra: String,
    pub reference: bool,
}

impl Token {
    pub fn new_token(
        kind: TokenKind,
        literal: String,
        tack: String,
        extra: String,
        reference: bool,
    ) -> Self {
        Self {
            kind,
            literal,
            tack,
            extra,
            reference,
        }
    }
}

pub struct Lexer {
    contents: Vec<char>,
    position_counter: usize,
}

impl Lexer {
    pub fn new_lexer(contents: String) -> Self {
        Self {
            contents: contents.chars().collect(),
            position_counter: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        //goes through every character and checks if it fits into the pepl language
        let mut tokens: Vec<Token> = Vec::new();
        while self.position_counter < self.contents.len() {
            let current_char: char = self.current_char();
            match current_char {
                // -----------------------------------------------
                //                     !
                // -----------------------------------------------
                '!' => {
                    self.position_counter += 1;
                    let operation: char = self.current_char();
                    let tack_bin: String = self.get_tack_bin();
                    match operation {
                        '+' => match self.current_char() {
                            '\\' => tokens.push(Token::new_token(
                                TokenKind::NewTack,
                                "!+".to_owned(),
                                tack_bin,
                                String::new(),
                                self.check_if_reference(),
                            )),
                            '+' => tokens.push(Token::new_token(
                                TokenKind::NewItem,
                                "!++".to_owned(),
                                tack_bin,
                                self.get_item_hex(),
                                self.check_if_reference(),
                            )),
                            _ => (),
                        },
                        '-' => match self.current_char() {
                            '\\' => tokens.push(Token::new_token(
                                TokenKind::DelItem,
                                "!-".to_owned(),
                                tack_bin,
                                String::new(),
                                self.check_if_reference(),
                            )),
                            _ => (),
                        },
                        _ => (),
                    };
                }
                // -----------------------------------------------
                //                  CREMENT
                // -----------------------------------------------
                '+' => {
                    tokens.push(Token::new_token(
                        TokenKind::Increase,
                        "+".to_owned(),
                        self.get_tack_bin(),
                        String::new(),
                        self.check_if_reference(),
                    ));
                }
                '-' => {
                    tokens.push(Token::new_token(
                        TokenKind::Decrease,
                        "-".to_owned(),
                        self.get_tack_bin(),
                        String::new(),
                        self.check_if_reference(),
                    ));
                }
                // -----------------------------------------------
                //                     I/O
                // -----------------------------------------------
                ',' => {
                    let tack_bin: String = self.get_tack_bin();
                    if self.current_char() == '>' {
                        tokens.push(Token::new_token(
                            TokenKind::Output,
                            ",>".to_owned(),
                            tack_bin,
                            String::new(),
                            self.check_if_reference(),
                        ));
                    } else if self.current_char() == '<' {
                        tokens.push(Token::new_token(
                            TokenKind::Input,
                            ",<".to_owned(),
                            tack_bin,
                            String::new(),
                            self.check_if_reference(),
                        ));
                    }
                }
                // -----------------------------------------------
                //                  IF
                // -----------------------------------------------
                '?' => {
                    let first_tack: String = self.get_tack_bin();
                    if self.current_char() == ':' {
                        let second_tack: String = self.get_tack_bin();
                        tokens.push(Token::new_token(
                            TokenKind::BeginIf,
                            "?:".to_owned(),
                            first_tack,
                            second_tack,
                            self.check_if_reference(),
                        ));
                    } else if self.current_char() == '|' {
                        tokens.push(Token::new_token(
                            TokenKind::EndIf,
                            "?|".to_owned(),
                            String::new(),
                            String::new(),
                            false,
                        ));
                    } else {
                        panic!("Wrong syntax!")
                    }
                }
                ':' => {
                    self.position_counter += 1;
                    if self.current_char() == '?' {
                        tokens.push(Token::new_token(
                            TokenKind::Else,
                            "?:".to_owned(),
                            String::new(),
                            String::new(),
                            false,
                        ));
                    } else {
                        break;
                    }
                }
                // -----------------------------------------------
                //                  FOR LOOP
                // -----------------------------------------------
                '>' => {
                    let tag_dec: String = self.get_tag_dec();
                    tokens.push(Token::new_token(
                        TokenKind::StartLoop,
                        ">".to_owned(),
                        String::new(),
                        tag_dec,
                        self.check_if_reference(),
                    ))
                }
                '<' => {
                    let tag_dec: String = self.get_tag_dec();
                    tokens.push(Token::new_token(
                        TokenKind::EndLoop,
                        "<".to_owned(),
                        String::new(),
                        tag_dec,
                        self.check_if_reference(),
                    ))
                }
                _ => (),
            }
            self.position_counter += 1;
        }
        tokens
    }

    fn current_char(&self) -> char {
        self.contents[self.position_counter]
    }

    fn check_if_reference(&mut self) -> bool {
        self.position_counter += 1;
        let current_char: char = self.current_char();
        if current_char == '&' {
            true
        } else {
            self.position_counter -= 1;
            false
        }
    }

    fn get_tack_bin(&mut self) -> String {
        let bin: [char; 2] = ['0', '1'];
        let mut bin_num: String = String::new();
        loop {
            self.position_counter += 1;
            let current_char: char = self.current_char();
            if bin.contains(&current_char) {
                bin_num.push(current_char);
            }
            if break_items().contains(&current_char) {
                break;
            }
        }
        binary_to_string(&vec![bin_num.parse().unwrap()]).unwrap()
    }

    fn get_tag_dec(&mut self) -> String {
        let dec: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut dec_num: String = String::new();
        let mut current_char: char;
        loop {
            self.position_counter += 1;
            current_char = self.current_char();
            if dec.contains(&current_char) {
                dec_num.push(current_char);
            }
            if break_items().contains(&current_char) {
                break;
            }
        }
        decimals_to_string(&vec![dec_num.parse().unwrap()]).unwrap()
    }

    fn get_item_hex(&mut self) -> String {
        let hex: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
        ];
        let mut hex_num = String::new();
        let mut current_char: char;
        loop {
            self.position_counter += 1;
            current_char = self.current_char();
            if hex.contains(&current_char) {
                hex_num.push(current_char);
            }
            if break_items().contains(&current_char) {
                break;
            }
        }
        if hex_num.len() > 2 {
            panic!("Only 255 possibilities allowed!");
        }
        hexadecimal_to_string(&vec![hex_num]).unwrap()
    }
}
