use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

struct StepPlan {
    steps: HashSet<String>,
    preconditions: HashMap<String,HashSet<String>>,
    conditioned_by: HashMap<String,Vec<String>>,
}

fn parse_step_plan(lines: Vec<String>) -> StepPlan {
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

    StepPlan{steps: steps, preconditions: preconditions, conditioned_by: conditioned_by}
}

fn get_ordered_steps(step_plan: StepPlan) -> String {
    let mut steps = step_plan.steps;
    let mut preconditions = step_plan.preconditions;
    let conditioned_by = step_plan.conditioned_by;

    let mut ordered_steps = String::from("");

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
        ordered_steps.push_str(&step);

        let csteps = conditioned_by.get(&step.to_string()).unwrap();

        for cstep in csteps{
            let mut precondition_set = preconditions.get_mut(&cstep.to_string()).unwrap();
            precondition_set.remove(&step.clone());
        }
        steps.remove(&step.clone());
    }

    ordered_steps
}

fn get_finish_time(step_plan: StepPlan, worker_count: u32, base_duration: u32) -> u32 {
    let mut steps = step_plan.steps;
    let mut preconditions = step_plan.preconditions;
    let conditioned_by = step_plan.conditioned_by;

    let step_durations: HashMap<String,u32> = (b'A'..=b'Z')
        .map(|c| c as char)
        .enumerate()
        .map(|(i,c)| (c.to_string() ,(i as u32) + base_duration + 1))
        .collect();

    let mut seconds = 0;
    let mut busy_workers: HashMap<String,u32> = HashMap::new();
    let mut free_workers = worker_count;

    loop {
        // Mark the passage of time
        seconds += 1;

        // Update how much time each worker must continuing working
        for (step, dur) in busy_workers.iter_mut() {
            *dur = *dur - 1;
            if *dur == 0 {
                free_workers += 1;

                // Remove any preconditions created by this step
                let csteps = conditioned_by.get(&step.to_string()).unwrap();
                for cstep in csteps{
                    let mut precondition_set = preconditions.get_mut(&cstep.to_string()).unwrap();
                    precondition_set.remove(&step.clone());
                }
            }
        }

        // Remove workers who are no longer busy
        busy_workers.retain(|_,dur| *dur != 0);

        // Look for work to perform if workers are free
        while free_workers > 0 && !steps.is_empty() {
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

            // What precondtions does this step have before we can perform it?
            let precondition_count = preconditions.get(&step.to_string()).unwrap().len();
            if precondition_count == 0 {
                // Start working
                let duration = step_durations.get(&step.to_string()).unwrap();
                busy_workers.insert(step.clone(), *duration);

                // Remove a work step and a free worker
                free_workers -= 1;
                steps.remove(&step.clone());
            } else {
                // We have preconditions that must be met before we continue
                break;
            }
        }

        // Are we done yet?
        if steps.is_empty() && busy_workers.len() == 0 { break };
    }

    seconds - 1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { panic!("Too few arguments!") }
    let part: u32 = args[2].parse().expect("Invalid part!");

    let f = File::open(&args[1]).expect("File not found!");
    let reader = BufReader::new(&f);

    let lines = reader.lines().map(|l| String::from(l.unwrap())).
        collect();

    let step_plan = parse_step_plan(lines);

    if part == 1 {
        let ordered_steps = get_ordered_steps(step_plan);
        println!("{}", ordered_steps);
    } else {
        if args.len() != 5 { panic!("Too few arguments!") }
        let worker_count: u32 = args[3].parse().expect("Invalid worker count!");
        let base_duration: u32 = args[4].parse().expect("Invalid worker count!");
        let finish_time = get_finish_time(step_plan, worker_count, base_duration);
        println!("{}", finish_time);
    }
}
