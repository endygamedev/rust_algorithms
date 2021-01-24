//! Merge sort


/// merge(left_vec, right_vec, res) joins parts of a list in ascending order
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

/// merge_sort(list) returns sorted `list` by merge method
pub fn merge_sort(list: Vec<isize>) -> Vec<isize> {
    let vec: Vec<isize> = list;
    let mid = vec.len() / 2;
    if mid == 0 { return vec; }

    let left_vec: Vec<isize> = merge_sort(vec[..mid].to_vec());
    let right_vec: Vec<isize> = merge_sort(vec[mid..].to_vec());

    let res: Vec<isize> = merge(left_vec, right_vec, vec);

    return res;
}


#[cfg(test)]
mod tests {
    use crate::structures;
    use super::merge_sort;
    use structures::random_list;

    #[test]
    fn test01() {
        let mut vec: Vec<isize> = random_list(10, -10, 10);
        let sorted_vec: Vec<isize> = merge_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn test02() {
        let mut vec: Vec<isize> = random_list(500, -500, 500);
        let sorted_vec: Vec<isize> = merge_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn test03() {
        let mut vec: Vec<isize> = random_list(1000, -1000, 1000);
        let sorted_vec: Vec<isize> = merge_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }
}