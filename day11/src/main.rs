use std::env;

fn power_level(x: u32, y: u32, serial_number: u32) -> i32 {
    let rack_id = x + 10;
    let pl = ((rack_id * y) + serial_number) * rack_id;
    ((pl / 100) % 10) as i32 - 5
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let serial_number: u32 = args[1].parse().expect("Invalid serial number!");

    let mut grid_rows = 300usize ;
    let mut grid_cols = 300usize;
    let mut grid = vec![vec![0i32; grid_cols]; grid_rows];

    for row in 0..grid_rows {
        for col in 0..grid_cols {
            let x = row + 1;
            let y = col + 1;
            grid[row][col] = power_level(x as u32, y as u32, serial_number);
        }
    }

    grid_cols -= 2;
    let mut col_reduced_grid = vec![vec![0i32; grid_cols]; grid_rows];

    for row in 0..grid_rows {
        for col in 0..grid_cols {
            for dcol in 0usize..3 {
                col_reduced_grid[row][col] += grid[row][col+dcol];
            }
        }
    }

    grid_rows -= 2;
    let mut reduced_grid = vec![vec![0i32; grid_cols]; grid_rows];

    for col in 0..grid_cols {
        for row in 0..grid_rows {
            for drow in 0usize..3 {
                reduced_grid[row][col] += col_reduced_grid[row+drow][col];
            }
        }
    }

    let mut max_power_level = -300*9;
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

    println!("({}, {}) produces {}", top_y, left_x, max_power_level);
}
