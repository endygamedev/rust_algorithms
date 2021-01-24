//! Binary search

/// _is_sorted(list) return the check if the `list` is sorted
fn _is_sorted(list: &Vec<isize>) -> bool {
    let mut original: Vec<isize> = list.clone();
    original.sort();
    return original == *list;
}

/// binary_search(list, value) returns the position of the `value` in the `list`
pub fn binary_search(list: Vec<isize>, value: isize) -> isize {
    // Checking if `list` is sorted because this algorithm works only with sorted lists
    assert!(_is_sorted(&list), "List isn't sorted!");
    // Checking if `value` is in the `list`
    // If it is not in the list, then return -1
    if !list.contains(&value) {
        return -1;
    }

    let mut left: usize = 0;
    let mut right: usize = list.len() - 1;
    let mut mid: usize = 0;

    while left <= right {
        mid = (left + right) / 2;
        if list[mid] < value {
            left = mid + 1;
        } else if list[mid] > value {
            right = mid - 1;
        } else {
            break;
        }
    }

    return mid as isize;
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn test01() {
        assert_eq!(binary_search(vec![1, 1, 2, 3, 4, 5], 2), 2);
    }

    #[test]
    fn test02() {
        assert_eq!(binary_search(vec![1, 1, 2, 10, 100, 2000], 10), 3);
    }

    #[test]
    fn test03() {
        assert_eq!(binary_search(vec![-10, 10, 20], -10), 0);
    }

    #[test]
    fn test04() {
        assert_eq!(binary_search(vec![1, 2, 3], 4), -1);
    }
}
