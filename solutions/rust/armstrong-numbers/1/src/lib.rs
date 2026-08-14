pub fn is_armstrong_number(num: u32) -> bool {
    let num_chars: Vec<u32> = num
        .to_string()
        .chars()
        .map(|e| e.to_digit(10).unwrap_or(0))
        .collect();

    let num_len: u32 = num_chars.len() as u32;
    let armstrong_num: u32 = num_chars.into_iter().fold(0, |acc, e| acc + e.pow(num_len));

    num.eq(&armstrong_num)
}
