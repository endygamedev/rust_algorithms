//! Shell sort

/// shell_sort(vec) returns sorted `vec` by shell method
pub fn shell_sort<T: Ord>(vec: &mut [T]) {
    let mut step: usize = vec.len() / 2;
    let mut j: usize;

    while step > 0 {
        for i in step..vec.len() {
            j = i;
            while j >= step && vec[j - step] > vec[j] {
                vec.swap(j - step, j);
                j -= step;
            }
        }
        step /= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::shell_sort;
    use crate::structures;
    use structures::random_list;

    #[test]
    fn basic_test() {
        let mut vec = vec![10, 2, 4, 5, 1, 2, 4, 5, 1, 2, 7];
        shell_sort(&mut vec);
        assert_eq!(vec, vec![1, 1, 2, 2, 2, 4, 4, 5, 5, 7, 10]);
    }

    #[test]
    fn str_test() {
        let mut vec = vec!["a", "c", "b"];
        shell_sort(&mut vec);
        assert_eq!(vec, vec!["a", "b", "c"]);
    }

    #[test]
    fn reverse_test() {
        let mut vec = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut clone = vec.clone();
        clone.sort();
        shell_sort(&mut vec);
        assert_eq!(vec, clone);
    }

    #[test]
    fn rand_test01() {
        let mut vec = random_list(10, -10, 10);
        let mut clone = vec.clone();
        clone.sort();
        shell_sort(&mut vec);
        assert_eq!(vec, clone);
    }

    #[test]
    fn rand_test02() {
        let mut vec = random_list(500, -500, 500);
        let mut clone = vec.clone();
        clone.sort();
        shell_sort(&mut vec);
        assert_eq!(vec, clone);
    }

    #[test]
    fn rand_test03() {
        let mut vec = random_list(1000, -1000, 1000);
        let mut clone = vec.clone();
        clone.sort();
        shell_sort(&mut vec);
        assert_eq!(vec, clone);
    }
}
