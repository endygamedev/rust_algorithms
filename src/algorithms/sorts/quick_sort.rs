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
    }
}


#[cfg(test)]
mod tests {
    use crate::structures;
    use super::quick_sort;
    use structures::random_list;

    #[test]
    fn test01() {
        let mut vec: Vec<isize> = random_list(10, -10, 10);
        let sorted_vec: Vec<isize> = quick_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn test02() {
        let mut vec: Vec<isize> = random_list(500, -500, 500);
        let sorted_vec: Vec<isize> = quick_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn test03() {
        let mut vec: Vec<isize> = random_list(1000, -1000, 1000);
        let sorted_vec: Vec<isize> = quick_sort(vec.clone());
        vec.sort();
        assert_eq!(vec, sorted_vec);
    }
}
