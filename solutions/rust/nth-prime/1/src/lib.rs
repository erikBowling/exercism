pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut count = 0;
    let mut num = 2;

    loop {
        if count == n {
            break;
        }
        num += 1;
        if check_prime(num) {
            count += 1;
        }
    }

    num
}

pub fn check_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    if n == 2 {
        return true;
    }

    let mut d = 2;
    while d < (n + 1 / 2) {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }

    true
}
