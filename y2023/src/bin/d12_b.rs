use y2023::{
    get_lines,
    util::d12::{
        get_springs_and_nums, D12Error,
        Spring::{self, *},
    },
};

fn num_arrangements_dp(springs: Vec<Spring>, nums: Vec<u32>) -> u64 {
    let mut endings = vec![];
    for num in nums {
        endings.push(Broken);
        for _ in 1..=num {
            endings.push(Working);
        }
    }
    endings.push(Broken);

    let n = endings.len();
    let m = springs.len();
    let mut dp = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            let broken = springs[j] == Unknown || springs[j] == Broken;
            let working = springs[j] == Unknown || springs[j] == Working;

            // In the first column
            if j == 0 {
                if i == 0 {
                    dp[i][j] = broken as u64;
                } else if i == 1 {
                    dp[i][j] = working as u64;
                }
                continue;
            }

            // Not in the first column, but in the first row
            if i == 0 {
                dp[i][j] = dp[i][j - 1] * broken as u64;
            // Neither first row nor first column
            } else if endings[i] == Working {
                // If the current has to end in Working, it can only extend a sequence with one less Working
                dp[i][j] = dp[i - 1][j - 1] * working as u64;
            } else {
                // endings[i] == Broken
                // If the current has to end in Broken, it can extend a sequence ending in Working or Broken
                dp[i][j] = (dp[i - 1][j - 1] + dp[i][j - 1]) * broken as u64;
            }
        }
    }

    dp[n - 2][m - 1] + dp[n - 1][m - 1]
}

fn num_arrangements(line: String) -> Result<u64, D12Error> {
    let (springs, nums) = get_springs_and_nums(line)?;
    let mut springs_long = springs.clone();
    let mut nums_long = nums.clone();

    for _ in 1..5 {
        springs_long = [springs_long.clone(), vec![Unknown], springs.clone()].concat();
        nums_long = [nums_long.clone(), nums.clone()].concat();
    }

    Ok(num_arrangements_dp(springs_long, nums_long))
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
