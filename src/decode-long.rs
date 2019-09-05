mod challenge_1;

use std::io::{self, Read};
use challenge_1::Bytes;

fn main() {
    let mut encoded = String::new();
    io::stdin().lock().read_to_string(&mut encoded).unwrap();
    encoded = encoded.replace("\n", "");
    let bytes = Bytes::from_base64(&encoded);

    for key_size in bytes.guess_key_sizes() {
        let key = bytes.find_key_of_size(key_size as usize);
        print!("{} {} - {:?}\n", key_size, bytes.decode(&key).score(), key.to_string());
    }
}
