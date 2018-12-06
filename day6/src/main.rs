use std::env;
use std::fmt;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Eq,PartialEq,PartialOrd,Ord)]
struct Point {
    x: u32,
    y: u32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
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

    let part: u32 = args[2].parse().expect("Invalid part!");

    let points: Vec<Point> = reader.
        lines().
        map(|l| l.unwrap().trim().parse::<Point>().expect("Invalid point!")).
        collect();

    for point in points {
        println!("{}", point);
    }
}
