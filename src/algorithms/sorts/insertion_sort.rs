//! Insertion sort


/// insertion_sort(list) returns sorted `list` by insertion method
pub fn insertion_sort(list: Vec<isize>) -> Vec<isize> {
    let mut vec: Vec<isize> = list.clone();
    let mut j: usize;
    for i in 0..vec.len() {
        j = i;
        while j > 0 && vec[j - 1] > vec[j] {
            vec.swap(j - 1, j);
            j -= 1;
        }
    }
    return vec;
}


#[cfg(test)]
mod tests {
    use crate::structures;
    use super::insertion_sort;
    use structures::random_list;

    #[test]
    fn test01() {
        let mut vec: Vec<isize> = random_list(10, -10, 10);
        let sorted_vec: Vec<isize> = insertion_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn test02() {
        let mut vec: Vec<isize> = random_list(500, -500, 500);
        let sorted_vec: Vec<isize> = insertion_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn test03() {
        let mut vec: Vec<isize> = random_list(1000, -1000, 1000);
        let sorted_vec: Vec<isize> = insertion_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }
}
