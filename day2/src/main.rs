use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

fn common_chars(a: &String, b: &String) -> String {
    a.chars().zip(b.chars()).filter(|(x,y)| x == y).map(|(x,_)| x).collect()
}

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
    if args.len() != 3 { panic!("Too few arguments!") }

    let f = File::open(&args[1]).expect("File not found!");
    let reader = BufReader::new(&f);

    let part: u32 = args[2].parse().expect("Invalid part!");

    let box_ids: Vec<String> = reader.
        lines().
        map(|l| l.unwrap().trim().to_string()).
        collect();

    if part == 1 {
        println!("Checksum = {}", checksum(&box_ids));
    } else {
        let box_count = box_ids.len();

        for i in 0..box_count {
            let box_id_one = &box_ids[i];

            for j in (i+1)..box_count {
                let box_id_two = &box_ids[j];
                if box_id_one.len() == box_id_two.len() {
                    let common = common_chars(box_id_one, box_id_two);
                    if common.len() == box_id_one.len() - 1 {
                        println!("{}", common);
                        return
                    }
                }
            }
        }
    }
}
