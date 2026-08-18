pub fn egg_count(display_value: u32) -> usize {
    let bin_value = format!("{:b}", display_value);
    bin_value.chars().filter(|c| *c == '1').count()
}
