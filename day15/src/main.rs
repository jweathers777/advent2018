use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::fmt;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp::Ordering;
use std::slice::Iter;

use self::Direction::*;
use self::UnitKind::*;

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    North, West, East, South
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sym = match self {
            North => 'N',
            South => 'S',
            West => 'W',
            East => 'E',
        };
        write!(f, "{}", sym)
    }
}

impl Direction {
    pub fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction;  4] = [North, West, East, South];
        DIRECTIONS.into_iter()
    }
}

#[derive(Copy, Clone, PartialEq)]
enum UnitKind {
    Elf, Goblin
}

#[derive(Copy, Clone)]
struct Unit {
    kind: UnitKind,
    attack_power: u32,
    hit_points: i32,
}

impl Unit {
    fn new(kind: UnitKind) -> Unit {
        Unit{kind, attack_power: 3, hit_points: 200}
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sym = match self.kind {
            Elf => 'E',
            Goblin => 'G',
        };
        write!(f, "{}({})", sym, self.hit_points)
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(row: {}, col: {})", self.row, self.col)
    }
}

struct CombatBoard {
    rows: Vec<Vec<char>>,
    elves: HashSet<Coord>,
    goblins: HashSet<Coord>,
    units: HashMap<Coord,Unit>,
    rounds: u32,
}

impl CombatBoard {
    fn new(s: &str) -> CombatBoard {
        let mut rows = Vec::new();
        let mut elves = HashSet::new();
        let mut goblins = HashSet::new();
        let mut units = HashMap::new();

        for (row, line) in s.lines().enumerate() {
            let mut cols = Vec::new();
            for (col, c) in line.chars().enumerate() {
                let coord = Coord{row, col};
                match c {
                    'E' => {
                        cols.push('.');
                        elves.insert(coord);
                        units.insert(coord, Unit::new(Elf));
                    },
                    'G' => {
                        cols.push('.');
                        goblins.insert(coord);
                        units.insert(coord, Unit::new(Goblin));
                    },
                    _ => {
                        cols.push(c);
                    },
                }
            }
            rows.push(cols);
        }

        CombatBoard{rows, elves, goblins, units, rounds: 0}
    }

    fn is_finished(&self) -> bool {
        self.elves.len() == 0 || self.goblins.len() == 0
    }

    fn sorted_unit_positions(&self) -> Vec<Coord> {
        let mut positions = Vec::new();

        for position in self.units.keys() {
            positions.push(position.clone());
        }

        positions.sort_unstable_by(|a,b| {
            if a.row < b.row {
                Ordering::Less
            } else if a.row > b.row {
                Ordering::Greater
            } else {
                if a.col < b.col {
                    Ordering::Less
                } else if a.col > b.col {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        });

        positions
    }

    fn step(&self, direction: Direction, coord: Coord) -> Option<Coord> {
        let mut row = coord.row;
        let mut col = coord.col;

        match direction {
            North => if row >= 1 { row -= 1 } else { return None },
            South => if row < self.rows.len() { row += 1 } else { return None },
            West => if col >= 1 { col -= 1 } else { return None },
            East => if col < self.rows[0].len() { col += 1 } else { return None },
        }

        if self.rows[row][col] == '#' { None } else { Some(Coord{row, col}) }
    }

    fn find_best_move(&self, position: Coord, kind: UnitKind) -> Option<Coord> {
        let (friends, enemies) = match kind {
            Elf => (&self.elves, &self.goblins),
            Goblin => (&self.goblins, &self.elves),
        };

        let mut visited = HashSet::new();
        let mut previous = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(position.clone());

        let mut dest_coord = None;
        let mut seeking_dest = true;

        while seeking_dest {
            let coord = queue.pop_front().unwrap();
            visited.insert(coord.clone());

            for &dir in Direction::iter() {
                match self.step(dir, coord) {
                    Some(new_coord) => {
                        if !visited.contains(&new_coord) &&
                            !friends.contains(&new_coord) {
                            if enemies.contains(&new_coord) {
                                let mut current_coord = coord;
                                loop {
                                    match previous.get(&current_coord) {
                                        Some(&prev) => {
                                            if prev == position {
                                                break;
                                            }
                                            current_coord = prev;
                                        },
                                        None => {
                                            if current_coord == position {
                                                return None
                                            } else {
                                                panic!("{} lacks a predecessor!", current_coord);
                                            }
                                        }
                                    }
                                    let &prev = previous.get(&current_coord).unwrap();
                                    if prev == position {
                                        break;
                                    }
                                    current_coord = prev;
                                }

                                dest_coord = Some(current_coord);
                                seeking_dest = false;
                                break;
                            } else {
                                if !previous.contains_key(&new_coord) {
                                    previous.insert(new_coord.clone(), coord.clone());
                                    queue.push_back(new_coord);
                                }
                            }
                        }
                    },
                    None => {},
                }
            }
            seeking_dest = seeking_dest && !queue.is_empty();
        }

        dest_coord
    }

    fn find_best_target(&self, coord: Coord, kind: UnitKind) -> Option<Coord> {
        let enemies = match kind {
            Elf => &self.goblins,
            Goblin => &self.elves,
        };
        let mut min_hit_points = 0;
        let mut best_target = None;

        for &dir in Direction::iter() {

            match self.step(dir, coord) {
                Some(new_coord) => {
                    if enemies.contains(&new_coord) {
                        let enemy = self.units.get(&new_coord).unwrap();
                        if best_target == None || min_hit_points > enemy.hit_points {
                            best_target = Some(new_coord);
                            min_hit_points = enemy.hit_points;
                        }
                    }
                },
                None => {},
            }
        }
        best_target
    }

    fn make_move(&mut self, start: Coord, finish: Coord) {
        let unit = self.units.remove(&start).unwrap();
        match unit.kind {
            Elf => {
                self.elves.remove(&start);
                self.elves.insert(finish.clone());
            },
            Goblin => {
                self.goblins.remove(&start);
                self.goblins.insert(finish.clone());
            }
        }
        self.units.insert(finish, unit);
    }

    fn attack_target(&mut self, position: Coord, target: Coord) {
        let attack_power = self.units.get(&position).unwrap().attack_power;
        let (kind, defeated) = {
            let defender = self.units.get_mut(&target).unwrap();
            defender.hit_points -= attack_power as i32;
            (defender.kind, defender.hit_points <= 0)
        };

        if defeated {
            match kind {
                Elf => self.elves.remove(&target),
                Goblin => self.goblins.remove(&target),
            };
            self.units.remove(&target);
        }
    }

    fn execute_round(&mut self) {
        let positions = self.sorted_unit_positions();

        for &position in positions.iter() {
            let (kind, living) = {
                match self.units.get(&position) {
                    Some(unit) => (unit.kind, true),
                    None => (Elf, false),
                }
            };

            if living {
                let new_position =
                    match self.find_best_move(position, kind) {
                        Some(coord) => {
                            self.make_move(position, coord);
                            coord
                        }
                        None => position,
                    };

                match self.find_best_target(new_position, kind) {
                    Some(target) => {
                        self.attack_target(new_position, target);
                        if self.is_finished() {
                            return;
                        }
                    },
                    None => {},
                }
            }
        }
        self.rounds += 1;
        //println!("After {} rounds:\n{}", self.rounds, self);
    }

    fn outcome(&self) -> u32 {
        let total_hit_points: u32 = self.units.values().
            map(|u| u.hit_points as u32).sum();
        total_hit_points * self.rounds
    }
}

impl fmt::Display for CombatBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (r, row) in self.rows.iter().enumerate() {
            let mut tail = vec![];
            for (c, col) in row.iter().enumerate() {
                let coord = Coord{row: r, col: c};
                if self.elves.contains(&coord) {
                    write!(f, "E");
                    let unit = self.units.get(&coord).unwrap();
                    tail.push(format!("{}", unit));
                } else if self.goblins.contains(&coord) {
                    write!(f, "G");
                    let unit = self.units.get(&coord).unwrap();
                    tail.push(format!("{}", unit));
                } else {
                    write!(f, "{}", col);
                }
            }

            if tail.len() > 0 {
                writeln!(f, "  {}", tail.join(", "));
            } else {
                writeln!(f, "");
            }
        }

        write!(f, "")
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Too few arguments!") }

    let mut s = String::new();
    let f = File::open(&args[1]).expect("File not found!");
    let mut reader = BufReader::new(&f);
    reader.read_to_string(&mut s).expect("Error reading file into string!");

    let mut board = CombatBoard::new(&s);

    println!("{}", board);

    while !board.is_finished() {
        board.execute_round();
    }

    println!("{}", board);
    println!("Outcome: {}", board.outcome());
}
