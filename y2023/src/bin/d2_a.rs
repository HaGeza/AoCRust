use y2023::get_lines;

fn id_if_valid_else_zero(line: String) -> u32 {
    let Some((id_part, rest)) = line.split_once(":") else {
        println!("Invalid line: {}", line);
        return 0;
    };

    let id = match id_part.split_once(" ") {
        Some(("Game", id_str)) => id_str.parse::<u32>().unwrap(),
        _ => {
            println!("Invalid line: {}", line);
            return 0;
        }
    };

    let sets = rest
        .split(";")
        .map(|s| s.trim().split(",").map(|c| c.trim()));

    for set in sets {
        let (red, green, blue) = set
            .map(|c| c.split_once(" ").unwrap())
            .map(|(id, color)| (id.parse::<u32>().unwrap(), color))
            .fold((0, 0, 0), |(r, g, b), (id, color)| match color {
                "red" => (r + id, g, b),
                "green" => (r, g + id, b),
                "blue" => (r, g, b + id),
                _ => (r, g, b),
            });

        if red > 12 || green > 13 || blue > 14 {
            return 0;
        }
    }

    println!("Valid line: {}", id);
    return id;
}

fn sum_of_valid_ids(fp: &str) -> Result<u32, std::io::Error> {
    let mut sum = 0;

    for line in get_lines(fp)? {
        sum += id_if_valid_else_zero(line?);
    }
    Ok(sum)
}

fn main() {
    match sum_of_valid_ids("data/d2/a.txt") {
        Ok(sum) => println!("Sum of valid IDs: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
