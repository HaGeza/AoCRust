use y2023::util::d24::{get_positions_and_velocities, D24Error};

fn get_2d_line_equation(start: [i64; 3], velocity: [i64; 3]) -> (f64, f64) {
    let m = velocity[1] as f64 / velocity[0] as f64;
    let c = start[1] as f64 - m * start[0] as f64;
    (m, c)
}

fn get_2d_delta(start: [i64; 3], velocity: [i64; 3], point: (f64, f64)) -> (f64, f64) {
    (
        (point.0 - start[0] as f64) / velocity[0] as f64,
        (point.1 - start[1] as f64) / velocity[1] as f64,
    )
}

fn solve(fp: &str, min_bound: f64, max_bound: f64) -> Result<u32, D24Error> {
    let (starts, velocities) = get_positions_and_velocities(fp)?;

    let mut intersections = 0;
    for i in 0..starts.len() - 1 {
        for j in i + 1..starts.len() {
            let (a, c) = get_2d_line_equation(starts[i], velocities[i]);
            let (b, d) = get_2d_line_equation(starts[j], velocities[j]);
            let (x, y) = if a == b {
                continue;
            } else {
                let x = (d - c) / (a - b);
                let y = a * x + c;
                (x, y)
            };

            if x < min_bound || x > max_bound || y < min_bound || y > max_bound {
                continue;
            }

            let delta_1 = get_2d_delta(starts[i], velocities[i], (x, y));
            let delta_2 = get_2d_delta(starts[j], velocities[j], (x, y));

            if delta_1.0 < 0.0 || delta_2.0 < 0.0 || delta_1.1 < 0.0 || delta_2.1 < 0.0 {
                continue;
            }
            intersections += 1;
        }
    }

    Ok(intersections)
}

fn main() {
    let (min_bound, max_bound) = (200000000000000.0, 400000000000000.0);
    // let (min_bound, max_bound) = (7.0, 27.0);

    match solve("data/d24/a.txt", min_bound, max_bound) {
        Ok(sol) => println!("{}", sol),
        Err(e) => eprintln!("Error: {}", e),
    }
}
