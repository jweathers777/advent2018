use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::fmt;
use std::env;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Copy, Clone)]
enum Direction {
    North, South, East, West
}

#[derive(Copy, Clone)]
enum Turn {
    Left, Straight, Right
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Copy, Clone)]
struct Cart {
    row: usize,
    col: usize,
    direction: Direction,
    next_turn: Turn,
}

impl Cart {
    fn new(row: usize, col: usize, direction: Direction) -> Cart {
        Cart{row, col, direction, next_turn: Turn::Left}
    }

    fn to_char(&self) -> char {
        match self.direction {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::West => '<',
            Direction::East => '>',
        }
    }
}

struct TrackSystem {
    rows: Vec<Vec<char>>,
    carts: HashMap<Coord,Cart>,
    crashes: HashSet<Coord>,
    errors: HashSet<Coord>,
}

impl TrackSystem {
    fn new(s: &str) -> TrackSystem {
        let mut rows = Vec::new();
        let mut carts = HashMap::new();

        for (row, line) in s.lines().enumerate() {
            let mut cols = Vec::new();
            for (col, c) in line.chars().enumerate() {
                let coord = Coord{row, col};
                match c {
                    '^' => {
                        cols.push('|');
                        carts.insert(coord, Cart::new(row, col,Direction::North));
                    },
                    'v' => {
                        cols.push('|');
                        carts.insert(coord, Cart::new(row, col, Direction::South));
                    },
                    '<' => {
                        cols.push('-');
                        carts.insert(coord, Cart::new(row, col, Direction::West));
                    },
                    '>' => {
                        cols.push('-');
                        carts.insert(coord, Cart::new(row, col, Direction::East));
                    },
                    _ => {
                        cols.push(c)
                    },
                }
            }
            rows.push(cols);
        }

        TrackSystem{rows, carts, crashes: HashSet::new(), errors: HashSet::new()}
    }

    fn forward(&mut self) {
        let mut positions = Vec::new();

        for position in self.carts.keys() {
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

        for old_position in positions {
            if !self.carts.contains_key(&old_position) {
                continue;
            }
            let mut cart = self.carts.remove(&old_position).unwrap();

            match cart.direction {
                Direction::North => cart.row -= 1,
                Direction::South => cart.row += 1,
                Direction::West => cart.col -= 1,
                Direction::East => cart.col += 1,
            }

            let new_position = Coord{row: cart.row, col: cart.col};
            let track_piece = self.rows[cart.row][cart.col];

            if track_piece == ' ' {
               self.errors.insert(new_position.clone());
            }

            if track_piece == '+' {
                match cart.next_turn {
                    Turn::Left => {
                        cart.next_turn = Turn::Straight;
                        cart.direction = match cart.direction {
                            Direction::North => Direction::West,
                            Direction::South => Direction::East,
                            Direction::West => Direction::South,
                            Direction::East => Direction::North,
                        }
                    },
                    Turn::Straight => {
                        cart.next_turn = Turn::Right;
                    },
                    Turn::Right => {
                        cart.next_turn = Turn::Left;
                        cart.direction = match cart.direction {
                            Direction::North => Direction::East,
                            Direction::South => Direction::West,
                            Direction::West => Direction::North,
                            Direction::East => Direction::South,
                        }
                    },
                }
            } else if track_piece == '/' {
                cart.direction = match cart.direction {
                    Direction::North => Direction::East,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                    Direction::East => Direction::North,
                }
            } else if track_piece == '\\' {
                cart.direction = match cart.direction {
                    Direction::North => Direction::West,
                    Direction::South => Direction::East,
                    Direction::West => Direction::North,
                    Direction::East => Direction::South,
                }
            }

            if self.carts.contains_key(&new_position) {
                self.carts.remove(&new_position);
                self.crashes.insert(new_position);
            } else {
                self.carts.insert(new_position, cart);
            }
        }
    }
}

impl fmt::Display for TrackSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (r, row) in self.rows.iter().enumerate() {
            write!(f, "{:03}", r);
            for (c, col) in row.iter().enumerate() {
                let coord = Coord{row: r, col: c};
                if self.crashes.contains(&coord) {
                    write!(f, "X");
                }
                else if self.carts.contains_key(&coord) {
                    let cart = self.carts.get(&coord).unwrap();
                    write!(f, "{}", cart.to_char());
                } else {
                    write!(f, "{}", col);
                }
            }
            writeln!(f, "");
        }

        write!(f, "")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { panic!("Too few arguments!") }

    let part = args[2].parse::<usize>().unwrap();

    let mut s = String::new();
    let f = File::open(&args[1]).expect("File not found!");
    let mut reader = BufReader::new(&f);
    reader.read_to_string(&mut s).expect("Error reading file into string!");

    let mut track_system = TrackSystem::new(&s);

    if part == 1 {
        while track_system.errors.len() < 1 && track_system.crashes.len() < 1 {
            track_system.forward();
        }

        let first_crash = track_system.crashes.iter().next().unwrap();
        println!("{},{}", first_crash.col, first_crash.row);
    } else {
        while track_system.errors.len() < 1 && track_system.carts.len() > 1 {
            track_system.forward();
        }
        if track_system.carts.len() == 1 {
            let (&last_coord,_) = track_system.carts.iter().next().unwrap();
            println!("{},{}", last_coord.col, last_coord.row);
        } else {
            println!("Even number of carts!");
        }
    }
}
