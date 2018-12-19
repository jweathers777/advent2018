use std::env;

fn create_new_recipes(recipes: &mut Vec<usize>, index_one: &mut usize, index_two: &mut usize) {
    let score_one = recipes[*index_one] as usize;
    let score_two = recipes[*index_two] as usize;

    let mut n: usize = score_one + score_two;
    let mut digits: Vec<usize> = Vec::new();

    if n == 0 {
        digits.push(0);
    } else {
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        digits.reverse();
    }

    recipes.append(&mut digits);

    let m = recipes.len() as usize;

    *index_one = (*index_one + score_one + 1usize) % m;
    *index_two = (*index_two + score_two + 1usize) % m;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Too few arguments!") }

    let recipe_count = args[1].parse::<usize>().unwrap();
    //let part = args[2].parse::<usize>().unwrap();

    let mut recipes: Vec<usize> = Vec::new();
    recipes.push(3);
    recipes.push(7);

    let mut index_one = 0;
    let mut index_two = 1;

    let full_recipe_count = recipe_count + 10;
    for _ in 0..full_recipe_count {
        create_new_recipes(&mut recipes, &mut index_one, &mut index_two);
    }

    let last_recipes = &recipes[recipe_count..full_recipe_count];
    for r in last_recipes {
        print!("{}", r);
    }
    println!("");
}
