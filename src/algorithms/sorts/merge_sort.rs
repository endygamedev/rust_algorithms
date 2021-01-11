/// Joins parts of a list in ascending order
fn merge(left_vec: Vec<isize>, right_vec: Vec<isize>, mut res: Vec<isize>) -> Vec<isize> {
    let mut left = 0;
    let mut right = 0;
    let mut index = 0;

    while left < left_vec.len() && right < right_vec.len() {
        if left_vec[left] <= right_vec[right] {
            res[index] = left_vec[left];
            left += 1;
        } else {
            res[index] = right_vec[right];
            right += 1;
        }
        index += 1;
    }

    if left < left_vec.len() {
        res[index..].copy_from_slice(&left_vec[left..]);
    }

    if right < right_vec.len() {
        res[index..].copy_from_slice(&right_vec[right..]);
    }
    return res;
}

/// Merge sort
pub fn merge_sort(list: Vec<isize>) -> Vec<isize> {
    let mut vec: Vec<isize> = list;
    let mid = vec.len() / 2;
    if mid == 0 {
        return vec;
    }

    let left_vec: Vec<isize> = merge_sort(vec[..mid].to_vec());
    let right_vec: Vec<isize> = merge_sort(vec[mid..].to_vec());

    let res: Vec<isize> = merge(left_vec, right_vec, vec);

    return res;
}
