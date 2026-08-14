pub fn build_proverb(list: &[&str]) -> String {
    // Lines in the proverb follow ...{list[n]}...{list[n+1]}...
    // If n + 1 does not exist, print last line

    let mut output_str = String::new();
    let mut first_word = String::new();
    for (i, word_one) in list.iter().enumerate() {
        // Capture first word for final output
        if i == 0 {
            first_word = word_one.to_string();
        }

        // If there's another word after the current one
        match list.get(i + 1) {
            Some(word_two) => {
                output_str.push_str(
                    format!("For want of a {} the {} was lost.\n", word_one, word_two).as_str(),
                );
            }
            None => {
                output_str.push_str(format!("And all for the want of a {}.", first_word).as_str());
            }
        }
    }

    output_str
}
