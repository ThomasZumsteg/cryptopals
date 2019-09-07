mod challenge_1;

use std::env;
use std::io::{self, Read};
use challenge_1::Bytes;
use openssl::symm::{decrypt, Cipher};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let mut encoded = String::new();
    io::stdin().lock().read_to_string(&mut encoded).unwrap();

    let Bytes(key) = Bytes::from_str(&args[1]);
    let Bytes(data) = Bytes::from_base64(&encoded.replace("\n", ""));

    let plain_text = Bytes(decrypt(
        Cipher::aes_128_ecb(),
        &key,
        None,
        &data).unwrap());

    println!("{}", plain_text.to_string());
}
