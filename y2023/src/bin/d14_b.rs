use y2023::util::d14::{
    get_grid,
    Cell::{self, *},
    D14Error,
};

fn tilt_grid(grid: &mut Vec<Vec<Cell>>) {
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

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == Round {
                grid[i][j] = Empty;
                grid[i - fall[i][j] as usize][j] = Round;
            }
        }
    }
}

fn rotate_grid_clockwise(grid: &mut Vec<Vec<Cell>>) {
    let n = grid.len();
    let m = grid[0].len();
    let mut new_grid = vec![vec![Empty; n]; m];

    for i in 0..n {
        for j in 0..m {
            new_grid[j][n - 1 - i] = grid[i][j];
        }
    }

    *grid = new_grid;
}

fn get_north_support(grid: &Vec<Vec<Cell>>) -> u64 {
    let n = grid.len();
    let m = grid[0].len();
    let mut north_support = 0;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == Round {
                north_support += (n - i) as u64;
            }
        }
    }

    north_support
}

fn get_round_positions(grid: &Vec<Vec<Cell>>) -> Vec<(usize, usize)> {
    let n = grid.len();
    let m = grid[0].len();
    let mut positions = vec![];

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == Round {
                positions.push((i, j));
            }
        }
    }

    positions
}

fn place_rounds(grid: &Vec<Vec<Cell>>, positions: &Vec<(usize, usize)>) -> Vec<Vec<Cell>> {
    let n = grid.len();
    let m = grid[0].len();
    let mut new_grid = grid.clone();

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == Round {
                new_grid[i][j] = Empty;
            }
        }
    }

    for (i, j) in positions {
        new_grid[*i][*j] = Round;
    }

    new_grid
}

fn solve(fp: &str) -> Result<u64, D14Error> {
    let mut grid = get_grid(fp)?;
    let mut arrangements = vec![get_round_positions(&grid)];

    let max_iter = 1000000000;

    for _ in 0..max_iter {
        for _ in 0..4 {
            tilt_grid(&mut grid);
            rotate_grid_clockwise(&mut grid);
        }
        let round_positions = get_round_positions(&grid);
        if arrangements.contains(&round_positions) {
            let cycle_start = arrangements
                .iter()
                .position(|x| *x == round_positions)
                .unwrap();
            let cycle_len = arrangements.len() - cycle_start;

            let final_positions =
                arrangements[cycle_start + (max_iter - cycle_start) % cycle_len].clone();
            let final_grid = place_rounds(&grid, &final_positions);
            return Ok(get_north_support(&final_grid));
        }
        arrangements.push(round_positions);
    }
    // In case there is no cycle
    Ok(get_north_support(&grid))
}

fn main() {
    match solve("data/d14/a.txt") {
        Ok(result) => println!("{}", result),
        Err(error) => println!("Error: {}", error),
    }
}
