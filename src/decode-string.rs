mod challenge_1;

use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    print!("{}\n", challenge_1::find_key(&args[1]).0);
}