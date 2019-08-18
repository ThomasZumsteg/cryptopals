use std::collections::HashMap;

pub fn encode(base8: &str) -> String {
    let mut result = String::new();
    let bytes: Vec<u8> = base8.chars().map(from_base8).collect();
    for i in (0..bytes.len()).step_by(3) {
        result.push(encode_base64((bytes[i] << 2) + (bytes[i+1] >> 2)));
        result.push(encode_base64((bytes[i+1] << 4 & 0x30) + bytes[i+2]));
    }
    return result;
}

pub fn encode_with_key(message: &str, key: &str) -> String {
    message.chars()
        .zip(key.chars().cycle())
        .fold(String::new(),
            |mut acc, (m, k)| {
                let xor = (m as u8) ^ (k as u8);
                acc.push(to_base8(xor >> 4));
                acc.push(to_base8(xor & 0xf));
                acc
            })
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

fn hamming_distance(buff_a: &str, buff_b: &str) -> u32 {
    buff_a.chars()
        .zip(buff_b.chars())
        .fold(0, |mut total, (a, b)| {
            let (a_byte, b_byte) = (a as u8, b as u8);
            for i in 0..8 {
                total += (((a_byte >> i) & 0x1) ^ ((b_byte >> i) & 0x1)) as u32;
            }
            total
        })
}

pub fn decode_with_size(encoded: &str, size: u8) -> String {
    let mut blocks = vec![String::new(); size as usize];
    for (c, chr) in encoded.chars().enumerate() {
        blocks[c % (size as usize)].push(chr);
    }
    let mut key = String::new();
    for block in blocks {
        let (k, _) = find_key(&block); 
        key += &k;
    }
    // print!("{}\n", key);
    key
}

pub fn find_key_size(encoded: &str) -> Vec<u8> {
    let mut edit_distances: HashMap<u8, f32> = HashMap::new();
    for i in 1..256 {
        let blocks: Vec<String> = make_blocks(encoded, i).iter().take(4)
            .map(|s| s.to_owned()).collect();
        let mut edit_distance = 0.0;
        for a in 0..3 {
            for b in (a+1)..4 {
                edit_distance += hamming_distance(&blocks[a], &blocks[b]) as f32 / (i as f32 * 6.0 * 8.0);
            }
        }
        edit_distances.insert(i as u8, edit_distance);
    }
    let mut results = edit_distances.iter().collect::<Vec<(&u8, &f32)>>();
    results.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
    results.iter().map(|v| v.0.to_owned()).collect()
}

fn make_blocks(bytes: &str, size: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut temp = String::new();
    for (c, chr) in bytes.chars().enumerate() {
        if c % size == 0 && c != 0 {
            result.push(temp);
            temp = String::new();
        }
        temp.push(chr);
    }
    result
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

    #[test]
    fn test_encode_with_key() {
        assert_eq!(
            encode_with_key(
                "Burning 'em, if you ain't quick and nimble\n\
                I go crazy when I hear a cymbal", "ICE"),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623\
            d63343c2a26226324272765272a282b2f20430a652e2c652a3\
            124333a653e2b2027630c692b20283165286326302e27282f"
        );
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(
            hamming_distance("this is a test", "wokka wokka!!!"),
            37
        );
    }
}
