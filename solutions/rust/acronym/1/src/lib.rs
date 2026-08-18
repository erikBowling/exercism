pub fn abbreviate(phrase: &str) -> String {
    // Individual words are captured e.g. As Soon As Possible => ASAP
    // Dashes are considered a delimeter e.g. Liquid-crystal display => LCD
    // - Clean string. Trim, remove unnecessary punctuation, swap dashes for spaces etc.
    // - Split at whitespace
    // - Collect
    // - Check each word for CamelCasing and split if found
    let cleaned_string: String = phrase
        .chars()
        .map(|c| match c {
            '-' => ' ',
            _ => c,
        })
        .filter(|c| !c.is_ascii_punctuation())
        .collect();

    let raw_individual_words: Vec<&str> = cleaned_string.split_whitespace().collect();
    let mut final_words: Vec<String> = Vec::new();

    // If word has more than 1 capital letter and isn't entirely capital letters
    for word in raw_individual_words {
        if !word.chars().all(|c| c.is_uppercase())
            && word.chars().filter(|c| c.is_uppercase()).count() > 1
        {
            split_at_capital_letters(word)
                .iter()
                .for_each(|w| final_words.push(w.to_string()));

            continue;
        }

        final_words.push(word.to_string());
    }

    let acronym: String = final_words
        .iter()
        .map(|word| word.chars().next().unwrap())
        .collect();

    acronym.to_uppercase()
}

pub fn split_at_capital_letters(s: &str) -> Vec<&str> {
    let mut indices = vec![0];

    // Find the starting byte position of every uppercase character
    for (idx, c) in s.char_indices() {
        if c.is_uppercase() && idx > 0 {
            indices.push(idx);
        }
    }
    indices.push(s.len());

    // Slice the string at the gathered boundaries
    indices.windows(2).map(|w| &s[w[0]..w[1]]).collect()
}
