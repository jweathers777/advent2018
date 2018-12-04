use std::env;
use std::fmt;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Copy,Clone)]
struct Rectangle {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Rectangle {
    fn parse(rectangle: &str) -> Rectangle {
        let tokens: Vec<&str> = rectangle.split_whitespace().collect();
        let id: usize = tokens[0].trim_start_matches('#').parse().expect("Bad id!");

        let offsets: Vec<usize> = tokens[2].trim_end_matches(':').
            split(',').
            map(|off| off.parse().expect("Bad offsets!")).
            collect();

        let dims: Vec<usize> = tokens[3].split('x').
            map(|dim| dim.parse().expect("Bad dimensions!")).
            collect();

        Rectangle {id: id, left: offsets[0], top: offsets[1], width: dims[0], height: dims[1] }
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} @ {},{}: {}x{}", self.id, self.left, self.top, self.width, self.height)
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 { panic!("Too few arguments!") }

    let f = File::open(&args[1]).expect("File not found!");
    let reader = BufReader::new(&f);

    let part: u32 = args[2].parse().expect("Invalid part!");

    let claims: Vec<Rectangle> = reader.
        lines().
        map(|l| Rectangle::parse(l.unwrap().trim())).
        collect();

    let mut max_right: usize = 0;
    let mut max_bottom: usize = 0;

    for claim in &claims {
        let right = claim.left + claim.width;
        let bottom = claim.top + claim.height;

        if max_right < right { max_right = right };
        if max_bottom < bottom { max_bottom = bottom };
    }

    let mut grid: Vec<Vec<usize>> = vec![vec![0; max_right]; max_bottom];

    for claim in &claims {
        let right = claim.left + claim.width;
        let bottom = claim.top + claim.height;

        for i in claim.left..right {
            for j in claim.top..bottom {
                grid[j][i] += 1
            }
        }
    }

    if part == 1 {
        let mut overclaimed_total = 0;

        for row in &grid {
            for col in row {
                if *col > 1 {
                    overclaimed_total += 1;
                }
            }
        }

        println!("{}", overclaimed_total);
    } else {
        let mut intact_claim_id = 0;

        'claim_loop: for claim in &claims {
            let right = claim.left + claim.width;
            let bottom = claim.top + claim.height;

            for i in claim.left..right {
                for j in claim.top..bottom {
                    if grid[j][i] > 1 { continue 'claim_loop };
                }
            }

            intact_claim_id = claim.id;
            break;
        }

        println!("{}", intact_claim_id);
    }
}
