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