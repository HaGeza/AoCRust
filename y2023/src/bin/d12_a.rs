use y2023::{
    get_lines,
    util::d12::{
        get_springs_and_nums, D12Error,
        Spring::{self, *},
    },
};

fn num_arrangements_backtrack(springs: Vec<Spring>, nums: Vec<u32>, next_spring: Spring) -> u64 {
    if springs.is_empty() {
        return nums.is_empty() as u64;
    }
    let nums_left = nums.iter().sum::<u32>() as i64;
    if (springs.len() as i64) < (nums_left + nums.len() as i64 - 1) {
        return 0;
    }

    let springs_left = springs.iter().filter(|&&c| c == Working).count() as i64;
    let unknowns_left = springs.iter().filter(|&&c| c == Unknown).count() as i64;
    if springs_left > nums_left || unknowns_left + springs_left < nums_left {
        return 0;
    }

    match (springs[0], next_spring) {
        (Broken, Broken) | (Unknown, Broken) | (Broken, Unknown) => {
            num_arrangements_backtrack(springs[1..].to_vec(), nums, Unknown)
        }
        (Working, Working) | (Unknown, Working) | (Working, Unknown) => {
            if nums.is_empty() {
                0
            } else {
                let (remaining_nums, new_next_char) = match nums[0] {
                    1 => (nums[1..].to_vec(), Broken),
                    num => ([vec![num - 1], nums[1..].to_vec()].concat(), Working),
                };
                num_arrangements_backtrack(springs[1..].to_vec(), remaining_nums, new_next_char)
            }
        }
        (Unknown, Unknown) => {
            num_arrangements_backtrack(springs.clone(), nums.clone(), Working)
                + num_arrangements_backtrack(springs, nums, Broken)
        }
        _ => 0,
    }
}

fn num_arrangements(line: String) -> Result<u64, D12Error> {
    let (springs, nums) = get_springs_and_nums(line)?;
    Ok(num_arrangements_backtrack(springs, nums, Unknown))
}

fn solve(fp: &str) -> Result<u64, D12Error> {
    let mut sum = 0;
    for line in get_lines(fp)? {
        sum += num_arrangements(line?)?;
    }
    Ok(sum)
}

fn main() {
    match solve("data/d12/a.txt") {
        Ok(solution) => println!("{}", solution),
        Err(e) => eprintln!("{}", e),
    }
}
