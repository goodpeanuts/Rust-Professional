pub fn new_birthday_probability(n: u32) -> f64 {
    let mut p = 1.0;
    for i in 0..n {
        p *= (365.0 - i as f64) / 365.0;
    }
    1.0 - p
}
