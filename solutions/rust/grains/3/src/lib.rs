// 1st square => 2^0 = 1
// 2nd square => 2^1 = 2
// 3rd square => 2^2 = 4
// ...and so on

pub fn square(s: u32) -> u64 {
    if s > 64 {
        panic!("Number too large")
    }

    let result: u64 = 2;
    result.pow(s - 1)
}

pub fn total() -> u64 {
    (1..65).map(square).sum()
}
