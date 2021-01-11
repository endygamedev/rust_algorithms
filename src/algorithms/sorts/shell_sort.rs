/// Shell sort
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
