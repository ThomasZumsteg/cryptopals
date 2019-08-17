pub fn encode(base8: &str) -> String {
    let mut result = String::new();
    let bytes: Vec<u8> = base8.chars().map(from_base8).collect();
    for i in (0..bytes.len()).step_by(3) {
        result.push(encode_base64((bytes[i] << 2) + (bytes[i+1] >> 2)));
        result.push(encode_base64((bytes[i+1] << 4 & 0x30) + bytes[i+2]));
    }
    return result;
}

pub fn xor(buff_a: &str, buff_b: &str) -> String {
    buff_a.chars()
        .zip(buff_b.chars().into_iter())
        .map(|(a, b)| to_base8(from_base8(a) ^ from_base8(b)))
        .collect()
}

struct Guess {
    result: String,
    score: u32,
}

pub fn find_key(message: &str) -> (String, u32) {
    let mut best_guess: Option<Guess> = None;
    let (bytes, _) = message.chars().fold((String::new(), None), |(mut result, byte), chr| {
        if let Some(value) = byte {
            result.push(((from_base8(value) << 4) + from_base8(chr)) as char);
            (result, None)
        } else {
            (result, Some(chr))
        }
    });
    for key in 0..255 {
        let (result, score) = bytes.chars().fold((String::new(), 0), |(mut decoded, mut score), b| {
            let c = (b as u8 ^ key) as char;
            if c.is_ascii_alphabetic() || c.is_whitespace() {
                score += 1;
            }
            decoded.push(c);
            (decoded, score)
        });
        if best_guess.is_none() || best_guess.as_ref().unwrap().score < score {
            best_guess = Some(Guess { result: result, score: score });
        }
    }
    if let Some(soltuion) = best_guess {
        return (soltuion.result, soltuion.score)
    }
    unimplemented!()
}

fn from_base8(chr: char) -> u8 {
    match chr {
        '0'...'9' => chr as u8 - '0' as u8,
        'a'...'f' => chr as u8 - 'a' as u8 + 10,
        _ => panic!("Not a valud base8 character {}", chr)
    }
}

fn to_base8(value: u8) -> char {
    match value {
        0...9 => (value + '0' as u8) as char,
        10...15 => (value  - 10 + 'a' as u8) as char,
        _ => panic!("Not a valud base8 value {}", value)
    }
}

fn encode_base64(value: u8) -> char {
    match value {
        0...25 => (value + 'A' as u8) as char,
        26...51 => (value - 26 + 'a' as u8) as char,
        52...61 => (value - 52 + '0' as u8) as char,
        62 => '+',
        63 => '/',
        _ => panic!("Not a valid base64 value {}", value)
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
        assert_eq!(from_base8('9'), 9);
        assert_eq!(from_base8('a'), 10);
        assert_eq!(from_base8('f'), 15);
    }

    #[test]
    fn test_to_base8() {
        assert_eq!(to_base8(0), '0');
        assert_eq!(to_base8(9), '9');
        assert_eq!(to_base8(10), 'a');
        assert_eq!(to_base8(15), 'f');
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

    #[test]
    fn test_xor() {
        let value_1 = "1c0111001f010100061a024b53535009181c";
        let value_2 = "686974207468652062756c6c277320657965";
        let value_3 = "746865206b696420646f6e277420706c6179";
        assert_eq!(xor(value_1, value_2), value_3);
        assert_eq!(xor(value_2, value_1), value_3);
        assert_eq!(xor(value_1, value_3), value_2);
        assert_eq!(xor(value_3, value_1), value_2);
        assert_eq!(xor(value_2, value_3), value_1);
        assert_eq!(xor(value_3, value_2), value_1);
    }

    #[test]
    fn test_decode() {
        let (result, _) = find_key("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        assert_eq!("Cooking MC's like a pound of bacon", result);
    }
}
