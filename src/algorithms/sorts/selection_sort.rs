/// Selection sort
pub fn selection_sort(list: Vec<isize>) -> Vec<isize> {
    let mut vec: Vec<isize> = list.clone();
    let mut minimal: usize;
    for i in 0..vec.len() {
        minimal = i;
        for j in i + 1..vec.len() {
            if vec[j] < vec[minimal] {
                minimal = j
            }
        }
        vec.swap(minimal, i);
    }
    return vec;
}
