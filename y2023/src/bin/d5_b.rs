use y2023::{get_lines, util::d5::D5Error};

fn min_location(fp: &str) -> Result<u64, D5Error> {
    let mut lines = get_lines(fp)?;

    let Some(Ok(seeds_line)) = lines.next() else {
        return Err(D5Error::NoSeedsLine);
    };
    let Some((_, seeds_part)) = seeds_line.split_once(":") else {
        return Err(D5Error::InvalidSeedsLine);
    };

    let seeds = seeds_part
        .split_whitespace()
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()?;

    let mut unmapped: Vec<(u64, u64)> = seeds
        .iter()
        .step_by(2)
        .zip(seeds.iter().skip(1).step_by(2))
        .map(|(&a, &b)| (a, a + b))
        .collect();
    let mut mapped = vec![];

    for line in lines {
        let line = line?;

        if line.is_empty() {
            continue;
        } else if line.contains("map:") {
            unmapped.extend(mapped.clone());
            mapped.clear();
        } else {
            let nums = line
                .split_whitespace()
                .map(|x| x.parse::<u64>())
                .collect::<Result<Vec<u64>, _>>()?;
            let (to_start, from_start, range) = (nums[0], nums[1], nums[2]);
            let (to_end, from_end) = (to_start + range, from_start + range);

            let initial_len = unmapped.len();
            for j in 0..initial_len {
                let (seed_start, seed_end) = unmapped[j];
                if seed_start >= seed_end {
                    continue;
                }

                if seed_start <= from_start {
                    if seed_end >= from_end {
                        // seed contains from
                        mapped.push((to_start, to_end));
                        unmapped[j] = (seed_start, from_start);
                        unmapped.push((from_end, seed_end));
                    } else if seed_end > from_start {
                        // right side of seed overlaps from
                        let end = to_start + seed_end - from_start;
                        mapped.push((to_start, end));
                        unmapped[j] = (seed_start, from_start);
                    }
                } else if seed_start < from_end {
                    if seed_end >= from_end {
                        // left side of seed overlaps from
                        let start = to_start + seed_start - from_start;
                        mapped.push((start, to_end));
                        unmapped[j] = (from_end, seed_end);
                    } else {
                        // from contains seed
                        let start = to_start + seed_start - from_start;
                        let end = to_start + seed_end - from_start;
                        mapped.push((start, end));
                        unmapped[j] = (from_start, from_start);
                    }
                }
            }
            unmapped = unmapped.iter().filter(|(a, b)| a < b).map(|&x| x).collect();
        }
    }

    unmapped.extend(mapped.clone());
    mapped.clear();

    let Some(min_loc) = unmapped.iter().map(|(a, _)| a).min() else {
        return Err(D5Error::InvalidSeedsLine);
    };
    Ok(*min_loc)
}

fn main() {
    match min_location("data/d5/a.txt") {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {}", e),
    }
}
