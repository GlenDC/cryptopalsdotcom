const BASE64_CHARS: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

pub fn encode(b: &[u8]) -> String {
    let mut r = Vec::new();
    let mut p = Vec::new();
    let mut s = Vec::from(b);

    let c = s.len() % 3;
    // add a right zero pad to make this string a multiple of 3 characters
    if c > 0 {
        for _ in c..3 {
            p.push('=' as u8);
            s.push('\0' as u8);
        }
    }

    // increment over the length of the string, three characters at a time
    let mut c = 0;
    while c < s.len() {
        // these three 8-bit (ASCII) characters become one 24-bit number
        let n = ((s[c] as u32) << 16) + ((s[c + 1] as u32) << 8) + s[c + 2] as u32;

        // this 24-bit number gets separated into four 6-bit numbers
        let a = [(n >> 18) & 63, (n >> 12) & 63, (n >> 6) & 63, n & 63];

        // those four 6-bit numbers are used as indices into the base64 character list
        for i in a {
            r.push(BASE64_CHARS[i as usize] as u8);
        }

        c += 3;
    }

    let mut v = r[..r.len() - p.len()].to_vec();
    v.extend(p);
    String::from_utf8(v).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_bytes() {
        assert_eq!(encode(&[0, 0, 0]), "AAAA");
        assert_eq!(encode(&[0, 0, 0, 0, 0, 0]), "AAAAAAAA");
        assert_eq!(encode(&[0, 0, 0, 0, 0, 0, 0, 0, 0]), "AAAAAAAAAAAA");
    }

    #[test]
    fn test_zero_bytes_pad() {
        assert_eq!(encode(&[0, 0]), "AAA=");
        assert_eq!(encode(&[0]), "AA==");
    }

    #[test]
    fn test_one_bytes() {
        assert_eq!(encode(&[0xFF, 0xFF]), "//8=");
    }

    #[test]
    fn test_mix_bytes() {
        assert_eq!(encode(&[0b00000000, 0b00010000, 0b00000000]), "ABAA");
    }
}
