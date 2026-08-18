#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base == 0 || from_base == 1 {
        return Err(Error::InvalidInputBase);
    }

    if to_base == 0 || to_base == 1 {
        return Err(Error::InvalidOutputBase);
    }

    for num in number {
        if num >= &from_base {
            return Err(Error::InvalidDigit(*num));
        }
    }

    // from_base => base_10 => to_base
    let mut base_10_number: u32 = number
        .iter()
        .rev()
        .enumerate() // reverse then enumerate. Order matters here.
        .fold(0, |acc, (i, n)| acc + (n * from_base.pow(i as u32)));

    if base_10_number == 0 {
        return Ok(vec![0]);
    }

    let mut result: Vec<u32> = Vec::new();

    // https://math.stackexchange.com/questions/111150/changing-a-number-between-arbitrary-bases
    while base_10_number > 1 {
        result.push(base_10_number % to_base);
        base_10_number /= to_base;
    }

    if base_10_number == 1 {
        result.push(base_10_number);
    }

    result.reverse();

    Ok(result)
}
