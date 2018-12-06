use std::env;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

fn reduce(s: &str) -> String {
    //println!("{}", s);
    let n = s.len();
    if n <= 1 {
        String::from(s)
    } else {
        let mut received: Vec<char> = s.chars().collect();
        let mut reduced: Vec<char> = s.chars().collect();

        let mut current_n = n;

        loop {
            let mut i = 1;
            let mut j = 0;
            let mut prev = received[0];
            let mut lprev = prev.to_lowercase().to_string();
            let mut reductions = 0;

            loop {
                let current = received[i];
                let lcurrent = current.to_lowercase().to_string();

                if current == prev || lcurrent != lprev {
                    reduced[j] = prev;
                    prev = current;
                    lprev = lcurrent;
                    i += 1;
                    j += 1;
                } else {
                    reductions += 1;
                    i += 2;
                    if i <= current_n {
                        prev = received[i-1];
                        lprev = prev.to_lowercase().to_string();
                    }
                }

                if i == current_n {
                    reduced[j] = prev;
                    current_n = j+1;
                    break;
                } else if i > current_n {
                    current_n = j;
                    break;
                }
            }

            //println!("{}", &reduced[0..current_n].iter().collect::<String>());
            if reductions == 0 {
                break;
            } else {
                let temp = reduced;
                reduced = received;
                received = temp;
            }
        }
        reduced.truncate(current_n);
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
