use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn common_chars(a: &String, b: &String) -> String {
    a.chars().zip(b.chars()).filter(|(x,y)| x == y).map(|(x,_)| x).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).expect("File not found!");
    let reader = BufReader::new(&f);

    let box_ids: Vec<String> = reader.
        lines().
        map(|l| l.unwrap().trim().to_string()).
        collect();

    let box_count = box_ids.len();

    for i in 0..box_count {
        let box_id_one = &box_ids[i];

        for j in (i+1)..box_count {
            let box_id_two = &box_ids[j];
            if box_id_one.len() == box_id_two.len() {
                let common = common_chars(box_id_one, box_id_two);
                if common.len() == box_id_one.len() - 1 {
                    println!("{}", common);
                    return
                }
            }
        }
    }
}
