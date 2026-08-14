pub fn is_leap_year(year: u64) -> bool {
    // If not divisible by 4
    // Standard case
    if !year.is_multiple_of(4) {
        return false;
    }

    // Accounts for centuries
    if year.is_multiple_of(100) && !year.is_multiple_of(400) {
        return false;
    }

    true
}
