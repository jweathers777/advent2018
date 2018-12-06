use std::env;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

fn eqv(c1: &char, c2: &char) -> bool {
    let lc1 = c1.to_lowercase().to_string();
    let lc2 = c2.to_lowercase().to_string();
    c1 != c2 && lc1 == lc2
}

fn reduce(s: &str) -> String {
    let n = s.len();
    if n <= 1 {
        String::from(s)
    } else {
        let mut received = s.chars().peekable();
        let mut reduced: Vec<char> = vec!['0';n];

        let mut j = 0;

        loop {
            match received.next() {
                Some(src) => {
                    if j >= 1 {
                        let left = reduced[j-1];
                        if eqv(&src, &left) {
                            j -= 1;
                            continue;
                        }
                    }

                    match received.peek() {
                        Some(&right) => {
                            if eqv(&src, &right) {
                                received.next();
                            } else {
                                reduced[j] = src;
                                j += 1;
                            }
                        },
                        None => {
                            reduced[j] = src;
                            j += 1;
                        }
                    }
                },
                None => { break }
            }
        }
        reduced.truncate(j);
        reduced.into_iter().collect::<String>()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 { panic!("Too few arguments!") }

    let mut s = String::new();
    let f = File::open(&args[1]).expect("File not found!");
    let mut reader = BufReader::new(&f);
    reader.read_to_string(&mut s).expect("Error reading file into string!");

    let part: u32 = args[2].parse().expect("Invalid part!");

    if part == 1 {
        println!("{}", reduce(s.as_str()).len());
    } else {
        let mut units = HashSet::new();
        for c in s.chars() {
            units.insert(c.to_lowercase().to_string());
        }

        let mut shortest_len = s.len();
        for unit in units.iter() {
            let mut sprime: String = s.chars().
                filter(|c| c.to_lowercase().to_string() != *unit).
                collect();
            let reduced_len = reduce(sprime.as_str()).len();

            if reduced_len < shortest_len {
                shortest_len = reduced_len;
            }
        }
        println!("{}", shortest_len);
    }
}
