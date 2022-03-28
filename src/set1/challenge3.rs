use std::collections::HashMap;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[derive(Debug)]
pub struct DecodingResult {
    pub key: u8,
    pub score: f32,
    pub str: String,
}

pub fn crack_single_byte_xor_cipher(s: &str) -> DecodingResult {
    let english_letter_frequencies: HashMap<char, f32> = hashmap![
        ' ' => 0.182884,
        'e' => 0.111607,
        'a' => 0.084966,
        'r' => 0.075809,
        'i' => 0.075448,
        'o' => 0.071635,
        't' => 0.069509,
        'n' => 0.066544,
        's' => 0.057351,
        'l' => 0.054893,
        'c' => 0.045388,
        'u' => 0.036308
    ];
    fn tally(s: &str) -> HashMap<char, f32> {
        let mut result = HashMap::<char, f32>::new();
        for c in s.chars() {
            *result.entry(c.to_ascii_lowercase()).or_insert(0.0) += 1.0;
        }
        result
    }
    let score = |s: &str| -> f32 {
        let tally = tally(s);
        let total = s.len() as f32;
        // Partial Bhattacharyya distance
        english_letter_frequencies
            .iter()
            .map(|(key, value)| {
                let score = tally.get(key).unwrap_or(&0f32) / total * value;
                score.sqrt()
            })
            .sum()
    };
    (0..u8::MAX)
        .map(|b| {
            (b, s
                .as_bytes()
                .iter()
                .map(|s| (s ^ b) as char)
                .collect::<String>())
        })
        .map(|(key, s)| (key, score(&s), s))
        .max_by(|(_, score_a, _), (_, score_b, _)| {
            score_a
                .partial_cmp(score_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(key, score, s)| {
            DecodingResult {
                key,
                score,
                str: s
            }
        })
        .unwrap()
}

#[cfg(test)]
mod test_s1_c3 {
    use crate::set1::challenge1::unhexlify;
    use super::crack_single_byte_xor_cipher;

    #[test]
    fn test_crack_single_byte_xor_cipher() {
        let hex_input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let expected = "Cooking MC's like a pound of bacon";
        let input = unhexlify(hex_input).unwrap();
        let actual = crack_single_byte_xor_cipher(&input);
        assert_eq!(88_u8, actual.key);
        assert_eq!(expected, actual.str);
    }
}
