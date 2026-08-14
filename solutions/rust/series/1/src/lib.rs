pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for w in digits.as_bytes().windows(len) {
        if let Ok(s) = String::from_utf8(w.into()) {
            result.push(s);
        }
    }

    result
}
