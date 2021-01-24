//! Shell sort

/// shell_sort(list) returns sorted `list` by shell method
pub fn shell_sort(list: Vec<isize>) -> Vec<isize> {
    let mut vec: Vec<isize> = list.clone();
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
    return vec;
}

#[cfg(test)]
mod tests {
    use super::shell_sort;
    use crate::structures;
    use structures::random_list;

    #[test]
    fn test01() {
        let mut vec: Vec<isize> = random_list(10, -10, 10);
        let sorted_vec: Vec<isize> = shell_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn test02() {
        let mut vec: Vec<isize> = random_list(500, -500, 500);
        let sorted_vec: Vec<isize> = shell_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn test03() {
        let mut vec: Vec<isize> = random_list(1000, -1000, 1000);
        let sorted_vec: Vec<isize> = shell_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }
}
