use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

fn chars_by_frequency(s: &String) -> HashMap<u32, Vec<char>> {
    let mut frequencies = HashMap::new();
    let mut result = HashMap::new();

    for c in s.chars() {
        let count = frequencies.entry(c).or_insert(0);
        *count += 1;
    }

    for (c, f) in &frequencies {
        let chars = result.entry(*f).or_insert(Vec::new());
        chars.push(*c);
    }

    result
}

fn checksum(box_ids: &Vec<String>) -> u32 {
    let mut contain_chars_twice = 0;
    let mut contain_chars_thrice = 0;

    for box_id in box_ids {
        let chars_by_freq = chars_by_frequency(box_id);
        if chars_by_freq.contains_key(&2) {
            contain_chars_twice += 1;
        }
        if chars_by_freq.contains_key(&3) {
            contain_chars_thrice += 1;
        }
    }

    contain_chars_twice * contain_chars_thrice
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).expect("File not found!");
    let reader = BufReader::new(&f);

    let box_ids: Vec<String> = reader.
        lines().
        map(|l| l.unwrap().trim().to_string()).
        collect();

    println!("Checksum = {}", checksum(&box_ids));
}
