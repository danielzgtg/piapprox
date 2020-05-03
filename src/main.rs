fn main() {
    let step: u64 = 1_000_000;
    let denom = step * step;
    let mut result: u64 = 0;
    let mut x_sq = 0;
    for x in 0..step {
        let mut dist_sq = x_sq;
        for y in 0..=x {
            if dist_sq >= denom {
                break;
            }
            result += 1;
            dist_sq += (y << 1) + 1;
        }
        x_sq += (x << 1) + 1;
    }
    let pi = result as f64 / denom as f64 * 8.0;
    println!("{}", pi);
}
