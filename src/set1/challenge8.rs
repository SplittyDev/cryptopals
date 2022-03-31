pub fn duplicate_block_count(block: &mut Vec<&[u8]>) -> usize {
    let len = block.len();
    block.sort();
    block.dedup();
    len - block.len()
}

pub fn find_aes128_cbc_encrypted_text(texts: &[&[u8]]) -> Vec<u8> {
    texts
        .iter()
        .map(|ciphertext| {
            // Divide ciphertext into chunks of 16 bytes each
            let mut chunks = ciphertext.chunks(16).collect::<Vec<_>>();
            (ciphertext, duplicate_block_count(&mut chunks))
        })
        .max_by(|(_, score_a), (_, score_b)| score_a.cmp(score_b))
        .map(|(ciphertext, _)| ciphertext)
        .unwrap()
        .into_iter()
        .cloned()
        .collect()
}

#[cfg(test)]
mod test_s1_c8 {
    use super::find_aes128_cbc_encrypted_text;

    #[test]
    fn test_find_aes128_encrypted_text() {
        let lines: Vec<Vec<u8>> = include_str!("../data/s1c8.txt")
            .lines()
            .map(|line| line.bytes().collect())
            .collect();
        let line_bytes = &lines.iter().map(|l| l.as_slice()).collect::<Vec<&[u8]>>()[..];
        let expected = b"d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a";
        let actual = find_aes128_cbc_encrypted_text(line_bytes);
        assert_eq!(expected, &actual[..]);
    }
}
