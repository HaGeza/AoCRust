use std::collections::HashMap;

use y2023::{
    get_lines,
    util::d19::{parse_instruction, D19Error, Instruction, Operator},
};

fn run_workflow(
    workflow: &HashMap<String, Vec<(Instruction, String)>>,
    line: String,
) -> Result<u64, D19Error> {
    let mut wkey = "in";
    let mut instruction_ind = 0;
    let parts = line
        .trim_matches(&['{', '}'])
        .split(',')
        .map(|s| {
            let Some((key, val)) = s.split_once('=') else {
                return Err(D19Error::InvalidInput);
            };
            Ok((key.to_string(), val.parse::<u32>()?))
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    loop {
        match wkey {
            "R" => return Ok(0),
            "A" => return Ok(parts.values().sum::<u32>() as u64),
            _ => (),
        }

        let instruction = &workflow[wkey][instruction_ind];

        match instruction {
            (Instruction::Conditional(key, op, val), dest) => {
                let act_val = parts.get(key).unwrap();
                let cond_true = match op {
                    Operator::LT => act_val < val,
                    Operator::GT => act_val > val,
                };

                if cond_true {
                    wkey = dest;
                    instruction_ind = 0;
                } else {
                    instruction_ind += 1;
                }
            }
            (Instruction::Unconditional, dest) => {
                wkey = dest;
                instruction_ind = 0;
            }
        }
    }
}

fn solve(fp: &str) -> Result<u64, D19Error> {
    let mut sum = 0;

    let mut workflows = HashMap::new();
    let mut instructions_added = false;

    for line in get_lines(fp)? {
        let line = line?;
        if line.is_empty() {
            instructions_added = true;
            continue;
        }

        if !instructions_added {
            let (key, instructions) = parse_instruction(line)?;
            workflows.insert(key, instructions);
        } else {
            sum += run_workflow(&workflows, line)?;
        }
    }

    Ok(sum)
}

fn main() {
    match solve("data/d19/a.txt") {
        Ok(sol) => println!("{}", sol),
        Err(err) => eprintln!("Error: {}", err),
    }
}
