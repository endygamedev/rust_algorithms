//! Bubble sort

/// bubble_sort(list) returns sorted `list` by bubble method
pub fn bubble_sort(list: Vec<isize>) -> Vec<isize> {
    let mut vec: Vec<isize> = list.clone();
    for _ in 0..vec.len() {
        for j in 0..vec.len() - 1 {
            if vec[j + 1] < vec[j] {
                vec.swap(j + 1, j)
            }
        }
    }
    return vec;
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;
    use crate::structures;
    use structures::random_list;

    #[test]
    fn test01() {
        let mut vec: Vec<isize> = random_list(10, -10, 10);
        let sorted_vec: Vec<isize> = bubble_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn test02() {
        let mut vec: Vec<isize> = random_list(500, -500, 500);
        let sorted_vec: Vec<isize> = bubble_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn test03() {
        let mut vec: Vec<isize> = random_list(1000, -1000, 1000);
        let sorted_vec: Vec<isize> = bubble_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }
}
