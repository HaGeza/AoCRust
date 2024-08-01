use y2023::util::d14::{get_grid, Cell::*, D14Error};

fn solve(fp: &str) -> Result<u64, D14Error> {
    let grid = get_grid(fp)?;
    let n = grid.len();
    let m = grid[0].len();

    let mut fall = vec![vec![0; m]; n];

    for i in 1..n {
        for j in 0..m {
            fall[i][j] = match &grid[i - 1][j] {
                Square => 0,
                Round => fall[i - 1][j],
                Empty => 1 + fall[i - 1][j],
            }
        }
    }

    let mut sum = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == Round {
                let what = n - i + fall[i][j];
                let the = what as u64;
                sum += the;
            }
        }
    }
    Ok(sum)
}

fn main() {
    match solve("data/d14/a.txt") {
        Ok(result) => println!("{}", result),
        Err(error) => println!("Error: {}", error),
    }
}
