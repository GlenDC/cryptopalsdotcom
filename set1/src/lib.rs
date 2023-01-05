pub mod base64;
pub mod bitwise;
pub mod hex;

#[cfg(test)]
mod tests {
    use crate::base64;
    use crate::bitwise;
    use crate::hex;

    #[test]
    fn test_challenge_1_hex64_to_base64() {
        let b = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        let s = base64::encode(&b);
        assert_eq!(
            s,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn test_challenge_2_fixed_xor() {
        let a = hex::decode("1c0111001f010100061a024b53535009181c");
        let b = hex::decode("686974207468652062756c6c277320657965");

        let s = hex::encode(&bitwise::xor(&a, &b));
        assert_eq!(s, "746865206b696420646f6e277420706c6179");
    }
}
