use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn get_lines(fp: &str) -> Result<Lines<BufReader<File>>, std::io::Error> {
    let file = File::open(fp)?;
    Ok(BufReader::new(file).lines())
}

pub fn get_subquestion_arg() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No input provided, running for a");
        "a".to_string()
    } else {
        args[1].clone()
    }
}

pub fn gcd(a: u64, b: u64) -> u64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub mod util {
    pub mod d10;
    pub mod d12;
    pub mod d13;
    pub mod d14;
    pub mod d15;
    pub mod d16;
    pub mod d18;
    pub mod d19;
    pub mod d21;
    pub mod d22;
    pub mod d23;
    pub mod d24;
    pub mod d4;
    pub mod d5;
    pub mod d7;
    pub mod d8;
}
