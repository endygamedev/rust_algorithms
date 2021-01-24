//! Cocktail shaker sort

/// cocktail_sort(list) returns sorted `list` by cocktail shaker method
pub fn cocktail_sort(list: Vec<isize>) -> Vec<isize> {
    let mut vec: Vec<isize> = list.clone();
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
    return vec;
}

#[cfg(test)]
mod tests {
    use super::cocktail_sort;
    use crate::structures;
    use structures::random_list;

    #[test]
    fn test01() {
        let mut vec: Vec<isize> = random_list(10, -10, 10);
        let sorted_vec: Vec<isize> = cocktail_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn test02() {
        let mut vec: Vec<isize> = random_list(500, -500, 500);
        let sorted_vec: Vec<isize> = cocktail_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn test03() {
        let mut vec: Vec<isize> = random_list(1000, -1000, 1000);
        let sorted_vec: Vec<isize> = cocktail_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }
}
