pub fn encode(input: &[u8]) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < input.len() {
        let chunk = &input[i..std::cmp::min(i + 3, input.len())];

        let value = (chunk[0] as u32) << 16 | (chunk.get(1).copied().unwrap_or(0) as u32) << 8 | chunk.get(2).copied().unwrap_or(0) as u32;

        result.push(encode_char((value as u8 >> 18) & 0b111111));
        result.push(encode_char((value as u8>> 12) & 0b111111));
        result.push(encode_char((value as u8 >> 6) & 0b111111));
        result.push(encode_char(value as u8 & 0b111111));

        for _ in 0..(3 - chunk.len()) {
            result.push('=');
        }

        i += 3;
    }

    result
}

fn encode_char(value: u8) -> char {
    if value < 26 {
        (value + b'A') as char
    } else if value < 52 {
        ((value - 26) + b'a') as char
    } else if value < 62 {
        ((value - 52) + b'0') as char
    } else if value == 62 {
        '+'
    } else {
        '/'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = b"Hello, world!";
        let encoded = encode(input);
        assert_eq!(encoded, "SGVsbG8sIHdvcmxkIQ==");
    }

    #[test]
    fn test_doc_examples() {
        assert!(encode(b"Hello, world!").is_empty() == false);
    }
}
