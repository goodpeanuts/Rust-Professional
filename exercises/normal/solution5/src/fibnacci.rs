pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut res = 0;

    let (mut x, mut y) = (0, 1);
    while y < threshold {
        (x, y) = (y, x + y);
        if x % 2 == 1 {
            res += x;
        }
    }
    res
}
