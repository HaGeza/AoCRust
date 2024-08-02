use y2023::util::d16::{get_number_of_energized, read_grid, D16Error};

fn solve(fp: &str) -> Result<u64, D16Error> {
    Ok(get_number_of_energized(&read_grid(fp)?, (0, 0), 0))
}

fn main() {
    match solve("data/d16/a.txt") {
        Ok(sol) => println!("{}", sol),
        Err(e) => eprintln!("Error: {}", e),
    }
}
