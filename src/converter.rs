pub fn encode(mut value: u32, base: String) -> Vec<char> {
    let mut encoded_vec: Vec<char> = Vec::new();
    loop {
        let quotient = (value as f32 / base.len() as f32).floor() as u32; // 1
        if quotient == 0 {
            encoded_vec.push(base.chars().nth(value as usize).unwrap());
            break;
        }

        let remainder = value % base.len() as u32;
        encoded_vec.push(base.chars().nth(remainder.try_into().unwrap()).unwrap()); // if not unwrap -> encoding error

        value = quotient;
    }

    encoded_vec.reverse();
    encoded_vec
}

//

pub fn decode(mut value: Vec<char>, base: String) -> u32 {
    let mut decoded_num: u32 = 0;
    value.reverse();
    for (index, current_char) in value.iter().enumerate() {
        let multiplier = base.find(*current_char).unwrap() as u32; // if not unwrap -> decoding error ('r' in hex for example)
        decoded_num += (base.len() as u32).pow(index as u32) * multiplier;
    }
    decoded_num
}
