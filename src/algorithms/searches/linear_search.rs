/// Linear search
pub fn linear_search(list: Vec<isize>, value: isize) -> isize {
    /// Checking if an _value_ is in the list
    /// If it is not in the list, then return -1
    if !list.contains(&value) {
        return -1;
    }

    let mut i: usize = 0;
    while i < list.len() && list[i] != value {
        i += 1
    }
    return i as isize;
}
