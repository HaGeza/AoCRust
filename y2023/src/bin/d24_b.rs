use y2023::util::d24::{get_positions_and_velocities, D24Error};

// Solution based on: https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kxqjg33/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

fn triplet_diff<T>(a: [T; 3], b: [T; 3]) -> [T; 3]
where
    T: std::ops::Sub<Output = T> + Copy,
{
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn triplet_sum<T>(a: [T; 3], b: [T; 3]) -> [T; 3]
where
    T: std::ops::Add<Output = T> + Copy,
{
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn triplet_div<T>(v: [T; 3], s: T) -> [T; 3]
where
    T: std::ops::Div<Output = T> + Copy,
{
    [v[0] / s, v[1] / s, v[2] / s]
}

fn cross_prod<T>(a: [T; 3], b: [T; 3]) -> [T; 3]
where
    T: std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot_prod<T>(a: [T; 3], b: [T; 3]) -> T
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
{
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn solve(fp: &str) -> Result<i64, D24Error> {
    let (starts, velocities) = get_positions_and_velocities(fp)?;
    if starts.len() < 3 {
        return Err(D24Error::Part2Unsolvable);
    }

    // Step 1: Look at all positions relative to stone 0 => the points of collision should still be collinear
    let p1 = triplet_diff(starts[1], starts[0]).map(|s| s as i128);
    let p2 = triplet_diff(starts[2], starts[0]).map(|s| s as i128);
    // Step 2: Look at points of collision relative to the CURRENT position of stone 0. These should still be collinear,
    // because the stones are moving at constant velocity. This trick will allow us to not deal with t0, but only t1 and t2.
    let v1 = triplet_diff(velocities[1], velocities[0]).map(|s| s as i128);
    let v2 = triplet_diff(velocities[2], velocities[0]).map(|s| s as i128);
    // Step 3: Calculate the collision times for stone 1 and 2. To derive the formulas:
    // (1) As the relative collision positions need to be collinear, their cross product needs to be zero (the magnitude
    //     of the cross product is the area of the parallelogram spanned by the two vectors)
    //     => cross_prod(p1 + t1 * v1, p2 + t2 * v2) = 0
    // (2) Expand the cross product: p1 x p2 + t * (v1 x p2) + t2 * (p1 x v2) + t1 * t2 * (v1 x v2) = 0
    // (3) The result of a x b is orthogonal to a and b, and therefore its dot product withe either a or b is 0
    //     => (a x b) . a = (a x b) . b = 0
    // (4) Apply dot product with v1, eliminating 2 of the 4 terms, and allowing us to express t2. Do the
    //     same with v2 to express t1.
    let t1 = -dot_prod(cross_prod(p1, p2), v2) / dot_prod(cross_prod(v1, p2), v2);
    let t2 = -dot_prod(cross_prod(p1, p2), v1) / dot_prod(cross_prod(p1, v2), v1);

    let pos1 = starts[1].map(|c| c as f64);
    let pos2 = starts[2].map(|c| c as f64);
    let vel1 = velocities[1].map(|c| c as f64);
    let vel2 = velocities[2].map(|c| c as f64);

    // Step 4: Get two equations for the throw: q + w * t1 = ... and q + w * t2 = ..., and express q
    let throw_start = triplet_div(
        triplet_sum(
            triplet_diff(triplet_div(pos1, t1 as f64), triplet_div(pos2, t2 as f64)),
            triplet_diff(vel1, vel2),
        ),
        1.0 / t1 as f64 - 1.0 / t2 as f64,
    );

    // Print the throw start for debugging (rounding will be needed)
    println!("{:?}", throw_start);

    // Return the sum of the rounded coordinates
    Ok(throw_start.map(|c| c.round() as i64).iter().sum())
}

fn main() {
    match solve("data/d24/a.txt") {
        Ok(sol) => println!("{}", sol),
        Err(e) => eprintln!("Error: {}", e),
    }
}
