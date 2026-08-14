#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    // Case 1: lists are equal
    if first_list == second_list {
        return Comparison::Equal;
    }

    // Case 2: List A contains list B
    if second_list.len() == 0
        || first_list
            .windows(second_list.len())
            .any(|x| x == second_list)
    {
        return Comparison::Superlist;
    }

    // Case 3: List B contains list A
    if first_list.len() == 0
        || second_list
            .windows(first_list.len())
            .any(|x| x == first_list)
    {
        return Comparison::Sublist;
    }

    // Case 4: If no other case is true, then they are unequal
    Comparison::Unequal
}
