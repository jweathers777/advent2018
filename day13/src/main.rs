use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::fmt;
use std::env;
use std::collections::HashSet;
use std::collections::HashMap;

enum Direction {
    North, South, East, West
}

enum Turn {
    Left, Straight, Right
}

#[derive(PartialEq, Eq, Hash)]
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
}

impl TrackSystem {
    fn new(s: &str) -> TrackSystem {
        let mut rows = Vec::new();
        let mut carts = Vec::new();

        for (row, line) in s.lines().enumerate() {
            let mut cols = Vec::new();
            for (col, c) in line.chars().enumerate() {
                match c {
                    '^' => carts.push(Cart::new(row, col,Direction::North)),
                    'v' => carts.push(Cart::new(row, col, Direction::South)),
                    '<' => carts.push(Cart::new(row, col, Direction::West)),
                    '>' => carts.push(Cart::new(row, col, Direction::East)),
                    _ => {},
                }
                cols.push(c);
            }
            rows.push(cols);
        }

        let min_row_index: usize = 0;
        let max_row_index: usize = rows.len() - 1;
        let min_col_index: usize = 0;
        let max_col_index: usize = rows[0].len() - 1;

        for cart in &carts {
            let adj_north = if cart.row <= min_row_index { ' ' } else { rows[cart.row-1][cart.col] };
            let adj_south = if cart.row >= max_row_index { ' ' } else { rows[cart.row+1][cart.col] };
            let adj_west = if cart.col <= min_col_index { ' ' } else { rows[cart.row][cart.col-1] };
            let adj_east = if cart.col >= max_col_index { ' ' } else { rows[cart.row][cart.col+1] };

            rows[cart.row][cart.col] =
                if adj_north == ' ' && adj_west == ' ' {
                    '/'
                } else if adj_north == ' ' && adj_east == ' ' {
                    '\\'
                } else if adj_south == ' ' && adj_west == ' ' {
                    '\\'
                } else if adj_south ==  ' ' && adj_east == ' ' {
                    '/'
                } else if adj_north != ' ' && adj_south != ' ' &&
                    adj_west != ' ' && adj_east != ' ' {
                    '+'
                } else if adj_north != ' ' {
                    '|'
                } else {
                    '-'
                }
        }

        TrackSystem{rows, carts, crashes: HashSet::new()}
    }

    fn forward(&mut self) {
        let mut new_positions = HashSet::new();

        for cart in self.carts.iter_mut() {
            match cart.direction {
                Direction::North => cart.row -= 1,
                Direction::South => cart.row += 1,
                Direction::West => cart.col -= 1,
                Direction::East => cart.col += 1,
            }

            let position = Coord{row: cart.row, col: cart.col};
            if new_positions.contains(&position) {
                self.crashes.insert(position);
            } else {
                new_positions.insert(position);
            }

            let track_piece = self.rows[cart.row][cart.col];

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

    println!("{}", track_system);

    while track_system.crashes.len() < 1 {
        track_system.forward();

        println!("{}", track_system);
    }
}
