mod converter;
#[cfg(test)]
mod tests {
    use crate::converter::*;

    #[test]
    fn dec_to_hex() {
        let expected_value: Vec<char> = "8b2f".chars().collect();
        let output = encode(35631, "0123456789abcdef".to_owned());
        assert_eq!(output, expected_value);
    }

    #[test]
    fn hex_to_dec() {
        let expected_value = 35631;
        let value: Vec<char> = "8b2f".chars().collect();
        let output = decode(value, "0123456789abcdef".to_owned());
        println!("{output}");
        assert_eq!(output, expected_value);
    }
}
