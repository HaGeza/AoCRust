use std::collections::HashMap;

use y2023::{
    get_lines,
    util::d19::{parse_instruction, D19Error, Instruction, Operator},
};

fn get_num_combinations(
    workflows: &HashMap<String, Vec<(Instruction, String)>>,
    wkey: &str,
    instruction_ind: usize,
    ranges: &HashMap<String, (u32, u32)>,
) -> Result<u64, D19Error> {
    match wkey {
        "R" => return Ok(0),
        "A" => {
            return Ok(ranges
                .iter()
                .map(|(_, (l, h))| (h - l + 1) as u64)
                .product())
        }
        _ => (),
    }

    let instruction = &workflows[wkey][instruction_ind];

    match instruction {
        (Instruction::Conditional(key, op, val), dest) => {
            let (l, h) = ranges[key];
            let (cw_range, ci_range) = match op {
                Operator::LT => ((l, *val - 1), (*val, h)),
                Operator::GT => ((*val + 1, h), (l, *val)),
            };

            let mut cw_ranges = ranges.clone();
            cw_ranges.insert(key.clone(), cw_range);
            let mut ci_ranges = ranges.clone();
            ci_ranges.insert(key.clone(), ci_range);

            Ok(get_num_combinations(workflows, &dest, 0, &cw_ranges)?
                + get_num_combinations(workflows, wkey, instruction_ind + 1, &ci_ranges)?)
        }
        (Instruction::Unconditional, dest) => get_num_combinations(workflows, dest, 0, ranges),
    }
}

fn solve(fp: &str) -> Result<u64, D19Error> {
    let mut workflows = HashMap::new();

    for line in get_lines(fp)? {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let (key, instructions) = parse_instruction(line)?;
        workflows.insert(key, instructions);
    }

    let mut ranges = HashMap::new();
    for s in ["x", "m", "a", "s"] {
        ranges.insert(s.to_string(), (1, 4000));
    }
    Ok(get_num_combinations(&workflows, "in", 0, &ranges)?)
}

fn main() {
    match solve("data/d19/a.txt") {
        Ok(sol) => println!("{}", sol),
        Err(err) => eprintln!("Error: {}", err),
    }
}
