use std::env;
use std::fmt;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::ops::Add;
use std::ops::Div;
use std::collections::HashMap;
use std::collections::HashSet;


#[derive(Copy,Clone,Hash,Eq,PartialEq,PartialOrd,Ord)]
struct Point {
    x: u32,
    y: u32,
}

fn manhattan_distance(p: &Point, q: &Point) -> u32 {
    let delta_x = if p.x >= q.x { p.x - q.x } else { q.x - p.x };
    let delta_y = if p.y >= q.y { p.y - q.y } else { q.y - p.y };

    delta_x + delta_y
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Add<u32> for Point {
    type Output = Point;

    fn add(self, other: u32) -> Point {
        Point {x: self.x + other, y: self.y + other}
    }
}
impl<'a> Add<&'a Point> for Point {
    type Output = Point;

    fn add(self, other: &'a Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Div<u32> for Point {
    type Output = Point;

    fn div(self, other: u32) -> Point {
        Point {x: self.x / other, y: self.y / other}
    }
}
impl<'a> Div<&'a Point> for Point {
    type Output = Point;

    fn div(self, other: &'a Point) -> Point {
        Point {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl std::str::FromStr for Point {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
            .split(',')
            .map(|x| x.trim())
            .collect();

        let x_fromstr = coords[0].parse::<u32>()?;
        let y_fromstr = coords[1].parse::<u32>()?;

        Ok(Point { x: x_fromstr, y: y_fromstr  })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 { panic!("Too few arguments!") }

    let f = File::open(&args[1]).expect("File not found!");
    let reader = BufReader::new(&f);

    //let part: u32 = args[2].parse().expect("Invalid part!");

    let points: Vec<Point> = reader.
        lines().
        map(|l| l.unwrap().trim().parse::<Point>().expect("Invalid point!")).
        collect();

    let min_x = 0;
    let min_y = 0;
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    let mut infinite = HashSet::new();
    let mut areas = HashMap::new();

    for i in min_x..=max_x {
        for j in min_y..=max_y {
            let q = Point{x: i, y: j};

            let distances: Vec<u32> = points.iter().
                map(|p| manhattan_distance(&p, &q)).collect();

            let min_distance = distances.iter().min().unwrap();

            let closest: Vec<Point> = points.iter().zip(distances.iter()).
                filter(|(_,d)| *d == min_distance).
                map(|(p,_)| *p).
                collect();

            if closest.len() == 1 {
                let p = closest[0].clone();
                let area = areas.entry(p).or_insert(0);
                *area += 1;
                if i == min_x || i == max_x || j == min_y || j == max_y {
                    infinite.insert(p);
                }
            }
        }
    }

    for p in infinite.iter() {
        areas.remove(p);
    }

    let max_p = areas.keys().max_by_key(|p| areas.get(*p).unwrap()).unwrap();
    let max_area = areas.get(max_p).unwrap();

    println!("{}: {}", max_p, max_area);
}
