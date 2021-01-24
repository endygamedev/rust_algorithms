//! Cocktail shaker sort

/// cocktail_sort(vec) returns sorted `vec` by cocktail shaker method
pub fn cocktail_sort<T: Ord>(vec: &mut [T]) {
    let mut left = 0;
    let mut right = vec.len() - 1;
    let mut i;

    while left <= right {
        for i in left..right {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
            }
        }
        right -= 1;

        i = right;
        while i > left {
            if vec[i - 1] > vec[i] {
                vec.swap(i - 1, i);
            };
            i -= 1;
        }
        left += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::cocktail_sort;
    use crate::structures;
    use structures::random_list;

    #[test]
    fn basic_test() {
        let mut vec = vec![10, 2, 4, 5, 1, 2, 4, 5, 1, 2, 7];
        cocktail_sort(&mut vec);
        assert_eq!(vec, vec![1, 1, 2, 2, 2, 4, 4, 5, 5, 7, 10]);
    }

    #[test]
    fn str_test() {
        let mut vec = vec!["a", "c", "b"];
        cocktail_sort(&mut vec);
        assert_eq!(vec, vec!["a", "b", "c"]);
    }

    #[test]
    fn reverse_test() {
        let mut vec = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut clone = vec.clone();
        clone.sort();
        cocktail_sort(&mut vec);
        assert_eq!(vec, clone);
    }

    #[test]
    fn rand_test01() {
        let mut vec = random_list(10, -10, 10);
        let mut clone = vec.clone();
        clone.sort();
        cocktail_sort(&mut vec);
        assert_eq!(vec, clone);
    }

    #[test]
    fn rand_test02() {
        let mut vec = random_list(500, -500, 500);
        let mut clone = vec.clone();
        clone.sort();
        cocktail_sort(&mut vec);
        assert_eq!(vec, clone);
    }

    #[test]
    fn rand_test03() {
        let mut vec = random_list(1000, -1000, 1000);
        let mut clone = vec.clone();
        clone.sort();
        cocktail_sort(&mut vec);
        assert_eq!(vec, clone);
    }
}
