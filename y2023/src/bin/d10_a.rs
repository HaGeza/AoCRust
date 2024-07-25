use y2023::util::d10::{get_filled_matrix, Cell, D10Error};

fn solve(fp: &str) -> Result<i32, D10Error> {
    let matrix = get_filled_matrix(fp)?;

    let mut solution = 0;
    for row in &matrix {
        for cell in row {
            match cell {
                Cell::Pipe(_, _, Some(distance)) => solution = solution.max(*distance),
                _ => (),
            }
        }
    }

    Ok(solution)
}

fn main() {
    match solve("data/d10/a.txt") {
        Ok(distance) => println!("{}", distance),
        Err(e) => eprintln!("{}", e),
    }
}
