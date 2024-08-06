use y2023::{
    get_lines,
    util::d18::{get_movement, parse_line, D18Error, Dir},
};

fn solve(fp: &str, use_colors: bool) -> Result<i64, D18Error> {
    let mut steps = vec![];
    let mut corners = vec![];
    let mut last_dir = Dir::U;

    for line in get_lines(fp)? {
        let (mut dir, mut dist, col) = parse_line(&line?)?;

        if use_colors {
            let cl = col.len();
            dist = i64::from_str_radix(&col[2..cl - 2], 16)?;
            dir = match col.chars().nth(cl - 2) {
                Some('0') => Dir::R,
                Some('1') => Dir::D,
                Some('2') => Dir::L,
                Some('3') => Dir::U,
                _ => return Err(D18Error::InvalidInput),
            };
        }

        corners.push((last_dir, dir));
        last_dir = dir;

        steps.push((dir, dist));
    }
    corners.push(corners[0]);

    let (mut x, mut y) = (0, 0);
    let mut sum: i64 = 0;

    for (i, (dir, dist)) in steps.iter().enumerate() {
        let actual_dist = match (corners[i], corners[i + 1]) {
            ((Dir::U, Dir::R), (Dir::R, Dir::D))
            | ((Dir::R, Dir::D), (Dir::D, Dir::L))
            | ((Dir::D, Dir::L), (Dir::L, Dir::U))
            | ((Dir::L, Dir::U), (Dir::U, Dir::R)) => dist + 1,
            ((Dir::U, Dir::L), (Dir::L, Dir::D))
            | ((Dir::L, Dir::D), (Dir::D, Dir::R))
            | ((Dir::D, Dir::R), (Dir::R, Dir::U))
            | ((Dir::R, Dir::U), (Dir::U, Dir::L)) => dist - 1,
            _ => *dist,
        };
        let (dy, dx) = get_movement(*dir, actual_dist)?;
        let (new_x, new_y) = (x + dx, y + dy);
        let area = (x * new_y - new_x * y) as i64;
        sum += area;
        x = new_x;
        y = new_y;
    }

    Ok(sum.abs() / 2)
}

fn main() {
    match solve("data/d18/a.txt", true) {
        Ok(sol) => println!("{}", sol),
        Err(err) => println!("Error: {}", err),
    }
}
