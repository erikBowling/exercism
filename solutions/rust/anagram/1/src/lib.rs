use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut results: HashSet<&'a str> = HashSet::new();
    let lower_word = word.to_lowercase();
    possible_anagrams
        .iter()
        .filter(|&w| {
            let l_case_w = w.to_lowercase();
            l_case_w
                .chars()
                .all(|c| get_char_count(&l_case_w, &c) == get_char_count(&lower_word, &c))
                && l_case_w.len() == lower_word.len()
                && *l_case_w != lower_word
        })
        .for_each(|w| _ = results.insert(w));

    results
}

pub fn get_char_count(word: &str, c: &char) -> usize {
    word.chars().filter(|temp_c| temp_c == c).count()
}
