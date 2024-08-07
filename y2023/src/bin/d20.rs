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
use y2023::{get_lines, get_subquestion_arg, lcm};

#[derive(Debug, Error)]
enum D20Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("invalid line")]
    InvalidLine,
    #[error("flipping non flip-flop")]
    FlippingNonFlipFlop,
    #[error("Part 2 error: {0}")]
    Part2Error(String),
}

#[derive(PartialEq, Eq)]
enum Switch {
    FlipFlop(bool),
    Conjunction,
    Broadcaster,
}

// Solution to part 2 only works if the input of rx is a conjunction, and I am not sure why it works even in that case.
// Multi-input flip-flop switches should behave erratically, messing up subsequent periodicity, but this does not seem to
// be the case for some reason...
fn solve(fp: &str, use_rx: bool) -> Result<u64, D20Error> {
    let mut sequences = HashMap::new();
    let mut conjunction_maps = HashMap::new();

    // Get sequences
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

    // Create conjunction maps and find rx input for part 2 (it must be a conjunction)
    let mut final_conj = None;
    for (origin, (sw, targets)) in &sequences {
        for target in targets {
            if let Some(hm) = conjunction_maps.get_mut(target) {
                hm.insert(origin.to_string(), false);
            }
            if target == "rx" && use_rx {
                if final_conj.is_some() {
                    return Err(D20Error::Part2Error("'rx' has multiple inputs".to_string()));
                }

                if *sw == Switch::Conjunction {
                    final_conj = Some(origin);
                } else {
                    return Err(D20Error::Part2Error(
                        "'rx' input is not conjunction".to_string(),
                    ));
                }
            }
        }
    }

    // Find final conjunction inputs for part 2
    let mut final_conj_inputs = HashMap::new();
    if use_rx {
        let Some(rx_input) = final_conj else {
            return Err(D20Error::Part2Error("'rx' has no inputs".to_string()));
        };
        for (origin, (_sw, targets)) in &sequences {
            for target in targets {
                if target == rx_input {
                    final_conj_inputs.insert(origin.to_string(), None);
                }
            }
        }
    }

    // Simulate
    let (mut low, mut high) = (0, 0);
    let mut ind = 0;

    // Part 1: simulate 1000 steps
    // Part 2: simulate until all conjunction inputs are found
    while (!use_rx && ind < 1000) || (use_rx && final_conj_inputs.values().any(|v| v.is_none())) {
        ind += 1;
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
                    if use_rx && next_pulse && final_conj_inputs.contains_key(&origin.to_string()) {
                        let period = final_conj_inputs.get_mut(&origin.to_string()).unwrap();
                        if period.is_none() {
                            *period = Some(ind);
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

    if !use_rx {
        Ok(low * high)
    } else {
        Ok(final_conj_inputs
            .values()
            .fold(1, |acc, &x| lcm(acc, x.unwrap())))
    }
}

fn main() {
    let use_rx = match get_subquestion_arg().as_str() {
        "a" => false,
        "b" => true,
        _ => panic!("Invalid subquestion"),
    };

    match solve("data/d20/a.txt", use_rx) {
        Ok(sol) => println!("{}", sol),
        Err(e) => println!("Error: {:?}", e),
    }
}
