static HEX_LOOKUP: &'static [u8] = b"0123456789abcdef";

pub fn hexlify<T>(s: T) -> String
where
    T: AsRef<str>,
{
    let mut buf = String::with_capacity(s.as_ref().len() * 2);
    for b in s.as_ref().bytes() {
        let chr1 = HEX_LOOKUP[(b >> 4 & 0x0F) as usize] as char;
        let chr2 = HEX_LOOKUP[(b & 0x0F) as usize] as char;
        buf.extend([chr1, chr2]);
    }
    buf
}

pub fn repeating_key_xor<K, S>(key: K, s: S) -> String
where
    K: AsRef<str>,
    S: AsRef<str>,
{
    let key_bytes = key.as_ref().as_bytes();
    let str_bytes = s.as_ref().as_bytes();
    let mut buf = String::with_capacity(str_bytes.len());
    for (i, c) in str_bytes.iter().enumerate() {
        buf.push((c ^ key_bytes[i % key_bytes.len()]) as char);
    }
    buf
}

#[cfg(test)]
mod test_s1_c5 {
    use super::{hexlify, repeating_key_xor};

    #[test]
    fn test_hexlify() {
        let input = "Hi";
        let expected = "4869";
        let actual = hexlify(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_repeating_key_xor() {
        let input = "Hello world";
        let key = "lmao";
        let expected = "$\x08\r\x03\x03M\x16\x00\x1e\x01\x05";
        let actual = repeating_key_xor(key, input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_repeating_key_xor_hex() {
        let input = "Hello";
        let key = "lmao";
        let expected = "24080d0303";
        let actual = hexlify(&repeating_key_xor(key, input));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_repeating_key_xor_hex_cryptopals() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let actual = hexlify(&repeating_key_xor(key, input));
        assert_eq!(expected, actual);
    }
}
