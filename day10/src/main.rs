use std::env;
use std::fmt;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::collections::HashMap;

#[derive(Copy,Clone,Eq,PartialEq,Hash)]
struct IndexPair {
    i: usize,
    j: usize,
}

#[derive(Copy,Clone,Hash,Eq,PartialEq,PartialOrd,Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl std::str::FromStr for Point {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '<' || p == '>' )
            .split(',')
            .map(|x| x.trim())
            .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point { x: x_fromstr, y: y_fromstr  })
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        *self = Point {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

#[derive(Copy,Clone,Hash,Eq,PartialEq,PartialOrd,Ord)]
struct Particle {
    position: Point,
    velocity: Point,
}

impl Particle {
    fn forward(& mut self) {
        self.position += self.velocity;
    }

    fn backward(& mut self) {
        self.position -= self.velocity;
    }
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "position={}  velocity={}", self.position, self.velocity)
    }
}

impl std::str::FromStr for Particle {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('v').collect();
        let position: Point = parts[0].
            trim_start_matches("position=").trim().parse()?;
        let velocity: Point = parts[1].
            trim_start_matches("elocity=").trim().parse()?;

        Ok(Particle { position: position, velocity: velocity })
    }
}

struct ParticleGrid {
    particles: HashMap<IndexPair,Vec<Particle>>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl ParticleGrid {
    fn new(particles: Vec<Particle>) -> ParticleGrid {
        let min_x = particles.iter().map(|p| p.position.x).min().unwrap();
        let max_x = particles.iter().map(|p| p.position.x).max().unwrap();

        let min_y = particles.iter().map(|p| p.position.y).min().unwrap();
        let max_y = particles.iter().map(|p| p.position.y).max().unwrap();

        let mut grid: HashMap<IndexPair, Vec<Particle>> = HashMap::new();
        for particle in particles.iter() {
            let col = (particle.position.x - min_x) as usize;
            let row = (particle.position.y - min_y) as usize;
            let mut position = grid.entry(IndexPair { i: row, j: col }).or_insert(vec![]);
            position.push(*particle);
        }

        ParticleGrid { particles: grid, min_x: min_x, max_x: max_x, min_y: min_y, max_y: max_y }
    }

    fn forward(&mut self) {
        for ps in self.particles.values_mut() {
            for particle in ps.iter_mut() {
                particle.forward();
            }
        }
        let particles: Vec<Particle> = self.particles.values().into_iter().
            flat_map(|ps| ps.into_iter()).
            map(|p| *p).
            collect();

        *self = ParticleGrid::new(particles);
    }

    fn backward(&mut self) {
        for ps in self.particles.values_mut() {
            for particle in ps.iter_mut() {
                particle.backward();
            }
        }
        let particles: Vec<Particle> = self.particles.values().into_iter().
            flat_map(|ps| ps.into_iter()).
            map(|p| *p).
            collect();

        *self = ParticleGrid::new(particles);
    }

    fn width(&self) -> u32 {
        (self.max_x - self.min_x) as u32
    }

    fn height(&self) -> u32 {
        (self.max_y - self.min_y) as u32
    }
}

impl fmt::Display for ParticleGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cols = (self.max_x - self.min_x + 1) as usize;
        let rows = (self.max_y - self.min_y + 1) as usize;

        let last_col = cols - 1;

        for row in 0..rows {
            for col in 0..cols {
                if self.particles.contains_key(&IndexPair{i: row, j: col}) {
                    write!(f, "#");
                } else {
                    write!(f, ".");
                }

                if col == last_col {
                    writeln!(f, "");
                } else {
                    write!(f, " ");
                }
            }
            writeln!(f, "");
        }
        writeln!(f, "")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Too few arguments!") }

    let f = File::open(&args[1]).expect("File not found!");
    let reader = BufReader::new(&f);

    let particles: Vec<Particle> = reader.
        lines().
        map(|l| l.unwrap().trim().parse::<Particle>().expect("Invalid particle!")).
        collect();

    let mut grid = ParticleGrid::new(particles);

    let mut last_width = grid.width();
    let mut last_height = grid.height();
    let mut i = 1u32;
    loop {
        grid.forward();

        let current_width = grid.width();
        let current_height = grid.height();
        if current_height > last_height || current_width > last_width {
            break;
        }
        last_height = current_height;
        last_width = current_width;
        i += 1;
    }
    i -= 1;
    println!("After {} second(s)", i);
    grid.backward();
    println!("{}", grid);
}
