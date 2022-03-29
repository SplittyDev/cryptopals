use super::challenge3::{DecodingResult, crack_single_byte_xor_cipher};

pub fn find_xor_encrypted_string<T>(list: &[T]) -> DecodingResult where T: AsRef<str> {
    list
        .iter()
        .map(|s| crack_single_byte_xor_cipher(s.as_ref()))
        .max_by(|a, b| {
            a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap()
}

#[cfg(test)]
mod test_s1_c4 {
    use crate::set1::challenge1::unhexlify;
    use super::find_xor_encrypted_string;

    #[test]
    fn test_find_xor_encrypted_string() {
        let inputs = include_str!("../data/s1c4.txt")
            .split_ascii_whitespace()
            .map(|s| unhexlify(s).unwrap())
            .collect::<Vec<_>>();
        let expected_key = 53_u8;
        let expected_str = "Now that the party is jumping\n";
        let actual = find_xor_encrypted_string(&inputs);
        assert_eq!(expected_key, actual.key);
        assert_eq!(expected_str, actual.str);
    }
}
