use lazy_static::lazy_static;
use std::collections::HashMap;

use thiserror::Error;
use y2023::get_lines;

#[derive(Debug, Error)]
enum D7Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("parse error")]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
}

const CARD_VALUES: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

// Lazy static initialization for the HashMap
lazy_static! {
    static ref CARD_TO_VALUE: HashMap<char, u8> = {
        let mut m = HashMap::new();
        for (i, &c) in CARD_VALUES.iter().enumerate() {
            m.insert(c, i as u8);
        }
        m
    };
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Score {
    hand_score: u8,
    card_scores: (u8, u8, u8, u8, u8),
}

impl Score {
    fn new(cards_str: &str) -> Result<Self, D7Error> {
        if cards_str.len() != 5 {
            return Err(D7Error::InvalidInput);
        }

        let cards = cards_str.chars().collect::<Vec<_>>();

        let counts_map = cards.iter().fold(HashMap::new(), |mut acc, &c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        let mut counts = counts_map.values().collect::<Vec<_>>();

        counts.sort();
        counts.reverse();

        let hand_score = if *counts[0] == 5 {
            6
        } else if *counts[0] == 4 {
            5
        } else if *counts[0] == 3 && *counts[1] == 2 {
            4
        } else if *counts[0] == 3 {
            3
        } else if *counts[0] == 2 && *counts[1] == 2 {
            2
        } else if *counts[0] == 2 {
            1
        } else {
            0
        };

        let card_scores = (
            CARD_TO_VALUE[&cards[0]],
            CARD_TO_VALUE[&cards[1]],
            CARD_TO_VALUE[&cards[2]],
            CARD_TO_VALUE[&cards[3]],
            CARD_TO_VALUE[&cards[4]],
        );

        Ok(Self {
            hand_score,
            card_scores,
        })
    }
}

fn get_total_winnings(fp: &str) -> Result<u64, D7Error> {
    let mut bids = vec![];
    let mut scores = vec![];

    for line in get_lines(fp)? {
        let line = line?;
        let (cards_str, bid_str) = line.split_once(" ").ok_or(D7Error::InvalidInput)?;

        bids.push(bid_str.parse::<u64>()?);
        scores.push(Score::new(cards_str)?);
    }

    let mut scores_w_ind = scores.iter().enumerate().collect::<Vec<_>>();
    scores_w_ind.sort_by(|(_, a), (_, b)| a.cmp(b));

    Ok(scores_w_ind
        .iter()
        .enumerate()
        .map(|(r, (i, _))| (r + 1) as u64 * bids[*i])
        .sum::<u64>())
}

fn main() {
    match get_total_winnings("data/d7/a.txt") {
        Ok(winnings) => println!("{}", winnings),
        Err(e) => println!("Error: {}", e),
    }
}
