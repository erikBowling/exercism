use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (raw_addends, raw_result) = input.split_once(" == ").unwrap();
    let addends: Vec<&str> = raw_addends.split("+").map(|a| a.trim()).collect();
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
    let digits: Vec<u8> = (0..=9).collect();
    let mut perm: Vec<u8> = Vec::new();

    if backtrack(
        letters.len(),
        &mut perm,
        digits,
        &addends,
        result,
        &letters,
        &firsts,
    ) {
        let zip_map: Vec<(char, u8)> = letters.iter().copied().zip(perm.iter().copied()).collect();
        let mut map: HashMap<char, u8> = HashMap::new();
        for (ch, n) in zip_map {
            map.insert(ch, n);
        }

        return Some(map);
    }

    None
}

pub fn alphametic_is_equal(addends: &[&str], result: &str, perm: &[u8], letters: &[char]) -> bool {
    let map: Vec<(char, u8)> = letters.iter().copied().zip(perm.iter().copied()).collect();
    let num_addends: Vec<u64> = addends
        .iter()
        .filter_map(|&a| {
            let mut temp = a.to_string();
            for (ch, val) in &map {
                temp = temp.replace(*ch, &val.to_string());
            }
            temp.parse::<u64>().ok()
        })
        .collect();

    let mut temp = result.to_string();
    for (ch, val) in map {
        temp = temp.replace(ch, &val.to_string());
    }

    let num_result: u64 = match temp.parse::<u64>() {
        Ok(x) => x,
        Err(_) => panic!("Invalid number"),
    };

    num_addends.iter().sum::<u64>() == num_result
}

pub fn backtrack(
    n: usize,
    perm: &mut Vec<u8>,
    list_digits: Vec<u8>,
    addends: &[&str],
    result: &str,
    letters: &[char],
    firsts: &[char],
) -> bool {
    if perm.len() == n {
        for (ch, n) in letters.iter().copied().zip(perm.iter().copied()) {
            if n == 0 && firsts.contains(&ch) {
                return false;
            }
        }

        if alphametic_is_equal(addends, result, perm, letters) {
            return true;
        }

        return false;
    }

    for digit in &list_digits {
        perm.push(*digit);
        let valid_digits: Vec<u8> = list_digits
            .iter()
            .copied()
            .filter(|d| !perm.contains(d))
            .collect();
        if backtrack(n, perm, valid_digits, addends, result, letters, firsts) {
            return true;
        }
        perm.pop();
    }

    false
}

// fn permutations(n: usize, perm: String, list: Vec<char>, perms: &mut Vec<String>) {
//     if perm.len() == n || list.is_empty() {
//         perms.push(perm);
//         return;
//     }

//     for c in &list {
//         let mut cur_string: String = perm.clone();
//         let cur_list: Vec<char> = list.iter().filter(|x| *x != c).cloned().collect();

//         cur_string.push(*c);
//         permutations(n, cur_string, cur_list, perms);
//     }
// }
