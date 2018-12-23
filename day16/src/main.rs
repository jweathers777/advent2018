use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::fmt;
use std::env;
use std::slice::Iter;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

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
    fn get_possible_instructions(&self) -> HashSet<Instruction> {
        let mut possible_instructions = HashSet::new();
        let a = self.instruction_call[1];
        let b = self.instruction_call[2];
        let c = self.instruction_call[3] as usize;
        for instruction in Instruction::iter() {
            let mut before = self.before.clone();

            instruction.execute(&mut before, a, b, c);
            if before == self.after {
                possible_instructions.insert(instruction.clone());
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

struct HopcroftKarp<'a> {
    ubound: usize,
    vbound: usize,
    adj: &'a HashMap<usize,Vec<usize>>,
    infinity: u32,
    pair_u: Vec<usize>,
    pair_v: Vec<usize>,
    dist: Vec<u32>,
}

impl<'a> fmt::Display for HopcroftKarp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "ubound: {}", self.ubound);
        writeln!(f, "vbound: {}", self.vbound);
        writeln!(f, "infinity: {}", self.infinity);
        write!(f, "     u: ");
        for u in 0..(self.ubound) {
            write!(f, "{:02} ", u);
        }
        write!(f, "\npair_u: ");
        for val in &self.pair_u {
            write!(f, "{:02} ", val);
        }
        writeln!(f, "");
        write!(f, "pair_v: ");
        for val in &self.pair_v {
            write!(f, "{:02} ", val);
        }
        writeln!(f, "");
        write!(f, "  dist: ");
        for val in &self.dist {
            write!(f, "{:02} ", val);
        }
        writeln!(f, "");
        writeln!(f, "-----")
    }
}

impl<'a> HopcroftKarp<'a> {
    fn perform(ubound: usize, vbound: usize, adj: &'a HashMap<usize,Vec<usize>>) -> HashMap<usize,usize> {
        let infinity = (ubound + vbound + 1) as u32;
        let pair_u: Vec<usize> = vec![];
        let pair_v: Vec<usize> = vec![];
        let dist: Vec<u32> = vec![infinity; ubound+1];

        let mut state = HopcroftKarp{ubound, vbound, adj, infinity, pair_u, pair_v, dist};

        state.hopcroft_karp();

        let mut matching = HashMap::new();
        for u in 0..ubound {
            matching.insert(u, state.pair_u[u]);
        }

        matching
    }

    fn hopcroft_karp(&mut self) {
        for _ in 0..(self.ubound) {
            self.pair_u.push(self.ubound);
        }

        for _ in 0..(self.vbound) {
            self.pair_v.push(self.vbound);
        }

        while self.bfs() {
            for u in 0..(self.ubound) {
                if self.pair_u[u] == self.ubound {
                    self.dfs(u);
                }
            }
        }
    }

    fn bfs(&mut self) -> bool {
        let mut q = VecDeque::new();
        for u in 0..(self.ubound) {
            if self.pair_u[u] == self.ubound {
                self.dist[u] = 0;
                q.push_back(u);
            } else {
                self.dist[u] = self.infinity;
            }
        }

        self.dist[self.ubound] = self.infinity;

        while !q.is_empty() {
            let u: usize = q.pop_front().unwrap();
            if self.dist[u] < self.dist[self.ubound] {
                match self.adj.get(&u) {
                    Some(adj) => for v in adj.iter() {
                        let v_u = self.pair_v[*v];
                        if self.dist[v_u] == self.infinity {
                            self.dist[v_u] = self.dist[u] + 1;
                            q.push_back(v_u);
                        }
                    },
                    None => {},
                }
            }
        }
        self.dist[self.ubound] != self.infinity
    }

    fn dfs(&mut self, u: usize) -> bool {
        if u != self.ubound {
            match self.adj.get(&u) {
                Some(adj) => for v in adj.iter() {
                    let v_u = self.pair_v[*v];
                    if self.dist[v_u] == self.dist[u] + 1 {
                        if self.dfs(v_u) {
                            self.pair_v[*v] = u;
                            self.pair_u[u] = *v;
                            return true;
                        }
                    }
                },
                None => {},
            }
            self.dist[u] = self.infinity;
            return false;
        }
        true
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { panic!("Too few arguments!") }

    let part = args[2].parse::<u32>().expect("Invalid part!");

    let f = File::open(&args[1]).expect("File not found!");
    let reader = BufReader::new(&f);

    let mut samples = vec![];
    let mut program_instructions = vec![];
    let mut lines = vec![];
    let mut i = 0;

    for line in reader.lines() {
        if i < 3 {
            let l = line.unwrap().trim().to_string();
            if l.is_empty() {
                i = 4;
            }
            lines.push(l);
            i += 1;
        } else if i == 3 {
            i = 0;
            let before = parse_register_set(lines[0].trim_start_matches("Before: "));
            let instruction_call = parse_instruction_call(&lines[1]);
            let after = parse_register_set(lines[2].trim_start_matches("After: "));
            samples.push(CpuSample{before, instruction_call, after});
            lines.clear();
        } else {
            let l = line.unwrap().trim().to_string();
            if !l.is_empty() {
                let instruction_call = parse_instruction_call(&l);
                program_instructions.push(instruction_call);
            }
        }
    }

    if part == 1 {
        let mut three_or_more = 0;
        for sample in &samples {
            if sample.get_possible_instructions().len() >= 3 {
                three_or_more += 1;
            }
        }
        println!("{}", three_or_more);
    } else {
        let mut opcode_table = HashMap::new();
        for sample in &samples {
            let opcode = sample.instruction_call[0];
            let new_values = sample.get_possible_instructions();

            if opcode_table.contains_key(&opcode) {
                let current_values: HashSet<Instruction> = opcode_table.remove(&opcode).unwrap();
                let updated_values: HashSet<Instruction> = current_values.intersection(&new_values).
                    map(|&i| i).collect();
                opcode_table.insert(opcode, updated_values);
            } else {
                opcode_table.insert(opcode, new_values);
            }
        }

        let mut opcode_adj = HashMap::new();
        let insts: Vec<Instruction> = Instruction::iter().
            map(|&i| i).collect();
        let ubound = opcode_table.len();
        let vbound = insts.len();

        for u in 0..ubound {
            let mut adjs = vec![];
            let u_insts = &opcode_table[&(u as u32)];
            for (v, inst) in insts.iter().enumerate() {
                if u_insts.contains(&inst) {
                    adjs.push(v);
                }
            }
            opcode_adj.insert(u, adjs);
        }

        let opcode_map = HopcroftKarp::perform(ubound, vbound, &opcode_adj);

        for opcode in 0..ubound {
            match opcode_map.get(&opcode) {
                Some(v) => println!("{} => {}", opcode, *v),
                None => println!("No mapping for {}", opcode),
            }
        }

        let mut registers: RegisterSet = [0; 4];

        let insts: Vec<Instruction> = Instruction::iter().map(|&i| i).collect();
        for program_instruction in &program_instructions {
            let opcode = program_instruction[0] as usize;
            let inst = insts[opcode_map[&opcode]];

            let a = program_instruction[1];
            let b = program_instruction[2];
            let c = program_instruction[3];

            inst.execute(&mut registers, a, b, c as usize);
        }
        println!("Register 0 contains: {}", registers[0]);
    }
}
