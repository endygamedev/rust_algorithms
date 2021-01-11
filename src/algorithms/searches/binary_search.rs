/// Binary search
/// TODO: Check if the list is sorted
pub fn binary_search(list: Vec<isize>, value: isize) -> isize {
    /// Checking if an _value_ is in the list
    /// If it is not in the list, then return -1
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
