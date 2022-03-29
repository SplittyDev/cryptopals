/// Base64 coding table
pub static BASE64_ENCODE_LUT: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

/// Decode a hex-encoded string into a regular string
pub fn unhexlify<T>(s: T) -> Option<String>
where
    T: AsRef<str>,
{
    // Grab a peekable iterator over the chars of the string
    let mut chars = s.as_ref().chars().peekable();

    /// Get the hex value of the specified ASCII character value
    ///
    /// NOTE: This assumes that the specified byte is a valid hex character (/[a-fA-F0-9]/)
    fn char2hex(c: u8) -> Option<u8> {
        if c >= 65 && c <= 90 {
            // A-Z
            Some(c - 65 + 10)
        } else if c >= 97 && c <= 122 {
            // a-z
            Some(c - 97 + 10)
        } else if c >= 48 && c <= 57 {
            // 0-9
            Some(c - 48)
        } else {
            None
        }
    }

    let mut output: Vec<char> = Vec::new();
    while chars.peek().is_some() {
        // Convert next 2 chars into their respective hex-values (0-16)
        let a = char2hex(chars.next()? as u8)?;
        let b = char2hex(chars.next()? as u8)?;
        // Calculate the final hex value (0-255)
        let val = a * 16 + b;
        output.push(val as char);
    }
    return Some(output.into_iter().collect());
}

/// Encode a regular string into a base64-encoded string
pub fn base64_encode<T>(s: T) -> Option<String>
where
    T: AsRef<str>,
{
    let input_size = s.as_ref().len();
    let output_size = 4 * ((input_size + 2) / 3);
    let mut bytes = s.as_ref().bytes().peekable();
    let mut output_bytes: Vec<char> = Vec::with_capacity(output_size);
    while output_bytes.len() < output_size {
        // Encode three octets into a triple
        let octet_a: u32 = bytes.next().map(|x| x as u32).unwrap_or(0);
        let octet_b: u32 = bytes.next().map(|x| x as u32).unwrap_or(0);
        let octet_c: u32 = bytes.next().map(|x| x as u32).unwrap_or(0);
        let triple = octet_a.wrapping_shl(0x10) + octet_b.wrapping_shl(0x08) + octet_c;
        // Encode the triple into four base64 indices
        let base64_bytes = [
            ((triple >> 3 * 6) & 0x3F) as u8,
            ((triple >> 2 * 6) & 0x3F) as u8,
            ((triple >> 1 * 6) & 0x3F) as u8,
            ((triple >> 0 * 6) & 0x3F) as u8,
        ];
        // Turn the base64 indices into their proper base64 representation
        let base64_chars = base64_bytes.map(|x| BASE64_ENCODE_LUT[x as usize]);
        output_bytes.extend_from_slice(&base64_chars[..])
    }
    // Add padding if necessary
    for i in 0..[0, 2, 1][input_size % 3] {
        output_bytes[output_size - 1 - i] = '=';
    }
    Some(output_bytes.iter().collect())
}

// Recode a hex-encoded string into a base64-encoded string
pub fn hex_to_base64<T>(s: T) -> Option<String>
where
    T: AsRef<str>,
{
    base64_encode(unhexlify(s)?)
}

#[cfg(test)]
mod test_s1_c1 {
    use super::{base64_encode, hex_to_base64, unhexlify};

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
        let actual = base64_encode(input).unwrap();
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
