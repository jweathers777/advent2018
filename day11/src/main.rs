use std::env;

fn power_level(x: u32, y: u32, serial_number: u32) -> i32 {
    let rack_id = x + 10;
    let pl = ((rack_id * y) + serial_number) * rack_id;
    ((pl / 100) % 10) as i32 - 5
}

fn max_power_square(grid: &[[i32; 300]; 300], size: usize) -> (usize, usize, i32) {
    let adj = size - 1;
    let base_max_power_level = -300*((size*size) as i32);

    let mut grid_rows = grid.len();
    let mut grid_cols = grid[0].len();

    grid_cols -= adj;
    let mut col_reduced_grid = vec![vec![0i32; grid_cols]; grid_rows];

    for row in 0..grid_rows {
        for col in 0..grid_cols {
            for dcol in 0usize..size {
                col_reduced_grid[row][col] += grid[row][col+dcol];
            }
        }
    }

    grid_rows -= adj;
    let mut reduced_grid = vec![vec![0i32; grid_cols]; grid_rows];

    for col in 0..grid_cols {
        for row in 0..grid_rows {
            for drow in 0usize..size {
                reduced_grid[row][col] += col_reduced_grid[row+drow][col];
            }
        }
    }

    let mut max_power_level = base_max_power_level;
    let mut left_x = 1usize;
    let mut top_y = 1usize;

    for row in 0..grid_rows {
        for col in 0..grid_cols {
            let value = reduced_grid[row][col];
            if max_power_level < value {
                max_power_level = value;
                left_x = col + 1;
                top_y = row + 1;
            }
        }
    }

    (top_y, left_x, max_power_level)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let serial_number: u32 = args[1].parse().expect("Invalid serial number!");

    let mut grid = [[0i32; 300]; 300];

    for row in 0..300 {
        for col in 0..300 {
            let x = row + 1;
            let y = col + 1;
            grid[row][col] = power_level(x as u32, y as u32, serial_number);
        }
    }

    let mut best_top_y: usize = 1;
    let mut best_left_x: usize = 1;
    let mut best_size: usize = 1;
    let mut best_max_power_level = -300*9;

    for size in 1usize..300 {
        let (top_y, left_x, max_power_level) =
            max_power_square(&grid, size);

        if best_max_power_level < max_power_level {
            best_top_y = top_y;
            best_left_x = left_x;
            best_size = size;
            best_max_power_level = max_power_level;
        }
    }

    println!("({}, {}, {}) produces {}", best_top_y, best_left_x, best_size, best_max_power_level);
}
