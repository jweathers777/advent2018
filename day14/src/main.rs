use std::env;

fn digits_for(n: usize) -> Vec<usize> {
    let mut m = n;
    let mut digits = Vec::new();

    if m == 0 {
        digits.push(0);
    } else {
        while m > 0 {
            digits.push(m % 10);
            m /= 10;
        }

        digits.reverse();
    }
    digits
}

fn create_new_recipes(recipes: &mut Vec<usize>, index_one: &mut usize, index_two: &mut usize) {
    let score_one = recipes[*index_one] as usize;
    let score_two = recipes[*index_two] as usize;

    let n: usize = score_one + score_two;
    let mut digits = digits_for(n);

    recipes.append(&mut digits);

    let m = recipes.len() as usize;

    *index_one = (*index_one + score_one + 1usize) % m;
    *index_two = (*index_two + score_two + 1usize) % m;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Too few arguments!") }

    let part = args[2].parse::<usize>().unwrap();

    let mut recipes: Vec<usize> = Vec::new();
    recipes.push(3);
    recipes.push(7);

    let mut index_one = 0;
    let mut index_two = 1;

    if part == 1 {
        let recipe_count = args[1].parse::<usize>().unwrap();

        let full_recipe_count = recipe_count + 10;
        for _ in 0..full_recipe_count {
            create_new_recipes(&mut recipes, &mut index_one, &mut index_two);
        }

        let last_recipes = &recipes[recipe_count..full_recipe_count];
        for r in last_recipes {
            print!("{}", r);
        }
    } else {
        let digits: Vec<usize> = args[1].chars().
            map(|ch| ch.to_digit(10).unwrap() as usize).
            collect();
        let n = digits.len();
        let mut start = 0;

        loop {
            create_new_recipes(&mut recipes, &mut index_one, &mut index_two);

            let m = recipes.len();
            if m >= n {
                let mut matches = true;

                for j in 0..2 {
                    if m >= n + j {
                        matches = true;
                        start = m - n - j;
                        let finish = start + n;
                        let last_recipes = &recipes[start..finish];

                        for i in 0..n {
                            if digits[i] != last_recipes[i] {
                                matches = false;
                                break;
                            }
                        }

                        if matches {
                            break;
                        }
                    } else {
                        matches = false;
                        break;
                    }
                }

                if matches {
                    break;
                }
            }
        }
        print!("{}", start);
    }
    println!("");
}
