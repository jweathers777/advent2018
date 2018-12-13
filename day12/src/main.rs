use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::HashSet;

struct PlantSystem {
    min_value: i32,
    max_value: i32,
    values: HashSet<i32>,
    rules: HashMap<String, String>,
}

impl PlantSystem {
    fn new(lines: Vec<String>) -> PlantSystem {
        let mut max_value = 0;
        let mut values = HashSet::new();
        let tokens: Vec<&str> = lines[0].trim().split_whitespace().collect();
        for (i, c) in tokens[2].chars().enumerate() {
            if c == '#' {
                values.insert(i as i32);
                max_value = i as i32;
            }
        }

        let mut rules = HashMap::new();
        for line in lines[2..].iter() {
            let tokens: Vec<&str> = line.trim().split_whitespace().collect();
            rules.insert(tokens[0].to_string(), tokens[2].to_string());
        }

        PlantSystem{min_value: 0i32, max_value: max_value, values: values, rules: rules}
    }

    fn total_value(&self) -> i32 {
//        let mut vals = self.values.iter().collect::<Vec<&i32>>();
//        vals.sort();
//
//        let vstring = vals.iter().
//            map(|i| i.to_string()).
//            collect::<Vec<String>>().join(" -> ");
//
//        println!("{}", vstring);

        self.values.iter().sum()
    }

    fn neighbors(&self, idx: i32) -> String {
        let lower_bound = idx - 2;
        let upper_bound = idx + 2;

        (lower_bound..=upper_bound).map(|i|
            if self.values.contains(&i) {'#'} else {'.'}
        ).collect::<String>()
    }

    fn to_string(&self) -> String {
        (self.min_value..=self.max_value).map(|i|
            if self.values.contains(&i) {'#'} else {'.'}
        ).collect::<String>()
    }

    fn advance(&mut self) {
        let mut values = HashSet::new();
        let lower_bound = self.min_value - 3;
        let upper_bound = self.max_value + 3;

        let mut min_value = upper_bound;
        let mut max_value = lower_bound;

        for idx in lower_bound..=upper_bound {
            let n = self.neighbors(idx);
            let rule_value = match self.rules.get(&n) {
                None => ".",
                Some(v) => v,
            };

            if rule_value == "#" {
                values.insert(idx);

                if idx < min_value {
                    min_value = idx;
                }

                if idx > max_value {
                    max_value = idx;
                }
            }
        }
        self.values = values;
        self.min_value = min_value;
        self.max_value = max_value;
    }
}

impl std::fmt::Display for PlantSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { panic!("Too few arguments!") }

    let n = args[2].parse::<usize>().expect("Invalid number of generations!");

    let f = File::open(&args[1]).expect("File not found!");
    let reader = BufReader::new(&f);

    let lines: Vec<String> = reader.lines().
        map(|l| l.unwrap().trim().to_string()).
        collect();

    let mut plant_system = PlantSystem::new(lines);

    let mut last_str = plant_system.to_string();
    let mut last_total_value = plant_system.total_value() as u64;

    for i in 0..n {
        plant_system.advance();
        let current_str = plant_system.to_string();
        let current_total_value = plant_system.total_value() as u64;

        if current_str == last_str && current_total_value != last_total_value {
            let delta = current_total_value - last_total_value;
            last_total_value = (n - i - 1) as u64 * delta + current_total_value;
            break;
        } else {
            last_str = current_str;
            last_total_value = current_total_value;
        }
    }

    println!("{}", last_total_value);
}
