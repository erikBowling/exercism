pub fn raindrops(n: u32) -> String {
    let mut output_str = String::new();

    if n.is_multiple_of(3) {
        output_str.push_str("Pling");
    }

    if n.is_multiple_of(5) {
        output_str.push_str("Plang");
    }

    if n.is_multiple_of(7) {
        output_str.push_str("Plong");
    }

    if output_str.is_empty() {
        output_str.push_str(format!("{}", n).as_str());
    }

    output_str
}
