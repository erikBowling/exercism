use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (raw_addends, raw_result) = input.split_once(" == ").unwrap();
    let addends: Vec<&str> = raw_addends
        .split("+")
        .into_iter()
        .map(|a| a.trim())
        .collect();
    let result = raw_result.trim();

    let mut letters: Vec<char> = input.chars().filter(|c| c.is_ascii_alphabetic()).collect();

    letters.sort();
    letters.dedup(); // Remove duplicates

    // Generate first letters to remove from possible perms if zero
    let mut firsts: Vec<char> = Vec::new();
    for addend in &addends {
        if let Some(f_char) = addend.chars().next() {
            firsts.push(f_char);
        }
    }

    firsts.push(result.chars().next().unwrap());

    // Generate all possible digit permutations
    let digit_chars = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut perms: Vec<String> = Vec::new();
    permutations(letters.len(), String::new(), digit_chars, &mut perms);

    // Remove all permutations with zeros for first letters
    perms.retain(|perm| {
        let zipped = perm.chars().zip(letters.iter().copied());
        for (n, l) in zipped {
            if n == '0' && firsts.contains(&l) {
                return false;
            }
        }
        true
    });

    // Brute force checking permutations
    for perm in perms {
        let digit_perm: Vec<u8> = perm
            .chars()
            .filter_map(|c| c.to_digit(10))
            .filter_map(|d| u8::try_from(d).ok())
            .collect();

        let zipped: Vec<(char, u8)> = letters.iter().copied().zip(digit_perm).collect();

        if alphametic_is_equal(&addends, result, &zipped) {
            let mut map: HashMap<char, u8> = HashMap::new();
            for (ch, n) in zipped {
                map.insert(ch, n);
            }

            return Some(map);
        }
    }

    None
}

fn permutations(n: usize, perm: String, list: Vec<char>, perms: &mut Vec<String>) {
    if perm.len() == n || list.is_empty() {
        perms.push(perm);
        return;
    }

    for c in &list {
        let mut cur_string: String = perm.clone();
        let cur_list: Vec<char> = list.iter().filter(|x| *x != c).cloned().collect();

        cur_string.push(*c);
        permutations(n, cur_string, cur_list, perms);
    }
}

pub fn alphametic_is_equal(addends: &[&str], result: &str, map: &[(char, u8)]) -> bool {
    let num_addends: Vec<u64> = addends
        .iter()
        .filter_map(|&a| {
            let mut temp = a.to_string();
            for (ch, val) in map {
                temp = temp.replace(*ch, &val.to_string());
            }
            temp.parse::<u64>().ok()
        })
        .collect();

    let mut temp = result.to_string();
    for (ch, val) in map {
        temp = temp.replace(*ch, &val.to_string());
    }

    let num_result: u64 = match temp.parse::<u64>() {
        Ok(x) => x,
        Err(_) => panic!("Invalid number"),
    };

    num_addends.iter().sum::<u64>() == num_result
}
