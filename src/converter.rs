pub fn encode(mut value: u32, base: String) -> String {
    // dec -> ?
    let mut encoded_vec: String = String::new();
    loop {
        let quotient = (value as f32 / base.len() as f32).floor() as u32; // 1
        if quotient == 0 {
            encoded_vec.push(base.chars().nth(value as usize).unwrap());
            break;
        }

        let remainder = value % base.len() as u32;
        encoded_vec.push(base.chars().nth(remainder.try_into().unwrap()).unwrap()); // TODO: if not unwrap -> encoding error

        value = quotient;
    }

    encoded_vec.chars().rev().collect::<String>()
}

//

pub fn decode(mut value: String, base: String) -> u32 {
    // ? -> dec
    let mut decoded_num: u32 = 0;
    value = value.chars().rev().collect::<String>();
    for (index, current_char) in value.chars().enumerate() {
        let multiplier = base.find(current_char).unwrap() as u32; // TODO: if not unwrap -> decoding error ('r' in hex for example)
        decoded_num += (base.len() as u32).pow(index as u32) * multiplier;
    }
    decoded_num
}

// I use decimal since in the interpreter everything is decimal again
