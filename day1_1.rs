use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn apply_frequencies(start_freq: i32, adjustments: &[i32]) -> i32 {
    let mut sum = start_freq;
    for adj in adjustments {
        sum += adj;
    }
    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1]).unwrap();
    let reader = BufReader::new(&f);

    let adjustments: Vec<i32> = reader.
        lines().
        map(|l| l.unwrap().trim().trim_start_matches('+').parse::<i32>().unwrap()).
        collect();

    let start_freq = 0;
    let freq = apply_frequencies(start_freq, &adjustments);

    println!("Final frequency is: {}", freq);
}
