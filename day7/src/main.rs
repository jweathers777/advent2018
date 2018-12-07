use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 { panic!("Too few arguments!") }

    let f = File::open(&args[1]).expect("File not found!");
    let reader = BufReader::new(&f);

    let lines = reader.lines().map(|l| String::from(l.unwrap()));

    let mut steps = HashSet::new();
    let mut preconditions = HashMap::new();
    let mut conditioned_by = HashMap::new();

    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let step = tokens[7].to_string();
        let precondition_step: String = tokens[1].to_string();

        let mut precondition_set = preconditions.entry(step.clone())
            .or_insert(HashSet::new());
        precondition_set.insert(precondition_step.clone());

        let mut conditioned_by_list = conditioned_by.entry(precondition_step.clone())
            .or_insert(vec![]);
        conditioned_by_list.push(step.clone());

        steps.insert(step.to_string());
        steps.insert(precondition_step.to_string());
    }

    for step in &steps {
        preconditions.entry(step.clone()).or_insert(HashSet::new());
        conditioned_by.entry(step.clone()).or_insert(vec![]);
    }


    let mut step_order = String::from("");

    while !steps.is_empty() {
        let step = steps.iter().min_by(|a,b| {
            let pre_a = preconditions.get(&a.to_string()).unwrap().len();
            let pre_b = preconditions.get(&b.to_string()).unwrap().len();

            if pre_a < pre_b {
                Ordering::Less
            } else if pre_a > pre_b {
                Ordering::Greater
            } else {
                a.cmp(b)
            }
        }).unwrap().clone();
        step_order.push_str(&step);

        let csteps = conditioned_by.get(&step.to_string()).unwrap();

        for cstep in csteps{
            let mut precondition_set = preconditions.get_mut(&cstep.to_string()).unwrap();
            precondition_set.remove(&step.clone());
        }
        steps.remove(&step.clone());
    }

    println!("{}", step_order);
}
