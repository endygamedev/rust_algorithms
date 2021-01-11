/// Quicksort
pub fn quick_sort(list: Vec<isize>) -> Vec<isize> {
    let mut vec: Vec<isize> = list.clone();

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
