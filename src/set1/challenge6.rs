use crate::set1::{challenge3::crack_single_byte_xor_cipher, challenge5::repeating_key_xor};

use super::challenge1::BASE64_ENCODE_LUT;

pub fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    let mut distance: usize = 0;
    for (byte_a, byte_b) in a.iter().zip(b.iter()) {
        distance += (byte_a ^ byte_b).count_ones() as usize;
    }
    distance
}

pub fn hamming_distance_str<A, B>(a: A, b: B) -> usize
where
    A: AsRef<str>,
    B: AsRef<str>,
{
    hamming_distance(a.as_ref().as_bytes(), b.as_ref().as_bytes())
}

pub fn base64_decode<T>(s: T) -> String
where
    T: AsRef<str>,
{
    let mut buf = String::new();
    let bytes = s.as_ref().as_bytes();
    let pad_len = bytes.iter().rev().take_while(|&&b| b == b'=').count();
    for b in bytes.chunks(4) {
        let decode_byte = |b: u8| {
            BASE64_ENCODE_LUT
                .iter()
                .position(|&x| x as u8 == b)
                .unwrap_or(0)
        };
        let n = (decode_byte(b[0]) << 18)
            + (decode_byte(b[1]) << 12)
            + (decode_byte(b[2]) << 6)
            + decode_byte(b[3]);
        let chars = [(n >> 16) & 0xFF, (n >> 8) & 0xFF, n & 0xFF].map(|x| x as u8 as char);
        buf.extend(&chars[..]);
    }
    (0..pad_len).for_each(|_| drop(buf.pop().unwrap()));
    buf
}

pub fn find_vigenere_key_size<T>(s: T) -> usize
where
    T: AsRef<str>,
{
    (2..=40)
        .map(|key_size| {
            let chunks = s
                .as_ref()
                .as_bytes()
                .chunks(key_size)
                .take(4)
                .collect::<Vec<_>>();
            let distance = (0..4)
                .flat_map(|i| {
                    (i..4)
                        .map(|j| hamming_distance(chunks[i], chunks[j]) as f32 / key_size as f32)
                        .collect::<Vec<_>>()
                })
                .sum::<f32>();
            (key_size, distance)
        })
        .min_by(|(_, score_a), (_, score_b)| {
            score_a
                .partial_cmp(score_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(key_size, _)| key_size)
        .unwrap()
}

pub fn crack_vigenere_cipher<T>(s: T) -> (String, String)
where
    T: AsRef<str>,
{
    let key_size = find_vigenere_key_size(&s);
    let mut blocks: Vec<String> = (0..key_size)
        .map(|_| String::with_capacity(key_size))
        .collect();
    for chunk in s.as_ref().chars().collect::<Vec<_>>().chunks(key_size) {
        for i in 0..key_size.min(chunk.len()) {
            blocks[i].push(chunk[i]);
        }
    }
    let key: String = blocks
        .iter()
        .map(|block| crack_single_byte_xor_cipher(block))
        .map(|result| result.key as char)
        .collect::<String>();
    (key.clone(), repeating_key_xor(&key, &s))
}

#[cfg(test)]
mod test_s1_c6 {
    use crate::set1::challenge6::crack_vigenere_cipher;

    use super::{base64_decode, find_vigenere_key_size, hamming_distance, hamming_distance_str};

    static STR: &'static str = include_str!("../data/s1c6.txt");

    #[test]
    fn test_hamming_distance() {
        let (input_a, input_b) = (b"this is a test", b"wokka wokka!!!");
        let expected = 37_usize;
        let actual = hamming_distance(input_a, input_b);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_hamming_distance_str() {
        let (input_a, input_b) = ("this is a test", "wokka wokka!!!");
        let expected = 37_usize;
        let actual = hamming_distance_str(input_a, input_b);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_base64_decode() {
        let input = "SGVsbG8gd29ybGQ=";
        let expected = "Hello world";
        let actual = base64_decode(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_vigenere_key_size() {
        let expected = 29_usize;
        let actual = find_vigenere_key_size(base64_decode(STR));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_crack_vigenere_cipher() {
        let expected_key = "Terminator X: Bring the noise";
        let (actual_key, actual_result) = crack_vigenere_cipher(base64_decode(STR));
        assert_eq!(expected_key, actual_key);
        assert!(actual_result.starts_with("I'm back and I'm ringin' the bell"));
        assert!(actual_result.ends_with("Play that funky music \n"));
    }
}
