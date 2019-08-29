use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct Bytes(Vec<u8>);

impl Bytes {
    fn from_base8(base8: &str) -> Bytes {
        let mut bytes = Vec::new();
        for (i, chr) in base8.chars().enumerate() {
            let nibble = match chr {
                '0'...'9' => chr as u8 - '0' as u8,
                'a'...'f' => chr as u8 - 'a' as u8 + 10,
                _ => panic!("Not a valud base8 character {}", chr)
            };
            if i % 2 == 0 {
                bytes.push(nibble << 4);
            } else {
                let last = bytes.last_mut().unwrap();
                *last += nibble;
            }
        }
        Bytes(bytes )
    }

    fn from_base64(base64: &str) -> Bytes {
        let mut bytes = Vec::new();
        for (i, chr) in base64.chars().enumerate() {
            let six_bits = match chr {
                'A'...'Z' => chr as u8 - 'A' as u8,
                'a'...'z' => chr as u8 - 'a' as u8 + 26,
                '0'...'9' => chr as u8 - '0' as u8 + 52,
                '+' => 62,
                '/' => 63,
                _ => panic!("Not a valid base64 value {}", chr),
            };
            match i % 4 {
                0 => bytes.push(six_bits << 2),
                1 => {
                    let last = bytes.last_mut().unwrap();
                    *last += six_bits >> 4;
                    bytes.push(six_bits << 4);
                },
                2 => {
                    let last = bytes.last_mut().unwrap();
                    *last += six_bits >> 2;
                    bytes.push(six_bits << 6);
                },
                3 => {
                    let last = bytes.last_mut().unwrap();
                    *last += six_bits;
                },
                _ => panic!("Never happens")

            }
        }
        Bytes(bytes)
    }

    fn decode(&self, Bytes(key): &Bytes) -> Bytes {
        let Bytes(message) = self;
        Bytes(message.iter().zip(key.iter().cycle()).map(|(k, v)| k ^ v).collect())
    }

    fn find_key_of_size(&self, key_size: usize) -> Bytes {
        let Bytes(bytes) = self;

        if key_size != 1 {
            unimplemented!()
        }

        let mut best_score: Option<usize> = None;
        let mut best_result: Option<Bytes> = None;
        for key in 0..(std::u8::MAX) {
            let Bytes(result) = self.decode(&Bytes(vec![key]));
            let score = result.iter().map(|&b| {
                let c = b as char;
                if c.is_ascii_alphabetic() || c.is_whitespace() { 1 } else { 0 }
            }).sum();
            println!("{}", Bytes(result.clone()).to_string());
            if best_score.is_none() || best_score.unwrap() < score {
                best_result= Some(Bytes(result));
                best_score = Some(score);
            }
        }
        if let Some(result) = best_result {
            return result;
        }
        unimplemented!()
    }
}

impl ToString for Bytes {
    fn to_string(&self) -> String {
        let Bytes(bytes) = self;
        bytes.iter().map(|&c| c as char).collect()
    }
}



// pub fn find_key(message: &str) -> (String, u32) {
//     let mut best_guess: Option<Guess> = None;
//     let bytes = hex_encode(&message);
//     for key in 0..255 {
//         let (result, score) = bytes.chars().fold((String::new(), 0), |(mut decoded, mut score), b| {
//             let c = (b as u8 ^ key) as char;
//             if c.is_ascii_alphabetic() || c.is_whitespace() {
//                 score += 1;
//             }
//             decoded.push(c);
//             (decoded, score)
//         });
//         if best_guess.is_none() || best_guess.as_ref().unwrap().score < score {
//             best_guess = Some(Guess { result: result, score: score });
//         }
//     }
//     if let Some(soltuion) = best_guess {
//         return (soltuion.result, soltuion.score)
//     }
//     unimplemented!()
// }

// fn hamming_distance(buff_a: &str, buff_b: &str) -> u32 {
//     buff_a.chars()
//         .zip(buff_b.chars())
//         .fold(0, |mut total, (a, b)| {
//             let (a_byte, b_byte) = (a as u8, b as u8);
//             for i in 0..8 {
//                 total += (((a_byte >> i) & 0x1) ^ ((b_byte >> i) & 0x1)) as u32;
//             }
//             total
//         })
// }

// pub fn decode_with_size(encoded: &str, size: u8) -> String {
//     let mut blocks = vec![String::new(); size as usize];
//     for (c, chr) in encoded.chars().enumerate() {
//         blocks[c % (size as usize)].push(chr);
//     }
//     let mut key = String::new();
//     for block in blocks {
//         let (k, _) = find_key(&block); 
//         key += &k;
//     }
//     key
// }

// pub fn find_key_size(encoded: &str) -> Vec<u8> {
//     let mut edit_distances: HashMap<u8, f32> = HashMap::new();
//     for i in 1..256 {
//         let blocks: Vec<String> = make_blocks(encoded, i).iter().take(4)
//             .map(|s| s.to_owned()).collect();
//         let mut edit_distance = 0.0;
//         for a in 0..3 {
//             for b in (a+1)..4 {
//                 edit_distance += hamming_distance(&blocks[a], &blocks[b]) as f32 / (i as f32 * 6.0 * 8.0);
//             }
//         }
//         edit_distances.insert(i as u8, edit_distance);
//     }
//     let mut results = edit_distances.iter().collect::<Vec<(&u8, &f32)>>();
//     results.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
//     results.iter().map(|v| v.0.to_owned()).collect()
// }

// fn make_blocks(bytes: &str, size: usize) -> Vec<String> {
//     let mut result = Vec::new();
//     let mut temp = String::new();
//     for (c, chr) in bytes.chars().enumerate() {
//         if c % size == 0 && c != 0 {
//             result.push(temp);
//             temp = String::new();
//         }
//         temp.push(chr);
//     }
//     result
// }


mod tests {
    use super::*;

    #[test]
    fn test_from_base8() {
        assert_eq!(Bytes::from_base8("0"), Bytes(vec![0 << 4]));
        assert_eq!(Bytes::from_base8("9"), Bytes(vec![9 << 4]));
        assert_eq!(Bytes::from_base8("a"), Bytes(vec![10 << 4]));
        assert_eq!(Bytes::from_base8("f"), Bytes(vec![15 << 4]));
    }

    #[test]
    fn test_from_base64() {
        assert_eq!(Bytes::from_base64("A"), Bytes(vec![0 << 2]));
        assert_eq!(Bytes::from_base64("a"), Bytes(vec![26 << 2]));
        assert_eq!(Bytes::from_base64("z"), Bytes(vec![51 << 2]));
        assert_eq!(Bytes::from_base64("0"), Bytes(vec![52 << 2]));
        assert_eq!(Bytes::from_base64("9"), Bytes(vec![61 << 2]));
        assert_eq!(Bytes::from_base64("+"), Bytes(vec![62 << 2]));
        assert_eq!(Bytes::from_base64("/"), Bytes(vec![63 << 2]));
    }

    #[test]
    fn test_basic() {
        // Set 1 Challenge 1
        let input ="49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let output ="SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(Bytes::from_base8(input), Bytes::from_base64(output));
    }

    #[test]
    fn test_xor() {
        // Set 1 Challenge 2
        let value_1 = Bytes::from_base8("1c0111001f010100061a024b53535009181c");
        let value_2 = Bytes::from_base8("686974207468652062756c6c277320657965");
        let value_3 = Bytes::from_base8("746865206b696420646f6e277420706c6179");
        assert_eq!(value_1.decode(&value_2), value_3);
        assert_eq!(value_2.decode(&value_1), value_3);
        assert_eq!(value_1.decode(&value_3), value_2);
        assert_eq!(value_3.decode(&value_1), value_2);
        assert_eq!(value_2.decode(&value_3), value_1);
        assert_eq!(value_3.decode(&value_2), value_1);
    }

    #[test]
    fn test_decode() {
        let bytes = Bytes::from_base8("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let key = bytes.find_key_of_size(1);
        assert_eq!("Cooking MC's like a pound of bacon", bytes.decode(&key).to_string());
    }

    // #[test]
    // fn test_encode_with_key() {
    //     assert_eq!(
    //         encode_with_key(
    //             "Burning 'em, if you ain't quick and nimble\n\
    //             I go crazy when I hear a cymbal", "ICE"),
    //         "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623\
    //         d63343c2a26226324272765272a282b2f20430a652e2c652a3\
    //         124333a653e2b2027630c692b20283165286326302e27282f"
    //     );
    // }

    // #[test]
    // fn test_hamming_distance() {
    //     assert_eq!(
    //         hamming_distance("this is a test", "wokka wokka!!!"),
    //         37
    //     );
    // }
}
