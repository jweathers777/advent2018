use std::env;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

fn reduce(a: &str) -> String {
    let n = a.len();
    if n <= 1 {
        String::from(a)
    } else {
        let mut s = String::with_capacity(n);
        let a_chars = a.chars();
        let mut shifted_a_chars = a.chars();
        shifted_a_chars.next();

        let mut zipped = a_chars.zip(shifted_a_chars);
        let mut append_last_char = false;
        let mut last_char = ' ';
        let mut reductions = 0;

        loop {
            match zipped.next() {
                Some((c1, c2)) => {
                    if c1 == c2 {
                        s.push(c1);
                        append_last_char = true;
                        last_char = c2;
                    } else {
                        let lc1 = c1.to_lowercase().to_string();
                        let lc2 = c2.to_lowercase().to_string();

                        if lc1 != lc2 {
                            s.push(c1);
                            append_last_char = true;
                            last_char = c2;
                        } else {
                            zipped.next();
                            reductions += 1;
                            append_last_char = false;
                        }
                    }
                },
                None => { break },
            };
        }

        if append_last_char { s.push(last_char) };

        if reductions == 0 { s } else { reduce(s.as_str()) }
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
