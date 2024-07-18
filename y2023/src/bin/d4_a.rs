use y2023::get_lines;
use y2023::util::d4::{get_winning_set_and_own_array, D4Error};

fn total_reward(fp: &str) -> Result<u32, D4Error> {
    let mut total = 0;

    for line in get_lines(fp)? {
        let (winning_set, own) = get_winning_set_and_own_array(&line?)?;

        let mut reward = 0;
        for o in own {
            if winning_set.contains(&o) {
                if reward == 0 {
                    reward = 1;
                } else {
                    reward *= 2;
                }
            }
        }

        total += reward;
    }

    Ok(total)
}

fn main() {
    match total_reward("data/d4/a.txt") {
        Ok(reward) => println!("{}", reward),
        Err(e) => eprintln!("Error: {}", e),
    }
}
