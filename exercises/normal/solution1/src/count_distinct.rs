pub fn new_count_distinct(input_str: &str) -> usize {
    let v = input_str.to_string();
    let mut v = v.split(",").collect::<Vec<_>>();
    v.sort();
    v.dedup();
    v.len()
}
