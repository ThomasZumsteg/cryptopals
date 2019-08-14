pub fn encode(base8: &str) -> String {
    let mut result = String::new();
    let bytes: Vec<u8> = base8.chars().map(|b| from_base8(b)).collect();
    for i in (0..bytes.len()).step_by(3) {
        result.push(encode_base64((bytes[i] << 2) + (bytes[i+1] >> 2)));
        result.push(encode_base64((bytes[i+1] << 4 & 0x30) + bytes[i+2]));
    }
    return result;
}

fn from_base8(chr: char) -> u8 {
    match chr {
        '0'...'9' => chr as u8 - '0' as u8,
        'a'...'f' => chr as u8 - 'a' as u8 + 10,
        _ => panic!("Not a valud base8 character {}", chr)
    }
}

fn encode_base64(chr: u8) -> char {
    match chr {
        0...25 => (chr + 'A' as u8) as char,
        26...51 => (chr - 26 + 'a' as u8) as char,
        52...61 => (chr - 52 + '0' as u8) as char,
        62 => '+',
        63 => '/',
        _ => panic!("Not a valid base64 digit {}", chr)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let input ="49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let output ="SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(encode(input), output);
    }

    #[test]
    fn test_from_base8() {
        assert_eq!(from_base8('0'), 0);
        assert_eq!(from_base8('a'), 10);
        assert_eq!(from_base8('f'), 15);
    }

    #[test]
    fn test_encode_base64() {
        assert_eq!(encode_base64(0), 'A');
        assert_eq!(encode_base64(25), 'Z');
        assert_eq!(encode_base64(26), 'a');
        assert_eq!(encode_base64(51), 'z');
        assert_eq!(encode_base64(52), '0');
        assert_eq!(encode_base64(61), '9');
        assert_eq!(encode_base64(62), '+');
        assert_eq!(encode_base64(63), '/');
    }
}