use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::fmt;
use std::env;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Ordering;

enum Direction {
    North, South, East, West
}

enum Turn {
    Left, Straight, Right
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

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
    carts: Vec<Cart>,
    crashes: HashSet<Coord>,
    errors: HashSet<Coord>,
}

impl TrackSystem {
    fn new(s: &str) -> TrackSystem {
        let mut rows = Vec::new();
        let mut carts = Vec::new();

        for (row, line) in s.lines().enumerate() {
            let mut cols = Vec::new();
            for (col, c) in line.chars().enumerate() {
                match c {
                    '^' => {
                        cols.push('|');
                        carts.push(Cart::new(row, col,Direction::North))
                    },
                    'v' => {
                        cols.push('|');
                        carts.push(Cart::new(row, col, Direction::South))
                    },
                    '<' => {
                        cols.push('-');
                        carts.push(Cart::new(row, col, Direction::West))
                    },
                    '>' => {
                        cols.push('-');
                        carts.push(Cart::new(row, col, Direction::East))
                    },
                    _ => {
                        cols.push(c)
                    },
                }
            }
            rows.push(cols);
        }

        let min_row_index: usize = 0;
        let max_row_index: usize = rows.len() - 1;
        let min_col_index: usize = 0;
        let max_col_index: usize = rows.iter().map(|r| r.len()).max().unwrap() - 1;

        let vertical = ['|', '+', '/', '\\'];
        let horizontal = ['-', '+', '/', '\\'];

        TrackSystem{rows, carts, crashes: HashSet::new(), errors: HashSet::new()}
    }

    fn forward(&mut self) {
        let mut positions = HashSet::new();

        self.carts.sort_unstable_by(|a,b| {
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

        for cart in self.carts.iter() {
            let position = Coord{row: cart.row, col: cart.col};
            positions.insert(position.clone());
        }

        for cart in self.carts.iter_mut() {
            let old_position = Coord{row: cart.row, col: cart.col};

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

            if positions.contains(&new_position) {
                self.crashes.insert(new_position);
            } else {
                positions.remove(&old_position);
                positions.insert(new_position);
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
        }
    }
}

impl fmt::Display for TrackSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cart_positions = HashMap::new();

        for cart in &self.carts {
            cart_positions.insert(Coord{row: cart.row, col: cart.col}, cart);
        }

        for (r, row) in self.rows.iter().enumerate() {
            write!(f, "{:03}", r);
            for (c, col) in row.iter().enumerate() {
                let coord = Coord{row: r, col: c};
                if self.crashes.contains(&coord) {
                    write!(f, "X");
                }
                else if cart_positions.contains_key(&coord) {
                    let cart = cart_positions.get(&coord).unwrap();
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
    if args.len() < 2 { panic!("Too few arguments!") }

    let mut s = String::new();
    let f = File::open(&args[1]).expect("File not found!");
    let mut reader = BufReader::new(&f);
    reader.read_to_string(&mut s).expect("Error reading file into string!");

    let mut track_system = TrackSystem::new(&s);
    let mut tick = 1usize;

    while track_system.errors.len() < 1 && track_system.crashes.len() < 1 {
        println!("Tick: {}", tick);
        println!("{}", track_system);
        tick += 1;
        track_system.forward();
    }

    println!("Tick: {}", tick);
    println!("{}", track_system);
    let first_crash = track_system.crashes.iter().next().unwrap();
    println!("{},{}", first_crash.col, first_crash.row);
}
