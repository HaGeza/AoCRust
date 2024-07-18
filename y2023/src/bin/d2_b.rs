use y2023::get_lines;

fn bag_power(line: String) -> u32 {
    let Some((_, rest)) = line.split_once(":") else {
        println!("Invalid line: {}", line);
        return 0;
    };

    let sets = rest
        .split(";")
        .map(|s| s.trim().split(",").map(|c| c.trim()));

    let (mut max_r, mut max_g, mut max_b) = (0, 0, 0);
    for set in sets {
        let (red, green, blue) = set
            .map(|c| c.split_once(" ").unwrap())
            .map(|(cnt, color)| (cnt.parse::<u32>().unwrap(), color))
            .fold((0, 0, 0), |(r, g, b), (cnt, color)| match color {
                "red" => (r + cnt, g, b),
                "green" => (r, g + cnt, b),
                "blue" => (r, g, b + cnt),
                _ => (r, g, b),
            });

        max_r = max_r.max(red);
        max_g = max_g.max(green);
        max_b = max_b.max(blue);
    }

    max_r * max_b * max_g
}

fn bag_power_sum(fp: &str) -> Result<u32, std::io::Error> {
    let mut sum = 0;

    for line in get_lines(fp)? {
        sum += bag_power(line?);
    }
    Ok(sum)
}

fn main() {
    match bag_power_sum("data/d2/b.txt") {
        Ok(sum) => println!("Sum of bag powers: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
