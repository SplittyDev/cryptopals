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

/// Crack an xor-encrypted string encrypted using a single-byte key
pub fn crack_single_byte_xor_cipher<T>(s: T) -> DecodingResult
where
    T: AsRef<str>,
{
    // English letter frequencies
    // Sourced from https://www3.nd.edu/~busiforc/handouts/cryptography/letterfrequencies.html
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
        'u' => 0.036308,
        'd' => 0.033844,
        'p' => 0.031671,
        'm' => 0.030129,
        'h' => 0.030034,
        'g' => 0.024705,
        'b' => 0.020720,
        'f' => 0.018121,
        'y' => 0.017779,
        'w' => 0.012899,
        'k' => 0.011016,
        'v' => 0.010074,
        'x' => 0.002902,
        'z' => 0.002722,
        'j' => 0.001965,
        'q' => 0.001962
    ];
    /// Count occurrences of each character in a string
    fn tally(s: &str) -> HashMap<char, f32> {
        let mut result = HashMap::<char, f32>::new();
        for c in s.chars() {
            *result.entry(c.to_ascii_lowercase()).or_insert(0.0) += 1.0;
        }
        result
    }
    // Score the likelihood of the specified string containing valid English text
    // using the letter-frequency table above and the Bhattacharyya-Distance algorithm
    let score = |s: &str| -> f32 {
        let tally = tally(s);
        let total = s.len() as f32;
        // Bhattacharyya distance
        english_letter_frequencies
            .iter()
            .map(|(key, value)| {
                let score = tally.get(key).unwrap_or(&0f32) / total * value;
                score.sqrt()
            })
            .sum()
    };
    // Iterate over all possible u8 values
    (0..u8::MAX)
        // Decrypt all values using the current key-guess
        .map(|b| {
            (
                b,
                s.as_ref()
                    .as_bytes()
                    .iter()
                    .map(|s| (s ^ b) as char)
                    .collect::<String>(),
            )
        })
        // Score each resulting string according to letter frequencies
        .map(|(key, s)| (key, score(&s), s))
        // Find the best candidate based on the previously calculated score
        .max_by(|(_, score_a, _), (_, score_b, _)| {
            score_a
                .partial_cmp(score_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        // Instantiate the decoding result
        .map(|(key, score, s)| DecodingResult { key, score, str: s })
        .unwrap()
}

#[cfg(test)]
mod test_s1_c3 {
    use super::crack_single_byte_xor_cipher;
    use crate::set1::challenge1::unhexlify;

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
