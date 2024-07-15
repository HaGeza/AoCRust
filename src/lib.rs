use std::{fs::File, io::BufReader};

pub fn get_reader(fp: &str) -> Result<BufReader<File>, std::io::Error> {
    let file = File::open(fp)?;
    Ok(BufReader::new(file))
}
