mod challenge_1;

use std::env;
use std::io::{self, Read};
use challenge_1::Bytes;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let mut encoded = String::new();
    io::stdin().lock().read_to_string(&mut encoded).unwrap();
    let bytes = Bytes::from_base64(&encoded.replace("\n", ""));

    println!("{}\n", bytes.decode(&Bytes::from_str(&args[1])).to_string());
}
