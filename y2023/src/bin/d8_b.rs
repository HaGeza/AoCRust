use y2023::util::d8::{read_environment, steps_from_a_to_b, D8Error};

fn gcd(a: u64, b: u64) -> u64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn solve(fp: &str) -> Result<u64, D8Error> {
    let (moves, nodes) = read_environment(fp)?;

    let starts: Vec<String> = nodes.keys().filter(|k| k.ends_with("A")).cloned().collect();
    let ends: Vec<String> = nodes.keys().filter(|k| k.ends_with("Z")).cloned().collect();

    let steps = starts
        .iter()
        .map(|a| steps_from_a_to_b(&moves, &nodes, a.clone(), &ends))
        .collect::<Result<Vec<u64>, D8Error>>()?;

    let lcm_result = steps.iter().fold(1, |acc, &x| lcm(acc, x));

    Ok(lcm_result)
}

fn main() {
    match solve("data/d8/a.txt") {
        Ok(steps) => println!("{}", steps),
        Err(e) => eprintln!("Error: {}", e),
    }
}
