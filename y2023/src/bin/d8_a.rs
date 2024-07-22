use y2023::util::d8::{read_environment, steps_from_a_to_b, D8Error, END_NODE, START_NODE};

fn solve(fp: &str) -> Result<u64, D8Error> {
    let (moves, nodes) = read_environment(fp)?;
    steps_from_a_to_b(
        &moves,
        &nodes,
        START_NODE.to_string(),
        &vec![END_NODE.to_string()],
    )
}

fn main() {
    match solve("data/d8/a.txt") {
        Ok(steps) => println!("{}", steps),
        Err(e) => eprintln!("{}", e),
    }
}
