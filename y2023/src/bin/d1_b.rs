use y2023::get_lines;

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

fn first_last_digit_combination_sums(fp: &str) -> Result<u32, std::io::Error> {
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

    for line in get_lines(fp)? {
        sum += first_last_digit_combination(line?, &word_to_digit);
    }
    return Ok(sum);
}

fn main() {
    match first_last_digit_combination_sums("data/d1/b.txt") {
        Ok(sum) => println!("{}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
