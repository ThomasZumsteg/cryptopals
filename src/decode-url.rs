mod challenge_1;

use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let body = reqwest::get(&args[1]).unwrap().text().unwrap();
    let mut max_score = 0;
    let mut max_encoded = String::new();
    let mut max_result = String::new();
    for line in body.lines() {
        let (result, score) = challenge_1::find_key(&line);
        if score > max_score {
            max_score = score;
            max_encoded = line.to_string();
            max_result = result;
        }
    }
    print!("{} => '{}'\n", max_encoded, max_result);
}
