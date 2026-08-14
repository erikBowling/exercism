pub fn collatz(n: u64) -> Option<u64> {
    // Base Cases n = 0, 1
    if n == 0 {
        return None;
    }

    if n == 1 {
        return Some(0);
    }

    let mut count: u64 = 0;
    let mut num = n;

    loop {
        if num.is_multiple_of(2) {
            num /= 2;
        } else {
            num = num * 3 + 1;
        }

        count += 1;

        if num == 1 {
            break;
        }
    }

    Some(count)
}
