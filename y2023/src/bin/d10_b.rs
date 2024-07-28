use y2023::util::d10::{get_filled_matrix, Cell, D10Error, Direction};

fn solve(fp: &str) -> Result<i32, D10Error> {
    let matrix = get_filled_matrix(fp)?;

    let mut inside = 0;
    let mut north_closed = false;
    let mut south_closed = false;

    for row in matrix {
        for cell in row {
            match cell {
                Cell::Pipe(d1, d2, Some(_)) => {
                    if d1 == Direction::N || d2 == Direction::N {
                        north_closed = !north_closed;
                    }
                    if d1 == Direction::S || d2 == Direction::S {
                        south_closed = !south_closed;
                    }
                }
                _ => inside += (north_closed && south_closed) as i32,
            }
        }
    }

    Ok(inside)
}

fn main() {
    match solve("data/d10/a.txt") {
        Ok(inside) => println!("{}", inside),
        Err(e) => eprintln!("{}", e),
    }
}
