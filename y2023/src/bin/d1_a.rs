use std::io::BufRead;

use y2023::get_reader;

fn first_last_digit_combination(line: String) -> u32 {
    let mut first: u8 = 0;
    let mut last: u8 = 0;
    let mut first_index: usize = 0;

    let bytes = line.as_bytes();

    for b in bytes {
        if b.is_ascii_digit() {
            first = b - '0' as u8;
            break;
        }
        first_index += 1;
    }

    for b in bytes.iter().skip(first_index).rev() {
        if b.is_ascii_digit() {
            last = b - '0' as u8;
            break;
        }
    }

    return (first * 10 + last) as u32;
}

fn first_last_digit_combination_sums() -> Result<u32, std::io::Error> {
    let reader = get_reader("data/d1/a.txt")?;
    let mut sum = 0;

    for line in reader.lines() {
        sum += first_last_digit_combination(line?);
    }
    return Ok(sum);
}

fn main() {
    match first_last_digit_combination_sums() {
        Ok(sum) => println!("{}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
