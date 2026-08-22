pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    binary_search(0, array.len(), array, key)
}

pub fn binary_search(start: usize, stop: usize, array: &[i32], key: i32) -> Option<usize> {
    if start >= stop {
        match start < array.len() && key == array[start] {
            true => return Some(start),
            false => return None,
        }
    }

    let midpoint = (start + stop) / 2;
    if key == array[midpoint] {
        return Some(midpoint);
    }

    if key < array[midpoint] {
        return binary_search(start, midpoint - 1, array, key);
    }

    binary_search(midpoint + 1, stop, array, key)
}
