use aes::{
    cipher::{consts::U16, generic_array::GenericArray, BlockDecrypt, KeyInit},
    Aes128,
};

pub fn aes_ecb_decrypt(key: &[u8], s: &[u8]) -> String {
    let key: GenericArray<_, U16> = GenericArray::clone_from_slice(key);
    let mut blocks: Vec<GenericArray<_, U16>> = s
        .chunks(16)
        .map(|block| GenericArray::clone_from_slice(block))
        .collect::<Vec<_>>();
    Aes128::new(&key).decrypt_blocks(&mut blocks);
    blocks.iter().flatten().map(|&x| x as char).collect()
}

#[cfg(test)]
mod test_s1_c7 {
    use crate::set1::challenge6::base64_decode_bytes;

    use super::aes_ecb_decrypt;

    #[test]
    fn test_aes_ecb_decrypt() {
        let input = base64_decode_bytes(include_bytes!("../data/s1c7.txt"));
        let key = b"YELLOW SUBMARINE";
        let actual = aes_ecb_decrypt(key, &input);
        assert!(actual.starts_with("I'm back and I'm ringin' the bell"));
        assert!(actual.ends_with("Play that funky music \n\u{4}\u{4}\u{4}\u{4}"));
    }
}
