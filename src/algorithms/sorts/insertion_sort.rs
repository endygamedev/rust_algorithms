//! Insertion sort

/// insertion_sort(vec) returns sorted `vec` by insertion method
pub fn insertion_sort<T: Ord>(vec: &mut [T]) {
    let mut j: usize;
    for i in 0..vec.len() {
        j = i;
        while j > 0 && vec[j - 1] > vec[j] {
            vec.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;
    use crate::structures;
    use structures::random_list;

    #[test]
    fn basic_test() {
        let mut vec = vec![10, 2, 4, 5, 1, 2, 4, 5, 1, 2, 7];
        insertion_sort(&mut vec);
        assert_eq!(vec, vec![1, 1, 2, 2, 2, 4, 4, 5, 5, 7, 10]);
    }

    #[test]
    fn str_test() {
        let mut vec = vec!["a", "c", "b"];
        insertion_sort(&mut vec);
        assert_eq!(vec, vec!["a", "b", "c"]);
    }

    #[test]
    fn reverse_test() {
        let mut vec = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut clone = vec.clone();
        clone.sort();
        insertion_sort(&mut vec);
        assert_eq!(vec, clone);
    }

    #[test]
    fn rand_test01() {
        let mut vec = random_list(10, -10, 10);
        let mut clone = vec.clone();
        clone.sort();
        insertion_sort(&mut vec);
        assert_eq!(vec, clone);
    }

    #[test]
    fn rand_test02() {
        let mut vec = random_list(500, -500, 500);
        let mut clone = vec.clone();
        clone.sort();
        insertion_sort(&mut vec);
        assert_eq!(vec, clone);
    }

    #[test]
    fn rand_test03() {
        let mut vec = random_list(1000, -1000, 1000);
        let mut clone = vec.clone();
        clone.sort();
        insertion_sort(&mut vec);
        assert_eq!(vec, clone);
    }
}
