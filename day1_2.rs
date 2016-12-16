use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;

fn apply_frequencies(start_freq: i32, adjustments: &[i32]) -> i32 {
    let mut observations = HashSet::new();

    let mut current_freq = start_freq;

    loop {
        for adj in adjustments {
            if observations.contains(&current_freq) {
                return current_freq;
            } else {
                observations.insert(current_freq);
                current_freq += adj;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1]).expect("File not found!");
    let reader = BufReader::new(&f);

    let adjustments: Vec<i32> = reader.
        lines().
        map(|l| l.unwrap().trim().parse::<i32>().unwrap()).
        collect();

    let start_freq = 0;
    let freq = apply_frequencies(start_freq, &adjustments);

    println!("First duplicate frequency is: {}", freq);
}
