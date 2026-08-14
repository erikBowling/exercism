/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // If input has any invalid characters
    if code.chars().any(|c| !(c.is_ascii_digit() || c == ' ')) {
        return false;
    }

    // If input is empty or only 1 character
    if code.trim().len() <= 1 {
        return false;
    }

    // Accumulate valid digits as u32
    let mut nums: Vec<u32> = code.chars().filter_map(|c| c.to_digit(10)).collect();

    // reverse and double every other number
    for (i, n) in nums.iter_mut().rev().enumerate() {
        if i % 2 == 1 {
            *n *= 2;
            if *n >= 10 {
                *n -= 9;
            }
        }
    }

    let luhn_sum: u32 = nums.iter().sum();

    luhn_sum.is_multiple_of(10)
}
