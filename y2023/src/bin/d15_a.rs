use y2023::{
    get_lines,
    util::d15::{simple_hash, D15Error},
};

fn solve(fp: &str) -> Result<u64, D15Error> {
    let Some(line) = get_lines(fp)?.next() else {
        return Err(D15Error::EmptyInput);
    };

    let mut sum = 0;
    for s in line?.split(',') {
        sum += simple_hash(s) as u64;
    }

    Ok(sum)
}

fn main() {
    match solve("data/d15/a.txt") {
        Ok(sol) => println!("{}", sol),
        Err(e) => println!("Error: {}", e),
    }
}
