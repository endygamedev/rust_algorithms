//! Linear search


/// linear_search(list, value) returns the position of the `value` in the `list`
pub fn linear_search(list: Vec<isize>, value: isize) -> isize {
    // Checking if an _value_ is in the list
    // If it is not in the list, then return -1
    if !list.contains(&value) {
        return -1;
    }

    let mut i: usize = 0;
    while i < list.len() && list[i] != value {
        i += 1
    }
    return i as isize;
}


#[cfg(test)]
mod tests {
    use super::linear_search;

    #[test]
    fn test01() { assert_eq!(linear_search(vec![3, 1, 2], 2), 2); }

    #[test]
    fn test02() { assert_eq!(linear_search(vec![1, 2, -1, -2, -3], -2), 3); }

    #[test]
    fn test03() { assert_eq!(linear_search(vec![-1, -10, 0], 1), -1); }
}