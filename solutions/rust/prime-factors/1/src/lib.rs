pub fn factors(n: u64) -> Vec<u64> {
    let mut num: u64 = n;
    let mut prime_factors: Vec<u64> = Vec::new();
    let mut divisor: u64 = 2;
    loop {
        if num.is_multiple_of(divisor) {
            num /= divisor;
            prime_factors.push(divisor);
            continue;
        }

        if num == 1 {
            break;
        }

        divisor += 1;
    }
    prime_factors
}
