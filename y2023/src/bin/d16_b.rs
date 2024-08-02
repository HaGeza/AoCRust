use y2023::util::d16::{get_number_of_energized, read_grid, D16Error};

fn solve(fp: &str) -> Result<u64, D16Error> {
    let grid = read_grid(fp)?;

    let mut max_sum = 0;
    let n = grid.len();
    let m = grid[0].len();

    for i in 0..n {
        let from_left = get_number_of_energized(&grid, (i as i32, 0), 0);
        let from_right = get_number_of_energized(&grid, (i as i32, m as i32 - 1), 2);
        max_sum = max_sum.max(from_left).max(from_right);
    }
    for j in 0..m {
        let from_top = get_number_of_energized(&grid, (0, j as i32), 1);
        let from_bottom = get_number_of_energized(&grid, (n as i32 - 1, j as i32), 3);
        max_sum = max_sum.max(from_top).max(from_bottom);
    }

    Ok(max_sum)
}

fn main() {
    match solve("data/d16/a.txt") {
        Ok(sol) => println!("{}", sol),
        Err(e) => eprintln!("Error: {}", e),
    }
}
