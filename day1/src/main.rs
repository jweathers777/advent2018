use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;

fn accumulated_frequency(start_freq: i32, adjustments: &[i32]) -> i32 {
    let mut sum = start_freq;
    for adj in adjustments {
        sum += adj;
    }
    sum
}

fn first_repeated_cumulative_freq(start_freq: i32, adjustments: &[i32]) -> i32 {
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
    if args.len() != 3 { panic!("Too few arguments!") }

    let f = File::open(&args[1]).expect("File not found!");
    let reader = BufReader::new(&f);

    let part: u32 = args[2].parse().expect("Invalid part!");

    let adjustments: Vec<i32> = reader.
        lines().
        map(|l| l.unwrap().trim().parse::<i32>().unwrap()).
        collect();

    let start_freq = 0;

    if part == 1 {
        let freq = accumulated_frequency(start_freq, &adjustments);
        println!("Final frequency is: {}", freq);
    } else {
        let freq = first_repeated_cumulative_freq(start_freq, &adjustments);
        println!("First duplicate frequency is: {}", freq);
    }
}
