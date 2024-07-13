use std::fs::File;
use std::io::{BufRead, BufReader};

fn first_last_digit_combination(line: String, word_to_digit: &[(&str, u8)]) -> u32 {
    let mut first: u8 = 0;
    let mut last: u8 = 0;
    let mut first_index: usize = 0;

    let bytes = line.as_bytes();

    'outer: for (i, b) in bytes.iter().enumerate() {
        if b.is_ascii_digit() {
            first = b - '0' as u8;
            break 'outer;
        } else {
            for (word, digit) in word_to_digit.iter() {
                let len = word.len();
                if i + 1 >= len && &line[i + 1 - len..i + 1] == *word {
                    first = *digit;
                    break 'outer;
                }
            }
        }
        first_index += 1;
    }

    let b_len = bytes.len();
    'outer: for (i, b) in bytes.iter().enumerate().skip(first_index).rev() {
        let _c = *b as char;
        if b.is_ascii_digit() {
            last = b - '0' as u8;
            break 'outer;
        } else {
            for (word, digit) in word_to_digit.iter() {
                let len = word.len();
                if b_len - i >= len && &line[i..i + len] == *word {
                    last = *digit;
                    break 'outer;
                }
            }
        }
    }

    return (first * 10 + last) as u32;
}

fn first_last_digit_combination_sums() -> Result<u32, std::io::Error> {
    // Load data file
    let fp = "data/d1/b.txt";
    let file = File::open(fp)?;
    let reader = BufReader::new(file);

    let word_to_digit = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut sum = 0;

    for line in reader.lines() {
        sum += first_last_digit_combination(line?, &word_to_digit);
    }
    return Ok(sum);
}

fn main() {
    match first_last_digit_combination_sums() {
        Ok(sum) => println!("{}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
