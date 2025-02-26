pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let left_paren = num_str.find('(').expect("cannot find ( in num_str");
    let right_paren = num_str.find(')').expect("cannot find ) in num_str");
    let nums = num_str[..left_paren]
        .to_lowercase()
        .chars()
        .collect::<Vec<_>>();
    let base = num_str[left_paren + 1..right_paren]
        .parse::<u32>()
        .expect("base must be num");

    let num_d = convert_to_d(&nums, base);
    d_convert_to_base(num_d, to_base)
}

const NUMS: &str = "0123456789abcdefghijklmnopqrstuvwxyz";

fn convert_to_d(num_str: &[char], base: u32) -> i64 {
    let mut res = i64::default();
    num_str.iter().for_each(|c| {
        let v = NUMS.find(*c).expect("unknown char");
        res = res * base as i64 + v as i64;
    });

    res
}

fn d_convert_to_base(num_d: i64, to_base: u32) -> String {
    if num_d == 0 {
        return "0".to_string();
    }

    let mut res = String::new();
    fn convert(num: i64, to_base: u32, res: &mut String) {
        if num >= to_base as i64 {
            convert(num / to_base as i64, to_base, res);
        }
        res.push(
            NUMS.chars()
                .nth((num % to_base as i64) as usize)
                .expect("cannot conver char to num"),
        );
    }
    convert(num_d, to_base, &mut res);
    res
}
