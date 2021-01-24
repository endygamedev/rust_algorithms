//! Selection sort

/// selection_sort(list) returns sorted `list` by selection method
pub fn selection_sort(list: Vec<isize>) -> Vec<isize> {
    let mut vec: Vec<isize> = list.clone();
    let mut minimal: usize;
    for i in 0..vec.len() {
        minimal = i;
        for j in i + 1..vec.len() {
            if vec[j] < vec[minimal] {
                minimal = j
            }
        }
        vec.swap(minimal, i);
    }
    return vec;
}

#[cfg(test)]
mod tests {
    use super::selection_sort;
    use crate::structures;
    use structures::random_list;

    #[test]
    fn test01() {
        let mut vec: Vec<isize> = random_list(10, -10, 10);
        let sorted_vec: Vec<isize> = selection_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn test02() {
        let mut vec: Vec<isize> = random_list(500, -500, 500);
        let sorted_vec: Vec<isize> = selection_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn test03() {
        let mut vec: Vec<isize> = random_list(1000, -1000, 1000);
        let sorted_vec: Vec<isize> = selection_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }
}
