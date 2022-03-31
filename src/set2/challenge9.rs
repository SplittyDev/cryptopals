pub fn pkcs7_pad<T>(bytes: T, block_len: usize) -> Vec<u8>
where
    T: AsRef<[u8]>,
{
    let bytes = bytes.as_ref();
    let pad_len = block_len % bytes.len();
    let mut vec = Vec::with_capacity(bytes.len() + pad_len);
    vec.extend(bytes);
    vec.extend((0..pad_len).map(|_| pad_len as u8));
    vec
}

#[cfg(test)]
mod test_s2_c9 {
    use super::pkcs7_pad;

    #[test]
    fn test_pkcs7_pad() {
        let input = b"Hello world";
        let block_size = 16_usize;
        let expected: [u8; 16] = [
            b'H', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', 5, 5, 5, 5, 5,
        ];
        let expected_len = 16;
        let actual = pkcs7_pad(input, block_size);
        assert_eq!(expected_len, actual.len());
        assert_eq!(&expected[..], actual);
    }

    #[test]
    fn test_pkcs7_pad_noop() {
        let input = [0u8; 16];
        let expected = [0u8; 16];
        let block_size = 16;
        let actual = pkcs7_pad(input, block_size);
        assert_eq!(block_size, actual.len());
        assert_eq!(&expected[..], actual);
    }
}
