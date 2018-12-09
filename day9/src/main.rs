use std::env;
use std::collections::LinkedList;


struct Loop<T> {
    list: LinkedList<T>,
}

impl<T> Loop<T> {
    fn new() -> Loop<T> {
        Loop{list: LinkedList::new()}
    }

    fn push(&mut self, elem: T) {
        self.list.push_back(elem)
    }

    fn pop(&mut self) -> Option<T> {
        self.list.pop_back()
    }

    fn rotate_front_to_back(&mut self, steps: u32) {
        for _ in 0..steps {
            let elem = self.list.pop_front().unwrap();
            self.list.push_back(elem);
        }
    }

    fn rotate_back_to_front(&mut self, steps: u32) {
        for _ in 0..steps {
            let elem = self.list.pop_back().unwrap();
            self.list.push_front(elem);
        }
    }

    fn rotate_clockwise(&mut self, steps: u32) {
        let n = self.list.len() as u32;

        if n > 1 {
            let right_steps = steps % n;
            let left_steps = n - right_steps;
            if right_steps <= left_steps {
                self.rotate_back_to_front(right_steps);
            } else {
                self.rotate_front_to_back(left_steps);
            }
        }
    }

    fn rotate_counter_clockwise(&mut self, steps: u32) {
        let n = self.list.len() as u32;

        if n > 1 {
            let left_steps = steps % n;
            let right_steps = n - left_steps;
            if right_steps <= left_steps {
                self.rotate_back_to_front(right_steps);
            } else {
                self.rotate_front_to_back(left_steps);
            }
        }
    }
}

impl<T> std::fmt::Display for Loop<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let last_index = self.list.len() - 1;
        for (index, elem) in self.list.iter().enumerate() {
            if index == last_index {
                write!(f, "({})", elem);
            } else {
                write!(f, "{}  ", elem);
            }
        }
        write!(f, "")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Too few arguments!") }

    let player_count: usize = args[1].parse().expect("Error reading number of players!");
    let last_marble_value: u32 = args[2].parse().expect("Error reading last marble value");

    let mut marble_circle: Loop<u32> = Loop::new();
    let mut player_scores = vec![0u32; player_count];

    let mut current_player = 0;
    let mut current_marble_value = 0u32;
    marble_circle.push(current_marble_value);

    while current_marble_value <= last_marble_value {
        current_marble_value += 1;

        if current_marble_value % 23 == 0 {
            player_scores[current_player] += current_marble_value;
            marble_circle.rotate_clockwise(7);
            let removed_marble_value = marble_circle.pop().expect("Missing marble!");
            player_scores[current_player] += removed_marble_value;
            marble_circle.rotate_counter_clockwise(1);

        } else {
            marble_circle.rotate_counter_clockwise(1);
            marble_circle.push(current_marble_value);
        }

        current_player = (current_player + 1) % player_count;
    }

    let winning_score = player_scores.iter().max().unwrap();
    println!("Winning Score: {}", winning_score);
}
