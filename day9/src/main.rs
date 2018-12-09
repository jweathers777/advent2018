use std::env;

#[allow(dead_code)]
fn print_marbles(current_marble_index: usize, marbles: &Vec<usize>) {
    let last_idx = marbles.len() - 1;
    for (idx, marble) in marbles.iter().enumerate() {
        if idx == current_marble_index {
            print!("({})", marble);
        } else {
            print!("{}", marble);
        }
        if idx == last_idx {
            println!("");
        } else {
            print!("  ");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Too few arguments!") }

//    let part: u32 = args[2].parse().expect("Error reading part!");
    let player_count: usize = args[1].parse().expect("Error reading number of players!");
    let last_marble_value: usize = args[2].parse().expect("Error reading last marble value");

    let mut marble_circle = Vec::new();
    let mut player_scores = vec![0u32; player_count];

    let mut current_player = 0;
    let mut current_marble_index = 0usize;
    let mut current_marble_value = 0usize;
    marble_circle.push(current_marble_value);

    while current_marble_value <= last_marble_value {
        let marble_circle_size = marble_circle.len() as u32;

        current_marble_value += 1;

        if current_marble_value % 23 == 0 {
            player_scores[current_player] += current_marble_value as u32;
            let shifted_index = current_marble_index as u32 + marble_circle_size - 7;
            current_marble_index = (shifted_index % marble_circle_size) as usize;
            let removed_marble_value = marble_circle.remove(current_marble_index);
            player_scores[current_player] += removed_marble_value as u32;

        } else {
            let shifted_index = current_marble_index as u32 + 1;
            current_marble_index = (shifted_index % marble_circle_size) as usize + 1;
            marble_circle.insert(current_marble_index, current_marble_value);
        }

        current_player = (current_player + 1) % player_count;
    }

    let winning_score = player_scores.iter().max().unwrap();
    println!("Winning Score: {}", winning_score);
}
