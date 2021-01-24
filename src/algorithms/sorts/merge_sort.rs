//! Merge sort

/// _merge(left_vec, right_vec, res) joins parts of a list in ascending order
fn _merge(left_vec: Vec<isize>, right_vec: Vec<isize>, mut res: Vec<isize>) -> Vec<isize> {
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
    if mid == 0 {
        return vec;
    }

    let left_vec: Vec<isize> = merge_sort(vec[..mid].to_vec());
    let right_vec: Vec<isize> = merge_sort(vec[mid..].to_vec());

    let res: Vec<isize> = _merge(left_vec, right_vec, vec);

    return res;
}

#[cfg(test)]
mod tests {
    use super::merge_sort;
    use crate::structures;
    use structures::random_list;

    #[test]
    fn basic_test() {
        let vec = vec![10, 2, 4, 5, 1, 2, 4, 5, 1, 2, 7];
        assert_eq!(merge_sort(vec), vec![1, 1, 2, 2, 2, 4, 4, 5, 5, 7, 10]);
    }

    #[test]
    fn reverse_test() {
        let vec = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut clone = vec.clone();
        clone.sort();
        assert_eq!(merge_sort(vec), clone);
    }

    #[test]
    fn rand_test01() {
        let vec = random_list(10, -10, 10);
        let mut clone = vec.clone();
        clone.sort();
        assert_eq!(merge_sort(vec), clone);
    }

    #[test]
    fn rand_test02() {
        let vec = random_list(500, -500, 500);
        let mut clone = vec.clone();
        clone.sort();
        assert_eq!(merge_sort(vec), clone);
    }

    #[test]
    fn rand_test03() {
        let vec = random_list(1000, -1000, 1000);
        let mut clone = vec.clone();
        clone.sort();
        assert_eq!(merge_sort(vec), clone);
    }
}
