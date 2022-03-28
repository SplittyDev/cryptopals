pub fn static_xor<const N: usize>(a: &[u8; N], b: &[u8; N]) -> [u8; N] {
    a
        .iter()
        .enumerate()
        .map(|(i, x)| x ^ b[i])
        .collect::<Vec<_>>()
        .as_slice()
        .try_into()
        .unwrap()
}

pub fn static_xor_unsized<const N: usize>(a: &[u8], b: &[u8]) -> Option<[u8; N]> {
    if a.len() == N && b.len() == N {
        let fixed_a: [u8; N] = a.try_into().ok()?;
        let fixed_b: [u8; N] = b.try_into().ok()?;
        Some(static_xor(&fixed_a, &fixed_b))
    } else {
        None
    }
}

pub fn static_str_xor_unsized<const N: usize>(a: &str, b: &str) -> Option<[u8; N]> {
    static_xor_unsized(a.as_bytes(), b.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::{static_xor, static_xor_unsized, static_str_xor_unsized};

    #[test]
    fn test_static_xor() {
        let a = [0xAA, 0xBB, 0xCC];
        let b = [0xC1, 0xB2, 0xA3];
        let expected = [0x6B, 0x09, 0x6F];
        let actual = static_xor(&a, &b);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_static_xor_2() {
        let a = [
            0x1c, 0x01, 0x11, 0x00, 0x1f, 0x01,
            0x01, 0x00, 0x06, 0x1a, 0x02, 0x4b,
            0x53, 0x53, 0x50, 0x09, 0x18, 0x1c
        ];
        let b = [
            0x68, 0x69, 0x74, 0x20, 0x74, 0x68,
            0x65, 0x20, 0x62, 0x75, 0x6c, 0x6c,
            0x27, 0x73, 0x20, 0x65, 0x79, 0x65
        ];
        let expected = [
            0x74, 0x68, 0x65, 0x20, 0x6b, 0x69,
            0x64, 0x20, 0x64, 0x6f, 0x6e, 0x27,
            0x74, 0x20, 0x70, 0x6c, 0x61, 0x79
        ];
        let actual = static_xor(&a, &b);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_static_xor_unsized_valid_input() {
        let input_a: [u8; 3] = [0xA1, 0xA2, 0xA3];
        let input_b: [u8; 3] = [0x10, 0x20, 0x30];
        let expected: [u8; 3] = [0xB1, 0x82, 0x93];
        let actual = static_xor_unsized::<3>(&input_a, &input_b);
        assert!(actual.is_some());
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn test_static_xor_unsized_invalid_input() {
        let input_a: [u8; 3] = [0xA1, 0xA2, 0xA3];
        let input_b: [u8; 4] = [0x10, 0x20, 0x30, 0x40];
        let actual = static_xor_unsized::<3>(&input_a, &input_b);
        assert!(actual.is_none());
    }

    #[test]
    fn test_static_str_xor_unsized_valid_input() {
        let input_a = "Foo";
        let input_b = "Bar";
        let expected: [u8; 3] = [0x04, 0x0e, 0x1d];
        let actual = static_str_xor_unsized::<3>(input_a, input_b);
        assert!(actual.is_some());
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn test_static_str_xor_unsized_invalid_input() {
        let input_a = "Foo";
        let input_b = "Bar_";
        let actual = static_str_xor_unsized::<3>(input_a, input_b);
        assert!(actual.is_none());
    }
}
