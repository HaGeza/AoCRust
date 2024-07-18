use y2023::get_lines;
use y2023::util::d4::{get_winning_set_and_own_array, D4Error};

fn total_reward(fp: &str) -> Result<u32, D4Error> {
    let mut total = 0;

    let mut num_cards = vec![1];

    for line in get_lines(fp)? {
        let (winning_set, own) = get_winning_set_and_own_array(&line?)?;
        let act_cards = match num_cards.first() {
            Some(nc) => *nc,
            None => {
                num_cards.push(1);
                1
            }
        };
        total += act_cards;

        let mut matches = 0;
        for o in own {
            if winning_set.contains(&o) {
                matches += 1;
                match num_cards.get_mut(matches) {
                    Some(nc) => *nc += act_cards,
                    None => num_cards.push(1 + act_cards),
                }
            }
        }
        num_cards.remove(0);
    }

    Ok(total)
}

fn main() {
    match total_reward("data/d4/a.txt") {
        Ok(reward) => println!("{}", reward),
        Err(e) => eprintln!("Error: {}", e),
    }
}
