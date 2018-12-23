use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::fmt;
use std::env;
use std::collections::HashMap;
use std::collections::VecDeque;

use self::Direction::*;
use self::GroundState::*;
use self::Process::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
struct Coord {
    row: u32,
    col: u32,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(row: {}, col: {})", self.row, self.col)
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Down, Left, Right
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sym = match self {
            Down => 'v', Left => '<', Right => '>'
        };
        write!(f, "{}", sym)
    }
}

#[derive(Copy, Clone, PartialEq)]
enum GroundState {
    Clay, Sand, Spring, WetSand, Water, Mark
}

impl fmt::Display for GroundState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sym = match self {
            Clay => '#', Sand => '.', Spring => '+',
            WetSand => '|', Water => '~', Mark => 'M',
        };
        write!(f, "{}", sym)
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Process {
    Flowing, SpreadingLeft, SpreadingRight
}

impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let word = match self {
            Flowing => "Flowing",
            SpreadingLeft => "SpreadingLeft",
            SpreadingRight => "SpreadingRight",
        };
        write!(f, "{}", word)
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct FlowState {
    process: Process,
    pos: Coord,
    direction: Direction,
    edges_index: Option<usize>,
}

impl fmt::Display for FlowState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.process, self.pos, self.direction)
    }
}

impl FlowState {
    fn new(process: Process, pos: Coord, direction: Direction, edges_index: Option<usize>) -> FlowState {
        FlowState{process, pos, direction, edges_index}
    }
}

struct GroundScan {
    patches: HashMap<Coord,GroundState>,
    spring: Coord,
    min_scan_row: u32,
    max_scan_row: u32,
    visited: u32,
}

impl GroundScan {
    fn new(s: &str, spring: Coord) -> GroundScan {
        let mut patches = HashMap::new();

        for line in s.lines() {
            let mut tokens: Vec<&str> = line.split(", ").collect();
            if tokens.len() != 2 {
                panic!("Invalid line: {}", line);
            }
            if tokens[0].starts_with("x=") {
                let col = tokens[0].trim_start_matches("x=").
                    parse::<u32>().expect(&format!("Invalid value {}", tokens[0]));
                let row_range: Vec<u32> = tokens[1].
                    trim_start_matches("y=").
                    split("..").
                    map(|v| v.parse::<u32>().expect(&format!("Invalid value in {}", tokens[1]))).
                    collect();
                if row_range.len() != 2 {
                    panic!("Invalid row range in line: {}", line);
                }
                for row in row_range[0]..=row_range[1] {
                    let coord = Coord{row, col};
                    patches.insert(coord, Clay);
                }
            } else if tokens[0].starts_with("y=") {
                let row = tokens[0].trim_start_matches("y=").
                    parse::<u32>().expect(&format!("Invalid value {}", tokens[0]));
                let col_range: Vec<u32> = tokens[1].
                    trim_start_matches("x=").
                    split("..").
                    map(|v| v.parse::<u32>().expect(&format!("Invalid value in {}", tokens[1]))).
                    collect();
                if col_range.len() != 2 {
                    panic!("Invalid col range in line: {}", line);
                }
                for col in col_range[0]..=col_range[1] {
                    let coord = Coord{row, col};
                    patches.insert(coord, Clay);
                }
            } else {
                panic!("Invalid coordinate syntax in line: {}", line);
            }
        }

        let min_scan_row: u32 = patches.keys().
            map(|p| p.row).min().unwrap();
        let max_scan_row: u32 = patches.keys().
            map(|p| p.row).max().unwrap();
        let visited = 0;

        patches.insert(spring, Spring);
        GroundScan{patches, spring, min_scan_row, max_scan_row, visited}
    }

    fn get_patch(&self, pos: Coord) -> GroundState {
        *(self.patches.get(&pos).unwrap_or(&Sand))
    }

    fn step(&self, pos: Coord, direction: Direction) -> Option<Coord> {
        let mut row = pos.row;
        let mut col = pos.col;

        match direction {
            Down => if row < self.max_scan_row { row += 1 } else { return None },
            Left => col -= 1,
            Right => col += 1,
        }

        Some(Coord{row, col})
    }

    fn mark_wet_sand(&mut self, pos: Coord) {
        if self.get_patch(pos) == Sand {
            if pos.row >= self.min_scan_row {
                self.visited += 1;
            }
            self.patches.insert(pos, WetSand);
        }
    }

    fn print_diagnostic(&mut self, state: &FlowState, patch: GroundState) {
        println!("{} -> {}", state, patch);
        let temp = self.get_patch(state.pos);
        self.patches.insert(state.pos, Mark);
        println!("{}", self);
        self.patches.insert(state.pos, temp);
    }

    fn trapped_spots_count(&self) -> usize {
        let mut trapped_spots = 0;
        for ground_state in self.patches.values() {
            if *ground_state == Water {
                trapped_spots += 1;
            }
        }
        trapped_spots
    }

    fn flow(&mut self, state: &FlowState, patch: GroundState, new_pos: Coord,
            nodes: &mut VecDeque<FlowState>, edges: &mut Vec<(Coord, Option<Coord>, Option<Coord>)>) {
        match state.direction {
            Down => match patch {
                Sand | WetSand  => {
                    self.mark_wet_sand(new_pos);
                    nodes.push_back(FlowState::new(Flowing, new_pos, Down, None));
                },
                Clay | Water => {
                    edges.push((state.pos, None, None));
                    let edge_index = edges.len() - 1;
                    nodes.push_back(FlowState::new(SpreadingLeft, state.pos, Left, Some(edge_index)));
                    nodes.push_back(FlowState::new(SpreadingRight, state.pos, Right, Some(edge_index)));
                },
                Spring => {
                    self.print_diagnostic(state, patch);
                    panic!("Invalid flow!");
                },
                Mark => {}
            },
            _ => {
                self.print_diagnostic(state, patch);
                panic!("Invalid flow!");
            }
        }
    }

    fn spread(&mut self, spread_direction: Direction, state: &FlowState,
              patch: GroundState, new_pos: Coord,
              nodes: &mut VecDeque<FlowState>, edges: &mut Vec<(Coord, Option<Coord>, Option<Coord>)>) {
        let process = if spread_direction == Left { SpreadingLeft } else { SpreadingRight };
        match state.direction {
            Left | Right => match patch {
                Sand | WetSand => {
                    self.mark_wet_sand(new_pos);
                    nodes.push_back(FlowState::new(process, new_pos, Down, state.edges_index));
                },
                Clay | Spring => {
                    let edges_index = state.edges_index.unwrap();
                    let (blocked_pos, left_pos, right_pos, seeking_edge) = match edges[edges_index] {
                        (blocked, None, None) => (blocked, state.pos, state.pos, true),
                        (blocked, Some(left_edge), None) => (blocked, left_edge, state.pos, false),
                        (blocked, None, Some(right_edge)) => (blocked, state.pos, right_edge, false),
                        (blocked, Some(left_edge), Some(right_edge)) => (blocked, left_edge, right_edge, false),
                    };

                    if seeking_edge {
                        let edge = Some(state.pos);
                        let edges_index = state.edges_index.unwrap();
                        edges[edges_index] = if spread_direction == Left { (blocked_pos, edge, None) } else { (blocked_pos, None, edge) };
                    } else {
                        let row = right_pos.row;
                        for col in (left_pos.col)..=(right_pos.col) {
                            self.patches.insert(Coord{row, col}, Water);
                        }
                        let next_pos = Coord{row: blocked_pos.row - 1, col: blocked_pos.col};
                        self.mark_wet_sand(next_pos);

                        edges[edges_index] = (next_pos, None, None);
                        edges.push((next_pos, None, None));
                        let new_edges_index = edges.len() - 1;
                        nodes.push_back(FlowState::new(SpreadingLeft, next_pos, Left, Some(new_edges_index)));
                        nodes.push_back(FlowState::new(SpreadingRight, next_pos, Right, Some(new_edges_index)));
                    }
                },
                Water => {},
                Mark => {}
            },
            Down => match patch {
                Sand | WetSand => {
                    let edges_index = state.edges_index.unwrap();
                    edges[edges_index] = (state.pos, None, None);
                    self.mark_wet_sand(new_pos);
                    nodes.push_back(FlowState::new(Flowing, new_pos, Down, None));
                },
                Clay | Water => nodes.push_back(FlowState::new(process, state.pos, spread_direction, state.edges_index)),
                Spring => {
                    self.print_diagnostic(state, patch);
                    panic!("Invalid flow!");
                },
                Mark => {}
            },
        }
    }

    fn open_spring(&mut self) -> u32 {
        let mut nodes = VecDeque::new();
        let mut edges = Vec::new();

        nodes.push_back(FlowState::new(Flowing, self.spring, Down, None));

        while !nodes.is_empty() {
            let state = nodes.pop_front().unwrap();
            match self.step(state.pos, state.direction) {
                None => {},
                Some(target_pos) => {
                    let patch = self.get_patch(target_pos);
                    match state.process {
                        Flowing => self.flow(&state, patch, target_pos, &mut nodes, &mut edges),
                        SpreadingLeft => self.spread(Left, &state, patch, target_pos, &mut nodes, &mut edges),
                        SpreadingRight => self.spread(Right, &state, patch, target_pos, &mut nodes, &mut edges),
                    }
                }
            };
        }

        self.visited
    }
}

impl fmt::Display for GroundScan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min_row: u32 = self.patches.keys().
            map(|p| p.row).min().unwrap();
        let max_row: u32 = self.patches.keys().
            map(|p| p.row).max().unwrap();
        let min_col: u32 = self.patches.keys().
            map(|p| p.col).min().unwrap();
        let max_col: u32 = self.patches.keys().
            map(|p| p.col).max().unwrap();

        for row in min_row..=max_row {
            for col in min_col..=max_col {
                match self.patches.get(&Coord{row, col}) {
                    Some(ground_state) => {
                        write!(f, "{}", ground_state).unwrap();
                    },
                    None => write!(f, "{}", Sand).unwrap(),
                };
            }
            writeln!(f, "");
        }
        writeln!(f, "")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Too few arguments!") }

    let mut s = String::new();
    let f = File::open(&args[1]).expect("File not found!");
    let mut reader = BufReader::new(&f);
    reader.read_to_string(&mut s).expect("Error reading file into string!");

    let mut ground_scan = GroundScan::new(&s, Coord{row: 0, col: 500});

    let visited = ground_scan.open_spring();
    println!("Spots Reached by Water: {}", visited);

    let trapped_spots = ground_scan.trapped_spots_count();
    println!("Spots With Trapped Water: {}", trapped_spots);
}
