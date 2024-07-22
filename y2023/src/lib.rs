use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn get_lines(fp: &str) -> Result<Lines<BufReader<File>>, std::io::Error> {
    let file = File::open(fp)?;
    Ok(BufReader::new(file).lines())
}

pub mod util {
    pub mod d4;
    pub mod d5;
    pub mod d7;
    pub mod d8;
}
