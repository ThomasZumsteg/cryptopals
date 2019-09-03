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

    fn from_str(value: &str) -> Bytes {
        Bytes(String::from(value).into_bytes())
    }

    fn decode(&self, Bytes(key): &Bytes) -> Bytes {
        let Bytes(message) = self;
        Bytes(message.iter().zip(key.iter().cycle()).map(|(k, v)| k ^ v).collect())
    }

    fn find_key_of_size(&self, key_size: usize) -> Bytes {
        let mut key = Vec::new();
        for block in self.blocks(key_size) {
            key.push(block.decode_block());
        }
        Bytes(key)
    }

    fn blocks(&self, size: usize) -> Vec<Bytes> {
        let Bytes(bytes) = self;
        let mut result: Vec<Vec<u8>> = vec![Vec::new(); size];
        for (i, b) in bytes.iter().enumerate() {
            result[i % size].push(*b);
        }
        result.iter().map(|b| Bytes(b.clone())).collect()
    }

    fn decode_block(&self) -> u8 {
        let mut best_score: Option<isize> = None;
        let mut best_key: Option<u8> = None;
        for key in 0..(std::u8::MAX) {
            let Bytes(result) = self.decode(&Bytes(vec![key]));
            let score = result.iter().map(|&b| {
                let c = b as char;
                match c {
                    'a'...'z' | 'A'...'Z' | ' ' => 1, 
                    _ if c.is_control() => -1,
                    _ => 0
                }
            }).sum();
            if best_score.is_none() || best_score.unwrap() < score {
                best_key = Some(key);
                best_score = Some(score);
            }
        }
        best_key.unwrap()
    }

    fn hamming_distance(&self, Bytes(other): &Bytes) -> u32 {
        let Bytes(bytes) = self;
        bytes.iter().zip(other)
            .fold(0, |mut total, (a, b)| {
                for i in 0..8 {
                    total += (((a >> i) & 0x1u8) ^ ((b >> i) & 0x1u8)) as u32;
                }
                total
            })
    }
}

impl ToString for Bytes {
    fn to_string(&self) -> String {
        let Bytes(bytes) = self;
        bytes.iter().map(|&c| c as char).collect()
    }
}


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

    #[test]
    fn test_encode_with_key() {
        let bytes = Bytes::from_base8("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623\
            d63343c2a26226324272765272a282b2f20430a652e2c652a3\
            124333a653e2b2027630c692b20283165286326302e27282f");
        assert_eq!(
            bytes.decode(&Bytes::from_str("ICE")).to_string(),
            "Burning 'em, if you ain't quick and nimble\n\
                I go crazy when I hear a cymbal",
        );
    }

    #[test]
    fn test_decode_with_key_size() {
        let bytes = Bytes::from_base8("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623\
            d63343c2a26226324272765272a282b2f20430a652e2c652a3\
            124333a653e2b2027630c692b20283165286326302e27282f");
        assert_eq!(
            bytes.find_key_of_size(3).to_string(),
            "ICE"
        );
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(
            Bytes::from_str("this is a test").hamming_distance(
                &Bytes::from_str("wokka wokka!!!")),
            37
        );
    }
}
