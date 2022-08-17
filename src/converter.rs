use crate::interpreter::{error::ErrorOnInterpreter, ErrorkindOnInterpreter};

use super::Interpreter;
pub fn encode(mut value: u32, base: String) -> Result<String, ErrorkindOnInterpreter> {
    // dec -> ?
    let mut encoded_vec: String = String::new();
    loop {
        let quotient = (value as f32 / base.len() as f32).floor() as u32; // 1
        if quotient == 0 {
            encoded_vec.push(base.chars().nth(value as usize).unwrap());
            break;
        }

        let remainder = value % base.len() as u32;
        encoded_vec.push(base.chars().nth(remainder as usize).unwrap()); // TODO: if not unwrap -> encoding error

        value = quotient;
    }

    Ok(encoded_vec.chars().rev().collect::<String>())
}
//

impl Interpreter {
    pub fn decode(&self, mut value: String, base: String) -> Result<u32, ErrorOnInterpreter> {
        // ? -> dec
        let mut decoded_num: u32 = 0;
        value = value.chars().rev().collect::<String>();
        for (index, current_char) in value.chars().enumerate() {
            let multiplier = base.find(current_char);
            match multiplier {
                Some(multiplier) => {
                    decoded_num += (base.len() as u32).pow(index as u32) * multiplier as u32
                }
                None => {
                    return Err(ErrorOnInterpreter::new_error(
                        ErrorkindOnInterpreter::InvalidNumber,
                        self.position_counter,
                    ))
                }
            }
        }
        Ok(decoded_num)
    }
}

// I use decimal since in the interpreter everything is decimal again
