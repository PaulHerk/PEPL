mod enums;

pub use crate::lexer::enums::{items, TokenKind};
#[derive(Debug)]
struct Token {
    kind: TokenKind,
    literal: String,
    tack: Vec<char>,
    extra: Vec<char>,
}

impl Token {
    pub fn new_token(kind: TokenKind, literal: String, tack: Vec<char>, extra: Vec<char>) -> Self {
        Self {
            kind,
            literal,
            tack,
            extra,
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

    pub fn lex(&mut self) {
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
                    let tack_bin: Vec<char> = self.get_tack_bin();
                    match operation {
                        '+' => match self.current_char() {
                            '\\' => tokens.push(Token::new_token(
                                TokenKind::NewTack,
                                "!+".to_owned(),
                                tack_bin,
                                Vec::new(),
                            )),
                            '+' => tokens.push(Token::new_token(
                                TokenKind::NewItem,
                                "!++".to_owned(),
                                tack_bin,
                                self.get_item_hex(),
                            )),
                            _ => (),
                        },
                        '-' => match self.current_char() {
                            '\\' => tokens.push(Token::new_token(
                                TokenKind::DelItem,
                                "!-".to_owned(),
                                tack_bin,
                                Vec::new(),
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
                        Vec::new(),
                    ));
                }
                '-' => {
                    tokens.push(Token::new_token(
                        TokenKind::Decrease,
                        "-".to_owned(),
                        self.get_tack_bin(),
                        Vec::new(),
                    ));
                }
                // -----------------------------------------------
                //                     I/O
                // -----------------------------------------------
                ',' => {
                    let tack_bin: Vec<char> = self.get_tack_bin();
                    if self.current_char() == '>' {
                        tokens.push(Token::new_token(
                            TokenKind::Output,
                            ",>".to_owned(),
                            tack_bin,
                            Vec::new(),
                        ));
                    } else if self.current_char() == '<' {
                        tokens.push(Token::new_token(
                            TokenKind::Input,
                            ",<".to_owned(),
                            tack_bin,
                            Vec::new(),
                        ));
                    }
                }
                // -----------------------------------------------
                //                  IF
                // -----------------------------------------------
                '?' => {
                    let first_tack: Vec<char> = self.get_tack_bin();
                    if self.current_char() == ':' {
                        let second_tack: Vec<char> = self.get_tack_bin();
                        tokens.push(Token::new_token(
                            TokenKind::BeginIf,
                            "?:".to_owned(),
                            first_tack,
                            second_tack,
                        ));
                    } else if self.current_char() == '|' {
                        tokens.push(Token::new_token(
                            TokenKind::EndIf,
                            "?|".to_owned(),
                            Vec::new(),
                            Vec::new(),
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
                            Vec::new(),
                            Vec::new(),
                        ));
                    } else {
                        return;
                    }
                }
                // -----------------------------------------------
                //                  FOR LOOP
                // -----------------------------------------------
                '>' => {
                    let tack_bin: Vec<char> = self.get_tack_bin();
                    match self.current_char() {
                        '|' => {
                            let tag_dec: Vec<char> = self.get_tag_dec();
                            tokens.push(Token::new_token(
                                TokenKind::StartFor,
                                ">|".to_owned(),
                                tack_bin,
                                tag_dec,
                            ))
                        }
                        _ => panic!("Wrong syntax!"),
                    }
                }
                '<' => {
                    let tag_dec: Vec<char> = self.get_tag_dec();
                    tokens.push(Token::new_token(
                        TokenKind::EndFor,
                        "<".to_owned(),
                        Vec::new(),
                        tag_dec,
                    ))
                }
                _ => (),
            }
            self.position_counter += 1;
        }
        println!("{:?}", tokens)
    }

    fn current_char(&self) -> char {
        self.contents[self.position_counter]
    }
    fn get_tack_bin(&mut self) -> Vec<char> {
        let bin: [char; 2] = ['0', '1'];
        let mut bin_num: Vec<char> = Vec::new();
        loop {
            self.position_counter += 1;
            let current_char: char = self.current_char();
            if bin.contains(&current_char) {
                bin_num.push(current_char);
            }
            if items().contains(&current_char) {
                break;
            }
        }
        bin_num
    }

    fn get_tag_dec(&mut self) -> Vec<char> {
        let dec: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut dec_num: Vec<char> = Vec::new();
        let mut current_char: char;
        loop {
            self.position_counter += 1;
            current_char = self.current_char();
            if dec.contains(&current_char) {
                dec_num.push(current_char);
            }
            if items().contains(&current_char) {
                break;
            }
        }
        dec_num
    }

    fn get_item_hex(&mut self) -> Vec<char> {
        let hex: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
        ];
        let mut hex_num: Vec<char> = Vec::new();
        let mut current_char: char;
        loop {
            self.position_counter += 1;
            current_char = self.current_char();
            if hex.contains(&current_char) {
                hex_num.push(current_char);
            }
            if items().contains(&current_char) {
                break;
            }
        }
        if hex_num.len() > 2 {
            panic!("Only 255 possibilities allowed!");
        }
        hex_num
    }
}
