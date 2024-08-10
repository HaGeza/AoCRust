use std::collections::HashSet;

use y2023::util::d22::{get_block_supports, D22Error};

fn chain_reaction(
    b_i: usize,
    block_supports: &Vec<Vec<usize>>,
    supported_blocks: &Vec<Vec<usize>>,
    missing: &mut HashSet<usize>,
) -> u64 {
    if block_supports[b_i - 1]
        .iter()
        .any(|&i| !missing.contains(&i))
    {
        0
    } else {
        missing.insert(b_i);
        let mut sum = 1;
        for &i in &supported_blocks[b_i - 1] {
            if missing.contains(&i) {
                continue;
            }
            sum += chain_reaction(i, block_supports, supported_blocks, missing);
        }
        sum
    }
}

fn solve(fp: &str) -> Result<u64, D22Error> {
    let block_supports = get_block_supports(fp)?;
    let mut supported_blocks = vec![vec![]; block_supports.len()];
    for (i, supports) in block_supports.iter().enumerate() {
        for &support in supports {
            if support != 0 {
                supported_blocks[support as usize - 1].push(i + 1);
            }
        }
    }

    let mut total = 0;
    for b_i in 1..=supported_blocks.len() {
        let mut missing = HashSet::from([b_i]);
        total += supported_blocks[b_i - 1]
            .iter()
            .map(|&i| chain_reaction(i, &block_supports, &supported_blocks, &mut missing))
            .sum::<u64>();
    }

    Ok(total)
}

fn main() {
    match solve("data/d22/a.txt") {
        Ok(sol) => println!("{}", sol),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
