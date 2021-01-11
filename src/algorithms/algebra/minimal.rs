/// Returns the minimal value in an array
pub fn minimal(list: Vec<isize>) -> isize {
    let mut res: isize = list[0];
    for element in list {
        if element < res {
            res = element
        }
    }
    return res;
}
