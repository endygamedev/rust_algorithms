//! Quicksort

/// quick_sort(list) returns sorted `list` by quicksort method
pub fn quick_sort(list: Vec<isize>) -> Vec<isize> {
    let vec: Vec<isize> = list.clone();

    let mut less: Vec<isize> = Vec::new();
    let mut greater: Vec<isize> = Vec::new();
    let mut equal: Vec<isize> = Vec::new();

    return if vec.len() > 0 {
        let pivot = vec[0];

        for element in vec {
            if element < pivot {
                less.push(element);
            } else if element == pivot {
                equal.push(element);
            } else {
                greater.push(element);
            }
        }

        [quick_sort(less), equal, quick_sort(greater)].concat()
    } else {
        vec![]
    };
}

#[cfg(test)]
mod tests {
    use super::quick_sort;
    use crate::structures;
    use structures::random_list;

    #[test]
    fn basic_test() {
        let vec = vec![10, 2, 4, 5, 1, 2, 4, 5, 1, 2, 7];
        assert_eq!(quick_sort(vec), vec![1, 1, 2, 2, 2, 4, 4, 5, 5, 7, 10]);
    }

    #[test]
    fn reverse_test() {
        let vec = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut clone = vec.clone();
        clone.sort();
        assert_eq!(quick_sort(vec), clone);
    }

    #[test]
    fn rand_test01() {
        let vec = random_list(10, -10, 10);
        let mut clone = vec.clone();
        clone.sort();
        assert_eq!(quick_sort(vec), clone);
    }

    #[test]
    fn rand_test02() {
        let vec = random_list(500, -500, 500);
        let mut clone = vec.clone();
        clone.sort();
        assert_eq!(quick_sort(vec), clone);
    }

    #[test]
    fn rand_test03() {
        let vec = random_list(1000, -1000, 1000);
        let mut clone = vec.clone();
        clone.sort();
        assert_eq!(quick_sort(vec), clone);
    }
}
