use y2023::get_lines;

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

fn first_last_digit_combination_sums(fp: &str) -> Result<u32, std::io::Error> {
    let mut sum = 0;

    for line in get_lines(fp)? {
        sum += first_last_digit_combination(line?);
    }
    return Ok(sum);
}

fn main() {
    match first_last_digit_combination_sums("data/d1/a.txt") {
        Ok(sum) => println!("{}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
