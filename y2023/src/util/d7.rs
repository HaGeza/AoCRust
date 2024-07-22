use std::collections::HashMap;

use crate::get_lines;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum D7Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("parse error")]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
    #[error("unreachable: {0}")]
    Unreachable(&'static str),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Score {
    hand_score: u8,
    card_scores: (u8, u8, u8, u8, u8),
}

impl Score {
    fn apply_joker(
        counts_map: &mut HashMap<char, u8>,
        card_to_value: &HashMap<char, u8>,
        joker: char,
    ) -> Result<(), D7Error> {
        if !counts_map.contains_key(&joker) {
            return Ok(());
        }

        let mut most_frequent = vec![];
        let mut max_frequency = 0;
        for (&c, &count) in counts_map.iter() {
            if c == joker {
                continue;
            }

            if count > max_frequency {
                most_frequent = vec![c];
                max_frequency = count;
            } else if count == max_frequency {
                most_frequent.push(c);
            }
        }

        let best_card = if most_frequent.is_empty() {
            card_to_value
                .iter()
                .max_by_key(|(_, &v)| v)
                .map(|(&c, _)| c)
                .ok_or(D7Error::Unreachable("no cards in value map"))?
        } else {
            most_frequent
                .iter()
                .max_by_key(|&&c| card_to_value[&c])
                .ok_or(D7Error::Unreachable("no cards in non-empty most frequent"))?
                .clone()
        };

        let (_, joker_count) = counts_map.remove_entry(&joker).unwrap();
        *counts_map.entry(best_card).or_insert(0) += joker_count;
        Ok(())
    }

    fn new(
        cards_str: &str,
        card_to_value: &HashMap<char, u8>,
        joker: Option<char>,
    ) -> Result<Self, D7Error> {
        if cards_str.len() != 5 {
            return Err(D7Error::InvalidInput);
        }

        let cards = cards_str.chars().collect::<Vec<_>>();

        let mut counts_map = cards.iter().fold(HashMap::new(), |mut acc, &c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        if let Some(joker) = joker {
            Score::apply_joker(&mut counts_map, card_to_value, joker)?;
        }

        let mut counts = counts_map.values().collect::<Vec<_>>();

        counts.sort();
        counts.reverse();

        let hand_score = match counts.as_slice() {
            [5] => 6,
            [4, 1] => 5,
            [3, 2] => 4,
            [3, 1, 1] => 3,
            [2, 2, 1] => 2,
            [2, 1, 1, 1] => 1,
            _ => 0,
        };

        let card_scores = (
            card_to_value[&cards[0]],
            card_to_value[&cards[1]],
            card_to_value[&cards[2]],
            card_to_value[&cards[3]],
            card_to_value[&cards[4]],
        );

        Ok(Self {
            hand_score,
            card_scores,
        })
    }
}

pub fn get_total_winnings(
    fp: &str,
    card_values: &[char],
    joker: Option<char>,
) -> Result<u64, D7Error> {
    let mut bids = vec![];
    let mut scores = vec![];

    let card_to_value = card_values
        .iter()
        .enumerate()
        .map(|(i, &c)| (c, i as u8))
        .collect::<HashMap<_, _>>();

    for line in get_lines(fp)? {
        let line = line?;
        let (cards_str, bid_str) = line.split_once(" ").ok_or(D7Error::InvalidInput)?;

        bids.push(bid_str.parse::<u64>()?);
        scores.push(Score::new(cards_str, &card_to_value, joker)?);
    }

    let mut scores_w_ind = scores.iter().enumerate().collect::<Vec<_>>();
    scores_w_ind.sort_by(|(_, a), (_, b)| a.cmp(b));

    Ok(scores_w_ind
        .iter()
        .enumerate()
        .map(|(r, (i, _))| (r + 1) as u64 * bids[*i])
        .sum::<u64>())
}
