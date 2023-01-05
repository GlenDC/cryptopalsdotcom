use std::fmt::Write;

pub fn decode(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        // NOTE: the line below panics if given string is not a valid hex string
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

pub fn encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}
