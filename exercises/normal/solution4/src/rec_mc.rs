pub fn dp_rec_mc(mut amount: u32) -> u32 {
    let mut res = 0;

    let ty = [100, 50, 30, 20, 10, 5, 2, 1];
    ty.iter().for_each(|t| {
        res += amount / t;
        amount %= t;
    });

    res
}
