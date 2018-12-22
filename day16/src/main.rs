use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::fmt;
use std::env;
use std::slice::Iter;

use self::Instruction::*;

type RegisterSet = [u32; 4];

fn parse_register_set(a: &str) -> RegisterSet {
    let mut register_set = [0; 4];
    let tokens = a.trim().trim_start_matches('[').
        trim_end_matches(']').
        split(", ");

    for (i, token) in tokens.enumerate() {
        register_set[i] = token.parse::<u32>().expect("Not a number!");
        if i == 3 {
            break;
        }
    }
    register_set
}

type InstructionCall = [u32; 4];

fn parse_instruction_call(a: &str) -> InstructionCall {
    let mut instruction_call = [0; 4];
    let tokens = a.split_whitespace();

    for (i, token) in tokens.enumerate() {
        instruction_call[i] = token.parse::<u32>().expect("Not a number!");
        if i == 3 {
            break;
        }
    }
    instruction_call
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct CpuSample {
    before: RegisterSet,
    instruction_call: InstructionCall,
    after: RegisterSet,
}

impl CpuSample {
    fn get_possible_instructions(&self) -> Vec<Instruction> {
        let mut possible_instructions: Vec<Instruction> = Vec::new();
        let a = self.instruction_call[1];
        let b = self.instruction_call[2];
        let c = self.instruction_call[3] as usize;
        for instruction in Instruction::iter() {
            let mut before = self.before.clone();

            instruction.execute(&mut before, a, b, c);
            if before == self.after {
                possible_instructions.push(instruction.clone());
            }
        }
        possible_instructions
    }
}

impl fmt::Display for CpuSample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Before: {:?}", self.before);
        writeln!(f, "{:?}", self.instruction_call);
        writeln!(f, "After: {:?}", self.after)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Instruction {
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Addr => "addr", Addi => "addi", Mulr => "mulr", Muli => "muli",
            Banr => "banr", Bani => "bani", Borr => "borr", Bori => "bori",
            Setr => "setr", Seti => "seti", Gtir => "gtir", Gtri => "gtri", Gtrr => "gtrr",
            Eqir => "eqir", Eqri => "eqri", Eqrr => "eqrr",
        };
        write!(f, "{}", name)
    }
}

impl Instruction {
    pub fn iter() -> Iter<'static, Instruction> {
        static INSTRUCTIONS: [Instruction;  16] = [
            Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori,
            Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
        ];
        INSTRUCTIONS.into_iter()
    }

    fn execute(&self, register_set: &mut RegisterSet, a: u32, b: u32, c: usize) {
        match self {
            Addr => register_set[c] = register_set[a as usize] + register_set[b as usize],
            Addi => register_set[c] = register_set[a as usize] + b,
            Mulr => register_set[c] = register_set[a as usize] * register_set[b as usize],
            Muli => register_set[c] = register_set[a as usize] * b,
            Banr => register_set[c] = register_set[a as usize] & register_set[b as usize],
            Bani => register_set[c] = register_set[a as usize] & b,
            Borr => register_set[c] = register_set[a as usize] | register_set[b as usize],
            Bori => register_set[c] = register_set[a as usize] | b,
            Setr => register_set[c] = register_set[a as usize],
            Seti => register_set[c] = a,
            Gtir => register_set[c] = if a > register_set[b as usize] { 1 } else { 0 },
            Gtri => register_set[c] = if register_set[a as usize] > b { 1 } else { 0 },
            Gtrr => register_set[c] = if register_set[a as usize] > register_set[b as usize] { 1 } else { 0 },
            Eqir => register_set[c] = if a == register_set[b as usize] { 1 } else { 0 },
            Eqri => register_set[c] = if register_set[a as usize] == b { 1 } else { 0 },
            Eqrr => register_set[c] = if register_set[a as usize] == register_set[b as usize] { 1 } else { 0 },
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Too few arguments!") }

    //let part = args[2].parse::<u32>().expect("Invalid part!");

    let f = File::open(&args[1]).expect("File not found!");
    let reader = BufReader::new(&f);

    let mut samples = vec![];
    let mut lines = vec![];
    let mut i = 0;

    for line in reader.lines() {
        if i < 3 {
            let l = line.unwrap().trim().to_string();
            if l.is_empty() {
                break;
            }
            lines.push(l);
            i += 1;
        } else {
            i = 0;
            let before = parse_register_set(lines[0].trim_start_matches("Before: "));
            let instruction_call = parse_instruction_call(&lines[1]);
            let after = parse_register_set(lines[2].trim_start_matches("After: "));
            samples.push(CpuSample{before, instruction_call, after});
            lines.clear();
        }
    }

    let mut three_or_more = 0;
    for sample in &samples {
        if sample.get_possible_instructions().len() >= 3 {
            three_or_more += 1;
        }
    }
    println!("{}", three_or_more);
}
