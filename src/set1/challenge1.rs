static BASE64_ENCODE_LUT: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z', '0', '1', '2', '3',
    '4', '5', '6', '7', '8', '9', '+', '/'
];

pub fn unhexlify(s: &str) -> Option<String> {
    let mut chars = s.chars().peekable();
    fn char2hex(c: u8) -> Option<u8> {
        if c >= 65 && c <= 90 {
            Some(c - 65 + 10)
        } else if c >= 97 && c <= 122 {
            Some(c - 97 + 10)
        } else if c >= 48 && c <= 57 {
            Some(c - 48)
        } else {
            None
        }
    }
    let mut output: Vec<char> = Vec::new();
    while chars.peek().is_some() {
        let a = char2hex(chars.next()? as u8)?;
        let b = char2hex(chars.next()? as u8)?;
        let val = a * 16 + b;
        output.push(val as char);
    }
    return Some(output.into_iter().collect());
}

pub fn base64(s: &str) -> Option<String> {
    let input_size = s.len();
    let output_size = 4 * ((input_size + 2) / 3);
    let mut bytes = s.bytes().peekable();
    let mut output_bytes: Vec<char> = Vec::with_capacity(output_size);
    while output_bytes.len() < output_size {
        let octet_a: u32 = bytes.next().map(|x| x as u32).unwrap_or(0);
        let octet_b: u32 = bytes.next().map(|x| x as u32).unwrap_or(0);
        let octet_c: u32 = bytes.next().map(|x| x as u32).unwrap_or(0);
        let triple = octet_a.wrapping_shl(0x10) + octet_b.wrapping_shl(0x08) + octet_c;
        let base64_bytes = [
            ((triple >> 3 * 6) & 0x3F) as u8,
            ((triple >> 2 * 6) & 0x3F) as u8,
            ((triple >> 1 * 6) & 0x3F) as u8,
            ((triple >> 0 * 6) & 0x3F) as u8,
        ];
        let base64_chars = base64_bytes.map(|x| BASE64_ENCODE_LUT[x as usize]);
        output_bytes.extend_from_slice(&base64_chars[..])
    }
    for i in 0..[0, 2, 1][input_size % 3] {
        output_bytes[output_size - 1 - i] = '=';
    }
    Some(output_bytes.iter().collect())
}

pub fn hex_to_base64(s: &str) -> Option<String> {
    base64(unhexlify(s)?.as_str())
}

#[cfg(test)]
mod tests {
    use super::{unhexlify, base64, hex_to_base64};

    #[test]
    fn test_unhexlify_with_valid_input() {
        let input = "4869";
        let expected = "Hi";
        let actual = unhexlify(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_unhexlify_with_malformed_input() {
        let input = "4869f";
        assert!(unhexlify(input).is_none());
    }

    #[test]
    fn test_base64() {
        let input = "Hello world";
        let expected = "SGVsbG8gd29ybGQ=";
        let actual = base64(input).unwrap();
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn test_hex_to_base64() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let actual = hex_to_base64(input).unwrap();
        assert_eq!(expected, actual);
    }
}