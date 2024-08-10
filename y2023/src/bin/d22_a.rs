use std::collections::HashSet;

use y2023::util::d22::{get_block_supports, D22Error};

fn solve(fp: &str) -> Result<u32, D22Error> {
    let block_supports = get_block_supports(fp)?;

    let mut disintegratable: HashSet<_> = (0..block_supports.len()).collect();
    for supports in block_supports {
        if supports.len() == 1 && supports[0] != 0 {
            disintegratable.remove(&(supports[0] - 1));
        }
    }

    Ok(disintegratable.len() as u32)
}

fn main() {
    match solve("data/d22/a.txt") {
        Ok(sol) => println!("{}", sol),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
