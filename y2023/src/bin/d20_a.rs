// low or high pulse
// flip-flop: % + high    => ignore
//            %-on + low  => %-off + low
//            %-off + low => %-on + high
// conjunction: & + all high  => low
//              & + otherwise => high
// broadcast: broadcaster => same as input
// button: when pushed => low to broadcaster

use std::collections::{HashMap, HashSet};

use thiserror::Error;
use y2023::get_lines;

#[derive(Debug, Error)]
enum D20Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("invalid line")]
    InvalidLine,
    #[error("flipping non flip-flop")]
    FlippingNonFlipFlop,
}

enum Switch {
    FlipFlop(bool),
    Conjunction,
    Broadcaster,
}

// fn get_state(sequences: &HashMap<String, (Switch, Vec<String>)>) -> Vec<bool> {
//     sequences
//         .iter()
//         .filter_map(|(_, (sw, _))| match sw {
//             Switch::FlipFlop(on_off) => Some(*on_off),
//             _ => None,
//         })
//         .collect()
// }

fn solve(fp: &str) -> Result<u64, D20Error> {
    let mut sequences = HashMap::new();
    let mut conjunction_maps = HashMap::new();

    for line in get_lines(fp)? {
        let line = line?;
        let Some((origin, targets)) = line.split_once(" -> ") else {
            return Err(D20Error::InvalidLine);
        };

        let targets = targets
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let mut origin_string = origin.to_string();
        let switch = if origin == "broadcaster" {
            Switch::Broadcaster
        } else {
            origin_string = origin[1..].to_string();
            match origin.chars().nth(0) {
                Some('%') => Switch::FlipFlop(false),
                Some('&') => {
                    conjunction_maps.insert(origin_string.clone(), HashMap::new());
                    Switch::Conjunction
                }
                _ => return Err(D20Error::InvalidLine),
            }
        };

        sequences.insert(origin_string, (switch, targets));
    }

    for (origin, (_sw, targets)) in &sequences {
        for target in targets {
            if let Some(hm) = conjunction_maps.get_mut(target) {
                hm.insert(origin.to_string(), false);
            }
        }
    }

    // let mut states = vec![(get_state(&sequences), 0, 0)];
    let (mut low, mut high) = (0, 0);

    for _ in 0..1000 {
        let mut step = vec![("broadcaster".to_string(), false)];
        while !step.is_empty() {
            let mut next_step = vec![];
            let mut flips = HashSet::new();

            for (origin, pulse) in step {
                if pulse {
                    high += 1;
                } else {
                    low += 1;
                }

                let Some((sw, targets)) = &sequences.get(&origin) else {
                    continue;
                };
                let next_pulse = match sw {
                    Switch::Broadcaster => Some(pulse),
                    Switch::Conjunction => {
                        let hm = conjunction_maps.get(&origin).unwrap();
                        let all_high = hm.values().all(|&v| v);
                        Some(!all_high)
                    }
                    Switch::FlipFlop(on_off) => {
                        if !pulse {
                            flips.insert(origin.clone());
                            Some(!*on_off)
                        } else {
                            None
                        }
                    }
                };
                if let Some(next_pulse) = next_pulse {
                    for target in targets {
                        next_step.push((target.to_string(), next_pulse));

                        if let Some(hm) = conjunction_maps.get_mut(target) {
                            hm.insert(origin.to_string(), next_pulse);
                        }
                    }
                }
            }

            for origin in flips {
                let flip_flop = sequences.get_mut(&origin).unwrap();
                let (Switch::FlipFlop(on_off), _) = flip_flop else {
                    return Err(D20Error::FlippingNonFlipFlop);
                };
                flip_flop.0 = Switch::FlipFlop(!*on_off);
            }

            step = next_step;
        }
    }

    println!("low: {}, high: {}", low, high);
    Ok(low * high)
}

fn main() {
    match solve("data/d20/test_2.txt") {
        Ok(sol) => println!("{}", sol),
        Err(e) => println!("Error: {:?}", e),
    }
}
